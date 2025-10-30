# Lazy Rendering System Analysis - Large Element Scrolling Issues

**Date**: October 29, 2025
**Context**: Investigating scrolling problems with large code blocks and images that extend past viewport
**Goal**: Understand current implementation and identify improvements for handling oversized content

---

## Problem Statement

Large elements (code blocks with 500+ lines, unresized images) that extend beyond the visible viewport area cause scrolling issues. Users cannot scroll to see the full content of these elements smoothly.

**Hypothesis**: The viewport culling buffer (1000px) may be insufficient for very large elements, or the height estimation for uncached large elements is inaccurate.

---

## Current Lazy Rendering Architecture

### Core Components

**1. ViewportCuller** (`src/rendering/viewport.rs`)
- Manages viewport culling decisions
- Tracks viewport rectangle via `ui.clip_rect()`
- Uses **1000px buffer** below viewport for smooth scrolling (line 28)
- Once past viewport bottom + buffer, stops rendering chunks

**2. ChunkRenderer Trait** (`src/rendering/chunk_renderer.rs`)
- Provides trait-based rendering with viewport culling
- Four renderer implementations: Text, Code, Table, Image
- Caches actual height after first render
- Uses estimated height for offscreen chunks

**3. Render Pipeline** (`src/rendering/ui.rs`)
- Creates ViewportCuller for each frame
- Checks `should_render()` before rendering chunks
- Fast path: skip chunks entirely if past viewport AND have cached height

---

## Key Mechanisms

### Viewport Buffer

```rust
// viewport.rs:28
if start_pos.y > self.viewport.max.y + 1000.0 {
    self.past_viewport = true;
}
```

**Current behavior**:
- Renders everything up to viewport bottom + 1000px
- Once past that threshold, only renders chunks without cached height
- 1000px buffer provides smooth scrolling for normal content

**Problem for large elements**:
- A 3000px tall code block starting at viewport bottom + 500px will:
  1. Start rendering at Y=500px below viewport
  2. Extend to Y=3500px below viewport
  3. Hit the 1000px threshold mid-render
  4. May get culled/cached incompletely

### Height Estimation

**Code blocks** (`chunk_renderer.rs:111-114`):
```rust
fn estimate_height(&self, chunk: &TextChunk, theme: &Theme) -> f32 {
    let line_count = chunk.text.lines().count().max(1);
    (line_count as f32 * theme.spacing.min_line_height) + padding * 2.0
}
```

**Images** (`chunk_renderer.rs:183-185`):
```rust
fn estimate_height(&self, _chunk: &TextChunk, _theme: &Theme) -> f32 {
    300.0 // Fixed default
}
```

**Issue**: Image estimation doesn't account for actual image size until first render.

### Caching Behavior

**Height caching** (`chunk_renderer.rs:49-58`):
```rust
if culler.intersects_viewport(approx_rect) {
    // In viewport - render and cache actual height
    let (actual_height, clicked) = self.render_visible(ctx);
    ctx.chunk.cached_height = Some(actual_height);
} else {
    // Outside viewport - use cached height or estimate
    let height = ViewportCuller::get_height(ctx.chunk, estimated_height);
    ctx.ui.add_space(height);
}
```

**Problem**: If element partially intersects viewport, it renders fully (good), but if it's completely offscreen, it uses estimate (potentially wrong for images).

---

## Issues Identified

### Issue 1: Fixed Image Height Estimate

**Location**: `ImageRenderer::estimate_height()` (chunk_renderer.rs:183)

**Current**: Returns fixed 300px estimate
**Actual**: Images can be 100px - 5000px+ depending on source

**Impact**:
- Large images (2000px+) underestimated by 6-7x
- Causes incorrect scroll bar sizing
- Viewport culling decisions based on wrong dimensions
- User scrolls past image location before it loads

**Solution needed**:
- Load image dimensions during parsing (before rendering)
- Store actual dimensions in chunk metadata
- Use real dimensions for estimation

### Issue 2: 1000px Buffer Too Small for Large Elements

**Location**: `ViewportCuller::should_render()` (viewport.rs:28)

**Current**: Hardcoded 1000px buffer
**Actual**: Code blocks can be 500-10,000+ lines (10px/line = 5,000-100,000px)

**Impact**:
- Large code blocks that start near buffer threshold get partially rendered
- Subsequent scrolling finds them "past viewport" with incomplete cache
- Jerky scrolling as chunks re-render on viewport entry

**Solution needed**:
- Dynamic buffer based on chunk size
- OR: Always render chunks that intersect buffer zone completely
- OR: Increase buffer to 5000px-10000px (tradeoff: more off-screen rendering)

### Issue 3: Viewport Intersection Check Timing

**Location**: `ChunkRenderer::render()` (chunk_renderer.rs:43-44)

**Current**: Uses estimated height to create approx_rect for intersection test
**Problem**: If estimate is wrong (images), intersection test is wrong

**Impact**:
- Image thinks it's outside viewport when it's actually visible
- Gets culled with wrong cached height
- Flickers or doesn't appear until scroll forces re-render

**Solution needed**:
- For images: eager-load dimensions or use actual texture size
- OR: Force render on first encounter regardless of viewport (cache accurate height)

### Issue 4: No Special Handling for Oversized Content

**Observation**: All chunks treated equally regardless of size

**Missing**:
- Detection of "large element" (>2000px estimated height)
- Special rendering path for oversized content
- Progressive rendering for very long code blocks
- Scroll-aware rendering boundaries

**Tradeoff**: Adding complexity vs. smooth UX for edge cases

---

## Root Cause Analysis

### Primary Issue: Inaccurate Height Estimates

1. **Images**: Fixed 300px estimate fails for large images
2. **Code blocks**: Estimate is accurate but buffer insufficient for 5000+ line blocks
3. **Viewport buffer**: 1000px designed for normal paragraphs, not 3000px+ elements

### Secondary Issue: Caching Strategy

Once element is past viewport with estimate-based cache:
- Cached height may be wrong (images)
- Element never re-renders to get accurate height
- Scroll bar jumps when user finally scrolls to it and it renders

### Compounding Factor: egui Layout

egui's `horizontal()` and `vertical()` layouts don't provide intrinsic dimensions until render:
- Images: can't get texture size without loading texture
- Code blocks: syntax highlighting happens during render, not before

---

## Proposed Solutions

### Solution 1: Intelligent Buffer Sizing (Low Risk)

**Change**: `viewport.rs:28` - dynamic buffer based on next chunk estimate

```rust
// Current
if start_pos.y > self.viewport.max.y + 1000.0 {

// Proposed
let buffer_size = estimated_height.max(1000.0).min(5000.0);
if start_pos.y > self.viewport.max.y + buffer_size {
```

**Impact**:
- Large elements get larger buffer automatically
- Capped at 5000px to avoid excessive rendering
- Minimal code change, backward compatible

**Tradeoff**: Renders more off-screen content for large elements (higher GPU usage)

### Solution 2: Image Dimension Pre-loading (Medium Risk)

**Change**: Load image dimensions during parsing (parse_markdown)

Add to `TextChunk`:
```rust
pub image_height: Option<f32>,  // Actual image height
```

Update `ImageRenderer::estimate_height()`:
```rust
fn estimate_height(&self, chunk: &TextChunk, _theme: &Theme) -> f32 {
    chunk.image_height.unwrap_or(300.0)
}
```

**Impact**:
- Accurate scrollbar from start
- Better viewport culling decisions
- No flicker on scroll

**Tradeoff**:
- Slower parsing (must read image files)
- Memory overhead (store dimensions for all images)
- Complexity in parser

### Solution 3: Forced First Render (Low Risk)

**Change**: Always render chunks on first encounter, ignore viewport

Add to `TextChunk`:
```rust
pub has_rendered: bool,  // Track if chunk ever rendered
```

Update `ChunkRenderer::render()`:
```rust
if !ctx.chunk.has_rendered || culler.intersects_viewport(approx_rect) {
    let (actual_height, clicked) = self.render_visible(ctx);
    ctx.chunk.cached_height = Some(actual_height);
    ctx.chunk.has_rendered = true;
}
```

**Impact**:
- Guarantees accurate cache on first scroll-through
- Eliminates estimate errors
- Simple implementation

**Tradeoff**:
- More initial rendering (slower first scroll)
- Defeats purpose of lazy rendering for very long documents

### Solution 4: Increase Global Buffer (Lowest Risk)

**Change**: Simply increase buffer from 1000px to 5000px

```rust
if start_pos.y > self.viewport.max.y + 5000.0 {
```

**Impact**:
- Handles most large code blocks (500 lines @ 10px = 5000px)
- No architecture changes
- Immediate fix

**Tradeoff**:
- Renders 5x more off-screen content
- Higher memory/GPU usage
- Doesn't solve image estimate problem

---

## Recommended Approach

### Short Term (Immediate Fix)

1. **Increase buffer to 3000px** (Solution 4 variant)
   - Handles 90% of large code blocks
   - Zero risk, one-line change
   - Test with 1000-line code block fixture

2. **Add image dimension tracking** (Solution 2)
   - Parse image files to get dimensions
   - Store in chunk metadata
   - Use for accurate estimation

### Medium Term (Better Architecture)

3. **Dynamic buffer sizing** (Solution 1)
   - Buffer scales with chunk size
   - Capped at reasonable maximum
   - Handles edge cases gracefully

### Long Term (If Needed)

4. **Progressive rendering for huge blocks**
   - Virtualized scrolling for 10,000+ line code blocks
   - Only render visible portion + buffer
   - Requires major refactor

---

## Testing Strategy

### Test Fixtures Needed

1. **Large code block**: 500-line, 1000-line, 2000-line code blocks
2. **Huge image**: 2000px+ tall image (unresized)
3. **Mixed content**: Large code block followed by normal text
4. **Extreme case**: 5000+ line code block

### Validation Criteria

- Smooth scrolling (60fps) through large elements
- Accurate scrollbar sizing from document start
- No flicker or pop-in during scroll
- No regression on normal-sized content

---

## Conclusion

**Core Problem**: Hardcoded 1000px viewport buffer and inaccurate image height estimates cause scrolling issues for oversized content.

**Recommended Fix**:
1. Increase buffer to 3000px (immediate)
2. Add image dimension pre-loading (medium term)
3. Consider dynamic buffer sizing if issues persist

**Risk**: Low - proposed changes are incremental and backward compatible.

**Effort**: 1-2 hours for buffer change + testing, 4-6 hours for image dimensions.
