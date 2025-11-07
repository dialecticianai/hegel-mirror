# parsing/

Markdown parsing with pulldown-cmark, converting markdown text into positioned TextChunks.

## Structure

```
parsing/
├── mod.rs              Public export: parse_markdown function
├── parser.rs           Orchestrates pulldown-cmark event stream, delegates to chunk helpers
├── chunks.rs           TextChunk creation helpers (text, code, image, table, etc)
├── html.rs             HTML block parsing for image alignment/width
└── position.rs         LineOffsets - O(log n) byte-to-line-col conversion
```
