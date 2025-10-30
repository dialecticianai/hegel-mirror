# parsing/

Markdown parsing with pulldown-cmark, converting markdown text into positioned TextChunks.

## Module Interface

### **mod.rs**
Public export: parse_markdown function (entry point for all parsing).

## Parser

### **parser.rs**
Main parse_markdown function orchestrating pulldown-cmark event stream processing. Handles markdown elements (text, code blocks, tables, images, headings, bold/italic), HTML blocks (for centered images), and style state tracking. Delegates chunk creation to chunks.rs helpers.

## Chunk Creation

### **chunks.rs**
Helper functions for creating TextChunks: push_text_chunk, push_code_chunk, push_break_chunk, push_image_chunk (with alignment/width variants), and push_table_chunk. Each function converts markdown elements and byte ranges into positioned TextChunks using LineOffsets.

### **html.rs**
HTML block parsing for image alignment. Extracts image src, alignment (center/left/right), and width from `<p align="..."><img src="..." width="..."></p>` patterns. Regex-free string parsing with quote handling (single/double). Includes comprehensive unit tests.

## Position Tracking

### **position.rs**
LineOffsets struct providing O(log n) byte-to-line-col conversion via precomputed line offset table. Binary search for line lookup, character counting for column calculation. Eliminates O(n) scanning on every event.
