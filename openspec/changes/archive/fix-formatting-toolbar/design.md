# Design: Fix Formatting Toolbar Cursor Position

## Problem Analysis

### Current Implementation

In `src/ui/editor.rs` (lines 12-81):

```rust
let output = ui.add_sized(
    [ui.available_width(), ui.available_height().max(600.0)],
    egui::TextEdit::multiline(body)
        .lock_focus(true),
        // ... layouter config
);
```

**The bug**: `ui.add_sized()` consumes the `TextEditOutput` and returns only a `Response`. The code then tries to recover cursor state via:

```rust
if let Some(state) = egui::TextEdit::load_state(ui.ctx(), output.id) {
    if let Some(range) = state.cursor.char_range() {
        *selection = Some((range.primary.index, range.secondary.index));
    }
}
```

The `output.id` from `add_sized` does NOT match the internal state ID that `TextEdit` uses for `load_state`. So `load_state` returns `None` or stale state, causing `selection` to default to `(body.len(), body.len())` — the end of the document.

### The Flow

```
User clicks Bold button
  → toolbar::show_toolbar returns actions.insert_bold = true
  → app.rs calls self.apply_format("**", "**")
  → apply_format calls insert_replacement(prefix, suffix)
  → insert_replacement reads self.selection
  → self.selection is ALWAYS (body.len(), body.len()) because load_state fails
  → Formatting wraps wrong text (or nothing) at end of document
```

### In `insert_replacement` (app.rs line 589)

```rust
let (start_char, end_char) = self.selection.unwrap_or((self.body.chars().count(), self.body.chars().count()));
```

When `selection` is `None` or `(body.len(), body.len())`, the fallback is end-of-document. This is why formatting always seems to start at the end.

---

## Proposed Fix

### Change 1: Use `.show(ui)` instead of `ui.add_sized()`

Replace `ui.add_sized()` in `editor.rs` with direct `.show()` call that captures `TextEditOutput`:

```rust
// BEFORE (broken):
let output = ui.add_sized([width, height], egui::TextEdit::multiline(body)...);

// AFTER (fixed):
let mut text_edit = egui::TextEdit::multiline(body)
    .desired_width(f32::INFINITY)
    .lock_focus(true)
    .layouter(...);

let output = text_edit.show(ui);
```

The `.show(ui)` method returns `TextEditOutput` which contains:
- `.response` — the Response
- `.state` — the `TextEditState` with accurate cursor info
- `.galley` — the rendered text layout

### Change 2: Extract cursor from `output.state` directly

```rust
// AFTER (fixed):
if let Some(cursor_range) = output.state.cursor.char_range() {
    *selection = Some((cursor_range.primary.index, cursor_range.secondary.index));
}
```

No more `load_state` with mismatched IDs.

### Change 3: Handle pending_selection the same way

The `pending_selection` logic (lines 84-93) already uses `output.id` correctly via `TextEditState::load`. With `.show()`, `output.state` gives us the actual state directly — but the `load/store` pattern should still work.

---

## Implementation Steps

### Step 1: Modify `src/ui/editor.rs`

Change `show_editor` function to use `.show(ui)`:

```rust
pub fn show_editor(...) -> bool {
    let mut text_edit = egui::TextEdit::multiline(body)
        .font(egui::TextStyle::Monospace)
        .desired_width(f32::INFINITY)
        .lock_focus(true)
        .layouter(&mut |ui, text, wrap_width| { /* same layouter */ });

    let output = text_edit.show(ui);

    // Apply pending_selection if any
    if let Some((start, end)) = pending_selection.take() {
        let mut state = output.state.clone();
        let c_start = egui::text::CCursor::new(start);
        let c_end = egui::text::CCursor::new(end);
        state.cursor.set_char_range(Some(egui::text::CCursorRange::two(c_start, c_end)));
        state.store(ui.ctx(), output.id);
    }

    // Sync selection from output.state directly
    if let Some(cursor_range) = output.state.cursor.char_range() {
        *selection = Some((cursor_range.primary.index, cursor_range.secondary.index));
    }

    output.response.changed()
}
```

**Note**: Need to check that `.show()` returns the same `id` that `pending_selection` logic expects for `store()`. If not, may need to use `output.state.id` or keep the `load/store` approach with the correct ID.

### Step 2: Test cursor behavior

After the change:
1. Place cursor in middle of text → click Bold → `****` should appear at cursor with cursor between them
2. Select a word → click Bold → word should be wrapped as `**word**`
3. Cursor at end of document → click Italic → `**` inserted at end with cursor between

### Step 3: Verify layout

The `ui.add_sized([width, height], ...)` enforced a minimum size. With `.show(ui)`, the `TextEdit` will size itself based on available space. May need to wrap in `ui.allocate_ui` or set `.desired_rows()` to maintain the 600px minimum height.

---

## Verification Plan

| Scenario | Expected | Test |
|----------|----------|------|
| Cursor at middle, no selection | Tokens at cursor position | Place cursor, click Bold, verify `****` at correct spot |
| Text selected | Selection wrapped | Select "hello", click Bold, verify `**hello**` |
| Cursor at EOF | Tokens at end | Move to end, click Italic, verify `*` at end |
| After format, keep selection state | No cursor jump | Format, type, verify text appears at cursor not end |

---

## Risk Assessment

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| Layout breaks without `add_sized` size constraint | Low | Use `.desired_rows()` or wrap in `ui.allocate_ui` to enforce min height |
| `output.id` mismatch for pending_selection | Low | Verify ID matches expected format; use `output.state.id` if needed |
| Cursor still not updating | Medium | Debug print `selection` value after sync to verify correct values |

---

## Files to Modify

- `src/ui/editor.rs` — main change, replace `add_sized` with `.show()`
- `src/app.rs` — no changes needed, `apply_format`/`insert_replacement` already read `self.selection` which will now be accurate