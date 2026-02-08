//! Servicio de sincronización con Git/GitHub.

use anyhow::Context;
use std::path::PathBuf;

/// Resultado de la operación de sincronización.
pub type SyncResult = anyhow::Result<String>;

/// Ejecuta la sincronización completa: commit, fetch, merge, push.
pub fn sync_to_github(
    repo_path: PathBuf,
    token: String,
    commit_msg: String,
    content_dir: String,
    assets_dir: String,
) -> SyncResult {
    let repo = match git2::Repository::open(&repo_path) {
        Ok(r) => r,
        Err(e) => {
            if e.code() == git2::ErrorCode::Owner {
                return Err(anyhow::anyhow!(
                    "Error de seguridad: el repositorio no es de tu propiedad.\n\
                    Ejecuta este comando en tu terminal:\n\
                    git config --global --add safe.directory {}", 
                    repo_path.display()
                ));
            }
            return Err(e.into());
        }
    };
    
    // Stage changes
    let mut index = repo.index()?;
    index.add_all([&content_dir, &assets_dir].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    
    // Commit
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let sig = repo.signature()?;
    let parent_commit = repo.head()?.peel_to_commit()?;
    repo.commit(Some("HEAD"), &sig, &sig, &commit_msg, &tree, &[&parent_commit])?;
    
    // Identify branch
    let head = repo.head()?;
    let branch = head.shorthand().unwrap_or("main");

    // 1. Fetch
    let mut remote = repo.find_remote("origin")?;
    let remote_url = remote.url().unwrap_or("desconocido").to_string();
    
    let mut fetch_options = git2::FetchOptions::new();
    let mut callbacks = git2::RemoteCallbacks::new();
    let token_for_fetch = token.clone();
    callbacks.credentials(move |_url, username_from_url, _allowed_types| {
        let user = username_from_url.unwrap_or("x-access-token");
        git2::Cred::userpass_plaintext(user, &token_for_fetch)
    });
    fetch_options.remote_callbacks(callbacks);
    
    // Intentamos hacer fetch
    if let Err(e) = remote.fetch(&[branch], Some(&mut fetch_options), None) {
        if e.to_string().contains("403") {
            return Err(anyhow::anyhow!(
                "Error 403 (Fetch): Acceso denegado a {}.\n\
                Verifica que tu token tenga permisos de lectura y acceso al repositorio.", 
                remote_url
            ));
        }
        // Ignoramos otros errores de fetch
    }

    // 2. Merge (Pull)
    if let Ok(fetch_head) = repo.find_reference("FETCH_HEAD") {
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let (analysis, _) = repo.merge_analysis(&[&fetch_commit])?;

        if analysis.is_fast_forward() {
            let refname = format!("refs/heads/{}", branch);
            if let Ok(mut reference) = repo.find_reference(&refname) {
                reference.set_target(fetch_commit.id(), "Fast-forward")?;
                repo.set_head(&refname)?;
                repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            }
        } else if analysis.is_normal() {
            repo.merge(&[&fetch_commit], None, None)?;
            if repo.index()?.has_conflicts() {
                repo.cleanup_state()?;
                return Err(anyhow::anyhow!(
                    "Conflictos de mezcla detectados. Por favor, resuélvelos manualmente en tu terminal."
                ));
            }
            let sig = repo.signature()?;
            let tree_id = repo.index()?.write_tree()?;
            let tree = repo.find_tree(tree_id)?;
            let head_commit = repo.head()?.peel_to_commit()?;
            let merge_commit = repo.find_commit(fetch_commit.id())?;
            repo.commit(
                Some("HEAD"), 
                &sig, 
                &sig, 
                "Merge remote-tracking branch", 
                &tree, 
                &[&head_commit, &merge_commit]
            )?;
            repo.cleanup_state()?;
        }
    }

    // 3. Push
    let mut callbacks = git2::RemoteCallbacks::new();
    let token_for_push = token.clone();
    callbacks.credentials(move |_url, username_from_url, _allowed_types| {
        let user = username_from_url.unwrap_or("x-access-token");
        git2::Cred::userpass_plaintext(user, &token_for_push)
    });
    
    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(callbacks);
    
    let ref_spec = format!("refs/heads/{}:refs/heads/{}", branch, branch);
    if let Err(e) = remote.push(&[&ref_spec], Some(&mut push_options)) {
        let err_msg = e.to_string();
        if err_msg.contains("403") {
            return Err(anyhow::anyhow!(
                "Error 403: Acceso denegado al subir a {}.\n\
                Posibles soluciones:\n\
                1. Verifica que tu Token tenga el permiso 'repo' habilitado.\n\
                2. Si usas un Fine-grained token, asegúrate de que el repositorio '{}' esté incluido en su acceso.\n\
                3. Verifica si tu organización requiere autorizar el token para SSO.\n\
                4. Asegúrate de que tu usuario tenga permisos de escritura en el repositorio.",
                remote_url, remote_url
            ));
        } else if err_msg.contains("401") {
            return Err(anyhow::anyhow!(
                "Error 401: No autorizado. Verifica que tu Token de GitHub sea correcto y no haya expirado."
            ));
        }
        return Err(e.into());
    }
    
    Ok("✅ Sincronizado correctamente".to_string())
}
