# rendering/

Trait-based rendering system with viewport culling, lazy loading, and text selection.

## Structure

```
rendering/
├── mod.rs                  Public exports: render_content, render_comment_section
├── ui.rs                   Main render coordinator - viewport culling, batching, selection, drag
├── chunk.rs                render_chunk dispatcher - routes to appropriate renderer
├── chunk_renderer.rs       ChunkRenderer trait - strategy pattern for Text/Code/Table/Image renderers
│
├── text.rs                 Text chunk rendering with styling (bold/italic/code/headings)
├── code.rs                 Syntax-highlighted code blocks
├── table.rs                Table rendering (egui::Grid, striped rows)
├── image.rs                Image rendering with alignment/width, texture caching
│
├── text_builder.rs         Text styling system (fonts, sizing, emoji support)
├── inline_batcher.rs       Batches consecutive text chunks for horizontal flow
│
├── selection_manager.rs    Selection handling - drag lifecycle, selection bar
├── helpers.rs              calculate_line_from_y - Y position to line interpolation
├── comments.rs             Floating comment UI
│
└── viewport.rs             ViewportCuller - lazy rendering decisions
```
