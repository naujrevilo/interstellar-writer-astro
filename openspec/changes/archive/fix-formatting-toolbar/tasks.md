# Tasks: Fix Formatting Toolbar Cursor Position

## Overview

Break down the fix into atomic implementation steps.

---

## Task 1: Replace `ui.add_sized()` with `.show(ui)`

**File**: `src/ui/editor.rs`
**Lines**: ~70 (lines 12-81)
**Risk**: Low

### Steps
1. Change from:
   ```rust
   let output = ui.add_sized(
       [ui.available_width(), ui.available_height().max(600.0)],
       egui::TextEdit::multiline(body)...
   );
   ```
   To:
   ```rust
   let mut text_edit = egui::TextEdit::multiline(body)
       .desired_width(f32::INFINITY)
       .lock_focus(true)
       .layouter(...);
   let output = text_edit.show(ui);
   ```

2. Ensure the TextEdit still gets proper sizing — either via `.desired_rows()` or wrapping in `ui.allocate_ui`.

### Completion Criterion
- Editor renders without layout collapse
- `output` is `TextEditOutput` (not `Response`)

---

## Task 2: Extract cursor from `output.state` directly

**File**: `src/ui/editor.rs`
**Lines**: ~15 (lines 95-101)
**Risk**: Low

### Steps
1. Replace the `load_state` approach:
   ```rust
   // BEFORE:
   if let Some(state) = egui::TextEdit::load_state(ui.ctx(), output.id) {
       if let Some(range) = state.cursor.char_range() {
           *selection = Some((range.primary.index, range.secondary.index));
       }
   }

   // AFTER:
   if let Some(cursor_range) = output.state.cursor.char_range() {
       *selection = Some((cursor_range.primary.index, cursor_range.secondary.index));
   }
   ```

### Completion Criterion
- `selection` updates correctly when cursor moves
- No more falling back to end-of-document

---

## Task 3: Verify `pending_selection` still works

**File**: `src/ui/editor.rs`
**Lines**: ~12 (lines 84-93)
**Risk**: Low

### Steps
1. The `pending_selection.take()` block already uses `output.id` with `TextEditState::load/store`.
2. With `.show()`, `output.id` should still be valid.
3. If not, use `output.state.id` instead.

### Completion Criterion
- Programmatic selection (like from dashboard click) still applies correctly

---

## Task 4: Ensure minimum height (layout regression check)

**File**: `src/ui/editor.rs`
**Risk**: Low

### Steps
1. The original `ui.add_sized([..., ui.available_height().max(600.0)])` enforced 600px minimum.
2. With `.show()`, add `.desired_rows(N)` to maintain same constraint, or wrap in `ui.allocate_ui`.

### Completion Criterion
- Editor has same visual height as before (600px minimum on first render)

---

## Task 5: Manual verification

**Scope**: No automated tests for UI cursor behavior in this codebase.

### Verification Checklist
- [ ] Place cursor mid-text → Bold → `****` appears at cursor, cursor between them
- [ ] Select a word → Bold → word wrapped as `**word**`
- [ ] Cursor at EOF → Italic → `*` inserted at end, cursor between
- [ ] Select multi-line → Bold → all lines wrapped
- [ ] No visual regression (editor height maintained)

---

## Summary

| Task | File | Est. Lines | Risk |
|------|------|------------|------|
| 1. Replace add_sized with show | editor.rs | ~70 | Low |
| 2. Extract cursor from output.state | editor.rs | ~15 | Low |
| 3. Verify pending_selection | editor.rs | ~12 | Low |
| 4. Ensure min height | editor.rs | ~10 | Low |
| 5. Manual verification | — | — | — |

**Total estimated changes**: ~107 lines in 1 file
**Review budget risk**: ✅ Low (under 400 lines)

---

## Next Step

Proceed to `/sdd-apply` to implement these tasks.