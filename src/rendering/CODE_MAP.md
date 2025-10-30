# rendering/

Trait-based rendering system with viewport culling, lazy loading, and text selection.

## Module Interface

### **mod.rs**
Public exports: render_content (main entry point), render_comment_section (floating comment UI).

## Core Rendering

### **ui.rs**
Main render_content function coordinating chunk rendering with viewport culling, inline text batching, selection management, and click-vs-drag detection. Handles drag lifecycle and selection bar drawing.

### **chunk.rs**
render_chunk dispatcher selecting appropriate renderer (Text/Code/Table/Image) based on chunk type. Builds RenderContext and delegates to ChunkRenderer trait implementations.

## Trait System

### **chunk_renderer.rs**
ChunkRenderer trait defining estimate_height and render_visible methods. Implements Strategy pattern via TextRenderer, CodeRenderer, TableRenderer, ImageRenderer. Provides unified render method with viewport culling and cached height logic. Eliminates code duplication by centralizing culling/caching logic in trait's default render() implementation.

## Specialized Renderers

### **text.rs**
Text chunk rendering with styling support (bold/italic/code/headings). Uses text_builder for emoji support and drag sensing via egui::interact.

### **code.rs**
Syntax-highlighted code block rendering via syntect. Applies theme colors for background/padding, syntect colors for syntax highlighting.

### **table.rs**
Table rendering using egui::Grid with striped rows. Handles column alignment (center/left/right) and header styling via text_builder.

### **image.rs**
Image loading from disk (via image crate) and rendering with alignment (center/left/right) and width constraints. Caches loaded textures in HashMap. Single-line selection on click.

## Text Styling

### **text_builder.rs**
Centralized text styling system. build_styled_text creates RichText with font families (Bold/Italic/BoldItalic), context-specific sizing (Body/Heading/TableCell), and inline code styling. render_styled_text wraps with EmojiLabel for colored emoji support via egui-twemoji.

### **inline_batcher.rs**
Batches consecutive inline text chunks into horizontal_wrapped layouts for proper text flow. Detects inline-able chunks (excludes images/code blocks/tables/headings/bullets/newlines) and finds batch ranges.

## Selection System

### **selection_manager.rs**
Centralized selection handling: drag start/update/end, hover-based selection updates, click-to-clear detection, and selection bar drawing. Uses LayoutMap for line-to-Y mapping and helpers for precise line calculation from Y position.

### **helpers.rs**
calculate_line_from_y utility interpolating Y position to precise line number within multi-line chunks. Used by selection_manager for accurate line targeting during drag.

## Performance

### **viewport.rs**
ViewportCuller managing lazy rendering decisions. Tracks viewport bounds, determines render-vs-skip for chunks, and handles cached height fallback for offscreen chunks. Provides 1000px buffer below viewport for smooth scrolling.
