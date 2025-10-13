# DEP_REVIEW.md - toy1_markdown_render

## Dependencies

### egui_commonmark 0.18.0
**Purpose**: Markdown rendering in egui
**API**: `CommonMarkViewer::new().show(ui, &mut cache, markdown_text)`

**Key findings**:
- Read-only display widget (struct has private fields)
- No native selection API exposed
- Methods: `show()`, `show_mut()` (only for checkbox toggling)
- Returns `InnerResponse<()>` - no selection callbacks

**Selection support**: ❌ None

### egui 0.29.1
**Purpose**: Immediate-mode GUI framework
**API**: Core UI primitives

**Key findings**:
- `egui::text_selection::LabelSelectionState` - **Handles text selection in labels**
- `LabelSelectionState::label_text_selection()` - Main selection API:
  ```rust
  pub fn label_text_selection(
      ui: &Ui,
      response: &Response,
      galley_pos: Pos2,
      galley: Arc<Galley>,
      fallback_color: Color32,
      underline: Stroke,
  )
  ```
- `LabelSelectionState::load(ctx)` / `store(ctx)` - Persist selection state
- `has_selection()`, `clear_selection()` - Query/mutate selection
- **One state for all labels** (global selection model)

**Selection support**: ✅ Full support via `LabelSelectionState`

### pulldown-cmark 0.12.2
**Purpose**: CommonMark parser (used by egui_commonmark internally)
**Direct usage**: None (wrapped by egui_commonmark)

### eframe 0.29.1
**Purpose**: egui application framework
**API**: `eframe::run_native()`, window management

## Selection Solution

**Problem**: `egui_commonmark` doesn't expose selection, but we need it for Mirror MVP.

**Solution**: Use `egui::text_selection::LabelSelectionState` directly.

**Implementation path**:
1. Parse markdown with `pulldown-cmark` (same as egui_commonmark does internally)
2. Render each element (headings, paragraphs, lists) as selectable labels
3. Use `LabelSelectionState::label_text_selection()` for each text block
4. Track selection via `LabelSelectionState::has_selection()`
5. Extract selected text from galley on selection event

**Trade-off**: Lose egui_commonmark's formatting (images, code blocks, styling) in v1, but gain native selection. Can incrementally add formatting back.

**Alternative**: Hybrid approach - use egui_commonmark for display-only regions (images, code), custom selectable labels for body text.

## Recommendation

Build custom markdown renderer using:
- `pulldown-cmark` for parsing
- `egui::text_selection::LabelSelectionState` for selection
- `egui::Label` or raw galley rendering for text

This gives us full control over selection while maintaining egui's immediate-mode model.
