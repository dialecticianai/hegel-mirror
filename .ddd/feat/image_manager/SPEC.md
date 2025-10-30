# ImageManager Specification

Centralized image loading system that pre-loads all images once, caches dimensions and textures, eliminates duplicate loading.

---

## Overview

**What it does**: Manages all image loading for markdown documents - loads each image file exactly once during parsing, extracts dimensions for height estimation, converts to egui textures, and provides cached textures to renderers.

**Key principles**:
- Load each image file exactly once (no duplicate I/O)
- Extract dimensions during load (height estimation for viewport culling)
- Cache egui textures for instant rendering
- Fail gracefully for missing/corrupt images
- Memory-efficient (release unused images on document close)

**Scope**: Production feature - replaces current dual-loading system in `src/parsing/chunks.rs` and `src/rendering/image.rs`

**Integration context**:
- Called by `parse_markdown()` to pre-load all images
- Provides dimensions to `TextChunk` for viewport culling
- Provides textures to `ImageRenderer` for rendering
- Replaces `HashMap<String, TextureHandle>` in `Document` struct

---

## Data Model

### ImageManager Struct

```rust
pub struct ImageManager {
    // Image metadata (dimensions always loaded)
    metadata: HashMap<String, ImageMetadata>,
    // Lazy-loaded textures (only when rendered)
    textures: HashMap<String, egui::TextureHandle>,
    // Base path for resolving relative paths
    base_path: PathBuf,
}

struct ImageMetadata {
    // Original dimensions from image file
    width: u32,
    height: u32,
    // Full resolved path
    full_path: PathBuf,
}
```

### API Contract

**Input**: Image file path (relative or absolute)
**Output**: Dimensions (w, h) and cached texture
**Error**: Returns `None` for missing/corrupt images

---

## Core Operations

### Operation 1: Load Metadata (Dimensions Only)

**Syntax**: `image_manager.load_metadata(path: &str) -> Option<(u32, u32)>`

**Parameters**:
- `path`: File path to image (PNG, JPG, etc.)

**Behavior**:
1. Check if metadata already loaded (cache hit) → return cached dimensions
2. Read file from disk
3. Decode image headers only (fast, minimal memory)
4. Extract width/height
5. Store metadata in cache (NO texture creation yet)
6. Return (width, height)

**Validation**:
- File must exist and be readable
- File must be valid image format
- Returns `None` on any error (no panic)

**Performance**: ~1-5ms per image (header read only, no full decode)

**Example**:
```rust
// During parsing - fast, just dimensions
if let Some((w, h)) = image_manager.load_metadata("logo.png") {
    chunk.image_height = Some(calculate_display_height(w, h, width_constraint));
}
```

### Operation 2: Get or Load Texture (Lazy)

**Syntax**: `image_manager.get_or_load_texture(ctx: &egui::Context, path: &str) -> Option<&TextureHandle>`

**Parameters**:
- `ctx`: egui context for texture creation
- `path`: Same path used in `load_metadata()`

**Behavior**:
1. Check texture cache - if exists, return immediately
2. If not cached:
   a. Check metadata cache for dimensions
   b. Read file from disk
   c. Decode full image to RGBA8
   d. Create egui ColorImage
   e. Load texture via `ctx.load_texture()`
   f. Store in texture cache
3. Return reference to texture handle

**Validation**:
- Returns `None` if metadata not loaded (must call `load_metadata()` first)
- Returns `None` if file no longer exists or decode fails
- Path must match exactly (case-sensitive)

**Performance**:
- First call: ~10-50ms per image (full decode + GPU upload)
- Subsequent calls: ~0ms (cached)

**Example**:
```rust
// During rendering - lazy loads on first render
if let Some(texture) = image_manager.get_or_load_texture(ctx, "logo.png") {
    ui.add(egui::Image::new(texture).fit_to_exact_size(display_size));
}
```

### Operation 3: Get Dimensions

**Syntax**: `image_manager.get_dimensions(path: &str) -> Option<(u32, u32)>`

**Parameters**:
- `path`: Image file path

**Behavior**:
1. Look up in cache
2. Return (width, height)
3. Returns `None` if not loaded

**Validation**:
- Read-only operation
- No disk I/O (cache only)

**Example**:
```rust
// For height calculation
if let Some((w, h)) = image_manager.get_dimensions("logo.png") {
    let aspect_ratio = h as f32 / w as f32;
    let display_height = desired_width * aspect_ratio;
}
```

---

## Test Scenarios

### Simple Case
**Input**: Single image file `test.png` (100x50 pixels)
**Expected**:
- `load_image("test.png")` returns `Some((100, 50))`
- `get_texture("test.png")` returns valid texture handle
- `get_dimensions("test.png")` returns `Some((100, 50))`

### Complex Case
**Input**: Multiple images with various formats and sizes
```
- logo.png (800x600)
- icon.jpg (32x32)
- banner.png (1200x300)
```
**Expected**:
- Each image loaded exactly once (verify via I/O tracing)
- All dimensions cached correctly
- All textures available for rendering
- Memory usage scales with image count

### Error Cases

**Missing file**:
- `load_image("nonexistent.png")` returns `None`
- No panic, no error log spam

**Corrupt file**:
- `load_image("corrupt.png")` returns `None`
- Graceful failure, no crash

**Duplicate load**:
- `load_image("test.png")` called twice
- Second call is no-op, returns cached dimensions
- Only one file read

### Integration Case
**Input**: Markdown document with 5 images
**Expected**:
- Parser calls `load_image()` for each image during parsing
- All dimensions stored in `TextChunk.image_height`
- Renderer calls `get_texture()` during viewport rendering
- Zero duplicate file reads
- Scrolling performance: 60fps with cached textures

---

## Success Criteria

### Functional
- [ ] ImageManager loads PNG, JPG, JPEG image formats
- [ ] Each image file read from disk exactly once
- [ ] Dimensions extracted and cached correctly
- [ ] Textures created and cached for rendering
- [ ] Missing images return `None` without panic
- [ ] Corrupt images handled gracefully

### Performance
- [ ] Loading images during parsing adds <100ms for 10 images
- [ ] Rendering uses cached textures (zero I/O during scroll)
- [ ] Memory usage: ~image file size + texture overhead per image
- [ ] No memory leaks (manager dropped when document closed)

### Integration
- [ ] Replaces `HashMap<String, TextureHandle>` in Document
- [ ] Parser calls `load_image()` in `push_image_chunk_with_alignment()`
- [ ] Renderer calls `get_texture()` in `render_image()`
- [ ] TextChunk.image_height populated with actual dimensions
- [ ] ImageRenderer.estimate_height() uses real dimensions
- [ ] No regression in existing image rendering functionality

### Edge Cases
- [ ] Empty path handled (returns None)
- [ ] Very large images (5000x5000) load without crash
- [ ] Symlinked images resolve and load correctly
- [ ] Relative vs absolute paths work consistently
- [ ] Case sensitivity handled per OS (Linux vs macOS vs Windows)

---

## Error Handling

**File Not Found**:
- Return `None` from `load_image()`
- Renderer shows `[Failed to load image: path]`
- No error logs (expected case for broken links)

**Decode Failure**:
- Return `None` from `load_image()`
- Same fallback behavior as missing file

**Out of Memory**:
- Texture allocation failure handled by egui
- Document as known limitation (100+ large images)

---

## Performance Targets

**Parsing overhead**: <10ms per image for pre-loading
**Rendering**: Zero I/O, uses cached textures
**Memory**: ~2x file size per image (decoded RGBA + texture)
**Scrolling**: 60fps maintained with cached textures

---

## Migration Path

### Current System
1. Parser: Loads full image to get dimensions (chunks.rs:157) - SLOW
2. Renderer: Loads full image again to create texture (image.rs:15) - DUPLICATE

### New System (Lazy Loading)
1. Parser: Calls `image_manager.load_metadata()` - just dimensions (FAST)
2. Dimensions cached for viewport culling estimates
3. Renderer: Calls `image_manager.get_or_load_texture()` on first render
4. Texture cached for subsequent frames
5. Images scrolled past but never rendered = never loaded (MEMORY EFFICIENT)

### Backwards Compatibility
- TextChunk.image_height remains optional (fallback to 300px)
- Rendering falls back to error message for failed images
- No breaking changes to external API

---

## Security Considerations

**Path Traversal**:
- Images resolved relative to markdown file location
- No additional validation needed (existing Path::join() handles this)

**Malicious Images**:
- `image` crate handles decode safety
- Large images limited by available memory
- No server-side execution (local files only)

---

## Open Questions

**Q**: Should we lazy-load textures (load dimensions during parsing, textures on-demand)?
**A**: Yes - load metadata (dimensions) during parsing, textures on first render. This handles huge documents with 100+ images without loading all textures into GPU memory upfront.

**Q**: Handle animated GIFs?
**A**: Out of scope - `image` crate loads first frame only, acceptable for MVP.

**Q**: Cache invalidation strategy?
**A**: Manager lifetime = document lifetime. New document = new manager. No explicit invalidation needed.
