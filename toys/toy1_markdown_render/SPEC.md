# toy1_markdown_render - SPEC

## Purpose

Validate `pulldown-cmark` → `egui_markdown` rendering pipeline for Mirror MVP.

## Learning Goals

1. Does `egui_markdown` handle CommonMark correctly? (headings, lists, code blocks, links) ✅
2. Frame rate with large files? (5k lines, 10k lines)
3. Memory usage patterns? (heap allocations, egui retained state)
4. Can we lazy-render (viewport culling) or must we render full document?
5. **Text selection:** Can we detect text selection in rendered Markdown?
6. **Selection state:** How to track selected text (char offsets, line/col, text content)?

## Napkin Physics

**Problem:** Render Markdown in egui window without reinventing parsers.

**Assumptions:**
- `pulldown-cmark` handles CommonMark parsing
- `egui_markdown` provides egui widgets for rendered output
- Immediate mode = redraw every frame (60fps target)
- Large files (10k+ lines) may need viewport culling

**Invariant:** Parse once per file load, render visible region per frame.

**Mechanism:**
1. Load Markdown file → string
2. Parse with `pulldown-cmark` → event stream
3. Pass to `egui_markdown` → egui widgets
4. Measure frame rate, memory usage

**First Try:** Render entire document every frame. If <60fps on 10k lines, add viewport culling.

## Inputs

- Markdown file path (CLI arg)
- Test files: `small.md` (100 lines), `medium.md` (1k lines), `large.md` (10k lines)

## Outputs

- egui window displaying rendered Markdown
- Frame rate measurement (egui built-in profiler)
- Memory usage (heap allocations via `cargo instruments` or manual observation)

## Success Criteria

- ✅ All CommonMark features render correctly (headings, lists, code blocks, links, emphasis)
- ✅ 60fps on 1k-line file
- ✅ <500ms cold launch time
- ⏱️ Frame rate on 10k-line file documented (may fail, that's the learning)

## Non-Goals

- Text selection (separate toy)
- Multi-file tabs (separate toy)
- Themes/styling (use egui defaults)
- Scrolling optimization (egui built-in ScrollArea)

## Test Plan

**Unit tests:** None (rendering is visual, test manually)

**Manual validation:**
1. Run with `small.md` → verify all Markdown features render
2. Run with `medium.md` → measure frame rate (egui profiler)
3. Run with `large.md` → measure frame rate, observe lag/stuttering
4. Document measurements in LEARNINGS.md

## Dependencies

```toml
[dependencies]
eframe = "0.29"
egui = "0.29"
egui_markdown = "0.4"
pulldown-cmark = "0.12"
```

## Time Box

**Target:** 90 minutes (30 min setup, 30 min impl, 30 min validation/LEARNINGS)

**3-attempt rule applies** if unexpected issues arise.
