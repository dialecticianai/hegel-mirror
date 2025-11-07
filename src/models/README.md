# models/

Core data structures representing application state and markdown content.

## Structure

```
models/
├── mod.rs              Public exports
├── chunk.rs            TextChunk - parsed markdown element with position, styling, cached height
├── comment.rs          Comment with text and line/col position
├── document.rs         Document - per-file review state, write_review/write_approval routing
├── selection.rs        Selection state for drag lifecycle
├── layout.rs           LayoutMap - chunk positions for selection bar (line → Y mapping)
├── review_mode.rs      ReviewMode enum (Immediate/Batched)
└── table.rs            Table structure (alignments, header, rows)
```
