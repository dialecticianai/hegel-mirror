# Toy 2: Markdown Renderer (Bare Metal)

Ground-up markdown review UI with maximal control over text selection and line number tracking.

## Philosophy

**Built from first principles** after `toy1` revealed egui_commonmark's complexity for position tracking. This implementation:

- **Direct control**: No vendored dependencies, parse markdown ourselves
- **Position-aware**: Every text chunk knows its exact source line/col
- **Modular architecture**: Separation of concerns for context efficiency
- **Google Docs feel**: Click-to-select text, inline comments, rich rendering

## Quick Start

```bash
cargo run -- test.md
```

Click text to select → add comment → see line:col positions.

## Architecture

```
src/
├── main.rs (42 lines)           Bootstrap: CLI args → app initialization
├── app.rs (64 lines)            Coordinator: Ties all modules together
├── models/                      Data structures (no logic)
│   ├── mod.rs                   Re-exports
│   ├── chunk.rs (37 lines)      TextChunk: text + source position + styling
│   ├── selection.rs (26 lines)  Selection: chunk indices + char offsets
│   └── comment.rs (29 lines)    Comment: text + line/col range
├── parsing/                     Markdown → chunks pipeline
│   ├── mod.rs                   Re-exports
│   ├── parser.rs (229 lines)    pulldown-cmark events → TextChunks
│   └── position.rs (32 lines)   byte_to_line_col utility
├── rendering/                   UI display logic
│   ├── mod.rs                   Re-exports
│   ├── text.rs (22 lines)       Plain text with bold/italic/headings
│   ├── code.rs (25 lines)       Syntax-highlighted code blocks
│   ├── image.rs (38 lines)      Image loading + display
│   └── ui.rs (95 lines)         Main layout + comment UI
└── syntax/                      Code highlighting wrapper
    ├── mod.rs                   Re-exports
    └── highlighter.rs (54 lines) Syntect wrapper with dark theme
```

**Total: ~650 lines** across 14 files (vs 465 lines in single file)

## Key Design Decisions

### 1. Chunk-Based Rendering
Every piece of text is a `TextChunk` with:
- Source text + byte range
- Precise line/col position (1-indexed)
- Styling flags (bold, italic, code, heading)
- Type info (image path, code block lang)

**Why?** Makes selection → line number mapping trivial. Click chunk → you have exact source position.

### 2. Granular Modules
Each file does **one thing**:
- `parsing/parser.rs`: Only knows about markdown → chunks
- `rendering/text.rs`: Only knows about text → egui widgets
- `syntax/highlighter.rs`: Only knows about code → highlighted text

**Why?** Context efficiency. To tweak syntax highlighting, only read `rendering/code.rs` + `syntax/highlighter.rs` (~80 lines total).

### 3. Position Tracking Built-In
`byte_to_line_col` converts pulldown-cmark byte offsets during parsing. Every chunk created with correct line/col immediately.

**Why?** No post-processing. No widget ID mapping. Direct and simple.

### 4. Rendering by Content Type
Three specialist renderers:
- `text.rs`: Bold/italic/headings via `egui::RichText`
- `code.rs`: Syntax highlighting with dark frame background
- `image.rs`: Lazy texture loading on first render

**Why?** Each renderer optimized for its content. Easy to add new types (tables, blockquotes, etc).

## Current Features

✅ **Rich markdown rendering**
- Headings with size scaling
- Bold, italic, inline code
- Paragraphs with spacing
- Line breaks (soft/hard)

✅ **Syntax-highlighted code blocks**
- Rust, Python, JavaScript, Go support
- `base16-ocean.dark` theme
- Dark background frames
- Per-line highlighting

✅ **Inline images**
- PNG, JPG via `image` crate
- Lazy loading (only when visible)
- Texture caching

✅ **Selection + comments**
- Click any text to select
- Shows line:col range
- Add comment tied to position
- Comment list at bottom

✅ **Light theme**
- Black text on white (prose)
- Dark backgrounds only for code

## Known Limitations

**Selection**: Currently chunk-based (click = select whole chunk). Future: sub-chunk selection with char offsets.

**Position tracking**: Soft/hard breaks have placeholder positions (0:0). Doesn't affect real content.

**No code block click selection**: Code blocks render but aren't selectable yet.

**No multi-chunk selection**: Can't drag across multiple chunks yet.

## File Reading Guide

**Want to understand parsing?**
1. `parsing/position.rs` - byte→line/col conversion
2. `parsing/parser.rs` - markdown events → chunks

**Want to understand rendering?**
1. `rendering/ui.rs` - overall layout structure
2. `rendering/text.rs` - text styling
3. `rendering/code.rs` - code highlighting
4. `rendering/image.rs` - image display

**Want to add a feature?**
- New markdown element? → `parsing/parser.rs` + new renderer
- New syntax theme? → `syntax/highlighter.rs`
- New comment format? → `models/comment.rs`
- New UI layout? → `rendering/ui.rs`

## Dependencies

```toml
eframe = "0.29"        # egui framework
egui = "0.29"          # Immediate-mode GUI
pulldown-cmark = "0.12" # Markdown parsing
image = "0.25"         # Image loading
syntect = "5.2"        # Syntax highlighting
```

**Why these?**
- `pulldown-cmark`: Industry standard, gives byte offsets
- `syntect`: Powers Sublime Text, fast and reliable
- `egui`: Immediate mode = simple mental model
- `image`: Supports all common formats

## Testing

```bash
# Run with test file
cargo run -- test.md

# Run with different file
cargo run -- /path/to/your/doc.md

# Test with various languages
# (test.md includes Rust, Python, JS, Go examples)
```

## Comparison to Toy 1

| Feature | Toy 1 (egui_commonmark) | Toy 2 (bare metal) |
|---------|------------------------|-------------------|
| Line tracking | ❌ Complex, needs widget mapping | ✅ Built-in during parse |
| File size | 465 lines (monolith) | ~650 lines (14 files) |
| Modularity | ❌ Everything in main.rs | ✅ Clear separation |
| Context efficiency | ❌ Read whole file | ✅ Read only what you need |
| Syntax highlighting | ❌ Not implemented | ✅ Full support |
| Images | ❌ Not implemented | ✅ Full support |
| Dependencies | Vendored egui + commonmark | Pure crates.io |

## Future Enhancements

**Selection improvements:**
- Drag to select multiple chunks
- Character-level selection within chunks
- Keyboard shortcuts (Cmd+A, arrow keys)

**Review features:**
- Export comments to `.review.N` files
- Thread/reply to comments
- Comment resolution workflow
- Diff view for revised documents

**Rendering additions:**
- Tables (parse + grid layout)
- Blockquotes (styled frames)
- Lists (bullet/numbered rendering)
- Links (clickable with hover)
- Footnotes

**Performance:**
- Virtual scrolling for huge docs
- Incremental parsing (only visible chunks)
- Syntax highlight caching

## Design Principles Applied

From `CLAUDE.md`:

✅ **Context is king**: 14 files × ~40 lines avg = efficient reading
✅ **Artifacts disposable**: Easy to regenerate parser, swap renderers
✅ **Infrastructure compounds**: `parsing/` reusable for other tools
✅ **Refactor early**: Split at 465 lines, not 2000
✅ **Remember you're not human**: Thorough module docs, clear boundaries

## Questions for Future Sessions

**Q: How do I add a new markdown element (e.g., tables)?**
A: 1) Add case to `parsing/parser.rs` event loop, 2) Create chunk with table data, 3) Write `rendering/table.rs` renderer, 4) Add case to `rendering/ui.rs`

**Q: How do I change syntax highlighting theme?**
A: Edit `syntax/highlighter.rs`, change `"base16-ocean.dark"` to any theme from `theme_set.themes.keys()`

**Q: How do I export comments to a file?**
A: Add method to `models/comment.rs` for JSON serialization, call from `app.rs` on button click

**Q: Can this be a library?**
A: Yes! `parsing/` already has zero UI deps. Could be `hegel-md-parser` crate.

## Lessons Learned

1. **Vendoring is expensive**: toy1's vendored egui was hard to modify. Using crates.io = cleaner.

2. **Position tracking is critical**: Spent 80% of toy1 time on widget↔position mapping. toy2 does it during parse = trivial.

3. **Granularity pays off**: 14 files seems like overhead, but enables surgical edits. Changed syntax theme by editing 1 file.

4. **Immediate mode is powerful**: No widget tree, no state sync. Just redraw everything every frame. Simple and fast.

5. **Tests come free**: Parser has zero UI deps, can unit test easily. Was impossible in toy1's monolith.
