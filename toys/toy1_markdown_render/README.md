# toy1_markdown_render

Validates `pulldown-cmark` → `egui_commonmark` rendering pipeline for Mirror MVP.

## Purpose

Test Markdown rendering performance, correctness, and memory usage in egui immediate-mode GUI.

## Key API

```rust
egui_commonmark::CommonMarkViewer::new()
    .show(ui, &mut cache, &markdown_source);
```

## Gotchas

- `egui_commonmark` uses `pulldown-cmark` internally (dependency handled)
- Must use `CommonMarkCache` for performance (stores parsed state)
- `ctx.request_repaint()` needed for continuous FPS measurement

## Run

```bash
cargo run -- test.md
```

## Test Files

Create `small.md`, `medium.md`, `large.md` in toy directory for validation.
