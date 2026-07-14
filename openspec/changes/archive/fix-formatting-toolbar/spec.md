# Editor Specification

## Purpose

Defines the text editor capabilities, specifically around cursor state tracking and formatting toolbar actions.

## ADDED Requirements

### Requirement: Toolbar Formatting with No Selection

The system MUST insert formatting tokens at the exact current cursor position when no text is selected.

#### Scenario: Cursor in the middle of a sentence
- GIVEN the cursor is placed at index 10 within a sentence
- AND no text is selected
- WHEN the user clicks the "Bold" toolbar button
- THEN `****` MUST be inserted starting at index 10
- AND the cursor MUST be repositioned to index 12 (between the asterisks)

#### Scenario: Cursor at the end of the document
- GIVEN the cursor is at the end of the document
- WHEN the user clicks the "Italic" toolbar button
- THEN `**` MUST be inserted at the end of the document
- AND the cursor MUST be repositioned inside the asterisks

### Requirement: Toolbar Formatting with Selected Text

The system MUST wrap the currently active text selection with the corresponding formatting tokens.

#### Scenario: Single word selected
- GIVEN the text "hello" is selected from index 5 to 10
- WHEN the user clicks the "Bold" toolbar button
- THEN `**` MUST be inserted at index 5
- AND `**` MUST be inserted at index 12 (original 10 + 2 for the first tokens)
- AND the resulting text MUST be `**hello**`
- AND the selection MUST be cleared and the cursor placed after the formatted text, OR the selection MUST encompass the newly formatted text.

#### Scenario: Multi-line selection
- GIVEN a multi-line text block is selected
- WHEN the user clicks a formatting button (e.g. Bold)
- THEN the entire multi-line block MUST be wrapped in the formatting tokens

### Requirement: Accurate Selection State Synchronization

The system MUST reliably extract the cursor and selection range from the underlying UI widget output, rather than falling back to document bounds.

#### Scenario: Extracting state from TextEdit
- GIVEN the `TextEdit` widget is rendered
- WHEN the frame completes
- THEN the cursor range MUST be extracted directly from `TextEditOutput` (or its immediate state)
- AND the internal editor state `selection` MUST accurately reflect this range without loss of precision.

## Acceptance Criteria

1. Clicking bold/italic with no text selected inserts the markdown tokens exactly at the cursor and places the cursor between them.
2. Clicking bold/italic with text selected wraps the exact selection in markdown tokens.
3. The cursor never unexpectedly falls back to the end of the document when a formatting action is triggered.
4. The editor's layout dimensions (width/height/rows) remain correct and do not collapse after switching rendering methods from `ui.add_sized` to `.show(ui)`.