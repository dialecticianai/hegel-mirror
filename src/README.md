# src/

Root source directory for Mirror - ephemeral Markdown review UI.

## Structure

```
src/
├── main.rs             Binary entry point, CLI parsing, project detection, eframe launch
├── lib.rs              Library exports for testing
├── app.rs              Main application state, multi-file tabs, review modes
├── storage.rs          Dual-mode review persistence (Hegel vs standalone routing)
├── image_manager.rs    Image loading with metadata caching for viewport culling
│
├── models/             Data structures and types (see models/README.md)
├── parsing/            Markdown to positioned TextChunks (see parsing/README.md)
├── rendering/          Chunk rendering with viewport culling (see rendering/README.md)
├── syntax/             Code syntax highlighting (see syntax/README.md)
└── theme/              Typography, spacing, colors, layout (see theme/README.md)
```
