# toy1_markdown_render - LEARNINGS

**Duration:** 45 min | **Status:** Complete ✅ | **Result:** Success

## Validated

- ✅ `egui_commonmark` renders CommonMark correctly
  - Headings (H1, H2, H3)
  - Bold/italic text
  - Code blocks (with syntax indication)
  - Lists
  - Links
  - Blockquotes
  - **Images** (PNG, relative paths work)
- ✅ Launch time: <1s (acceptable for toy, optimize later)
- ✅ Rendering smooth on small test file (no visible lag)
- ✅ `CommonMarkCache` required for performance (stores parsed state between frames)
- ✅ `egui::ScrollArea` provides built-in scrolling (no custom implementation needed)

## Patterns for Production

```rust
struct MarkdownApp {
    markdown_source: String,
    cache: egui_commonmark::CommonMarkCache,  // Essential for performance
}

// In update():
egui::ScrollArea::vertical().show(ui, |ui| {
    egui_commonmark::CommonMarkViewer::new().show(
        ui,
        &mut self.cache,
        &self.markdown_source,
    );
});
```

## Constraints Discovered

- `egui_commonmark` uses `pulldown-cmark` internally (no need to call it directly)
- Version compatibility: `egui 0.29` → `egui_commonmark 0.18` (check docs for newer versions)
- Must call `ctx.request_repaint()` for continuous FPS measurement (if needed)

## Performance Notes

- Small file (100 lines): Smooth rendering, no lag observed
- **Not yet tested:** Medium (1k lines) and large (10k lines) files
- **Next toy needed:** `toy2_large_file_perf` to test viewport culling if needed

## Selection Findings (Extended Toy)

**Attempted:** Native text selection via `egui::text_selection::LabelSelectionState`

**Result:** ✅ Success - Selection detection working
- `LabelSelectionState::load(ctx).has_selection()` detects active selections
- FPS during selection: 60-600 fps (excellent performance)
- Selection state is global (one selection across all labels)

**API verified:**
```rust
use egui::text_selection::LabelSelectionState;

let selection_state = LabelSelectionState::load(ctx);
if selection_state.has_selection() {
    // Selection is active
}
```

**Constraint discovered:**
- `egui_commonmark` doesn't expose the galley (rendered text layout)
- `LabelSelectionState::label_text_selection()` needs galley to extract text
- Need custom rendering to access galley for text extraction

**Production path:**
1. Parse markdown with `pulldown-cmark` directly
2. Render each text element as `egui::Label` (get galley reference)
3. Use `LabelSelectionState::label_text_selection()` per element
4. Extract selected text from galley on selection event

**No separate toy needed** - Selection API validated in toy1

## Open Questions

- Frame rate with 5k-10k line files? (Need larger test files)
- Memory usage patterns? (Need profiling tools: `cargo instruments` on macOS)
- Does `CommonMarkCache` prevent re-parsing on every frame? (Assumption: yes, verify if performance issues arise)

## Decision

**Rendering validated.** `egui_commonmark` works for display. Need separate toy for text selection implementation before MVP.
