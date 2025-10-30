# models/

Core data structures representing application state and markdown content.

## Module Interface

### **mod.rs**
Public exports: TextChunk, Alignment, Comment, Document, LayoutMap, ReviewMode, Selection, Table.

## Data Models

### **chunk.rs**
TextChunk structure representing a parsed markdown element with source position (line/col ranges), styling flags (bold, italic, code, heading_level), layout hints (newline_after), and type-specific data (image_path with alignment/width, code_block_lang, table). Includes cached_height for viewport culling performance.

### **comment.rs**
Comment structure holding review comment text with line/col position ranges. Provides format() method for display.

### **document.rs**
Document structure encapsulating per-file review state: filename, source text, parsed chunks (lazily initialized), selection state, comment queue, loaded image textures, layout map, storage handler, and approval flag. Used for multi-file tab support.

### **selection.rs**
Selection state tracking start/end lines and drag status. Provides methods for drag lifecycle (start_drag, update_drag, end_drag), active state checking, and line containment testing.

### **layout.rs**
LayoutMap tracking chunk positions for selection bar rendering. Maps line numbers to Y coordinates via binary search. Supports per-line interpolation within multi-line chunks for precise selection anchoring.

### **review_mode.rs**
ReviewMode enum: Immediate (append comments to disk instantly) vs Batched (queue in memory, atomic write on submit). Defaults to Immediate.

### **table.rs**
Table structure holding column alignments, header row, and body rows. Created from pulldown_cmark table events during parsing.
