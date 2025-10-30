# ImageManager Implementation Plan

Incremental TDD plan for building centralized lazy image loading system.

---

## Overview

**Goal**: Replace duplicate image loading with ImageManager that loads metadata during parsing and textures on-demand during rendering.

**Scope**:
- Create ImageManager module with metadata and texture caching
- Update parser to use ImageManager for dimension loading
- Update renderer to use ImageManager for texture access
- Maintain backward compatibility with existing image rendering

**Priorities**:
1. Correct lazy loading behavior (metadata upfront, textures on-demand)
2. Zero duplicate file I/O
3. Maintain 60fps scrolling performance
4. Graceful error handling for missing/corrupt images

---

## Methodology

**TDD Principles**:
- Write failing tests first for each operation
- Implement minimal code to pass tests
- Commit after each numbered step completion
- Focus on public API contracts, not internals

**What to Test**:
- Metadata loading returns correct dimensions
- Texture caching works (subsequent calls instant)
- Missing files handled gracefully
- Lazy loading doesn't load textures until needed

**What NOT to Test**:
- Internal hash map implementation details
- Exact error message formatting
- Performance metrics (manual validation only)

---

## Step 1: Create ImageManager Module with Metadata Loading

### Goal
Establish core module structure with metadata-only loading capability.

### Step 1.a: Write Tests

Create test file for ImageManager with basic metadata loading tests:
- Test loading valid image returns correct dimensions
- Test loading same image twice returns cached dimensions
- Test loading missing image returns None
- Test loading corrupt image returns None

Expected behavior: metadata cache populated, no textures created yet.

### Step 1.b: Implement

Tasks:
1. Create new file for ImageManager module
2. Define ImageManager struct with metadata HashMap
3. Define ImageMetadata struct with width, height, path
4. Implement load_metadata method using image crate
5. Add metadata caching logic
6. Handle file read and decode errors gracefully

Key considerations:
- Use image crate's Reader API to decode headers only (fast)
- Store resolved absolute paths in metadata
- Return None on any error, no panic

### Success Criteria
- [ ] ImageManager module compiles
- [ ] load_metadata returns dimensions for valid images
- [ ] Repeated calls return cached data without file I/O
- [ ] Missing/corrupt images return None gracefully
- [ ] All tests pass

---

## Step 2: Add Lazy Texture Loading

### Goal
Enable on-demand texture creation when images are rendered.

### Step 2.a: Write Tests

Add tests for lazy texture loading:
- Test get_or_load_texture creates texture on first call
- Test subsequent calls return cached texture
- Test texture loading without metadata fails gracefully
- Test texture caching works across multiple calls

Expected behavior: texture only created when explicitly requested, not during metadata load.

### Step 2.b: Implement

Tasks:
1. Add textures HashMap to ImageManager
2. Add egui Context to ImageManager constructor
3. Implement get_or_load_texture method
4. Load full image, convert to RGBA8, create ColorImage
5. Call ctx.load_texture and cache result
6. Return reference to cached texture

Key considerations:
- Check metadata exists before loading texture
- Full image decode only happens on first get_or_load_texture call
- Texture handle lifetime managed by HashMap

### Success Criteria
- [ ] get_or_load_texture loads texture on first call
- [ ] Subsequent calls return cached texture instantly
- [ ] Calling without prior metadata load returns None
- [ ] Texture cache persists for manager lifetime
- [ ] All tests pass

---

## Step 3: Add Dimension Query Method

### Goal
Provide read-only access to cached dimensions without triggering texture load.

### Step 3.a: Write Tests

Add tests for dimension querying:
- Test get_dimensions returns dimensions for loaded metadata
- Test get_dimensions returns None for unloaded images
- Test get_dimensions doesn't trigger texture loading

Expected behavior: pure read operation, no side effects.

### Step 3.b: Implement

Tasks:
1. Implement get_dimensions method
2. Look up metadata cache
3. Return tuple of width and height
4. Ensure no texture loading side effects

Key considerations:
- Simple cache lookup, no file I/O
- Independent of texture cache state

### Success Criteria
- [ ] get_dimensions returns correct values
- [ ] Returns None for non-existent images
- [ ] Does not load textures as side effect
- [ ] All tests pass

---

## Step 4: Integrate ImageManager into Parsing

### Goal
Replace current full image loading in parser with metadata-only loading.

### Step 4.a: Write Tests

Add integration tests for parser usage:
- Test parsing markdown with images populates TextChunk.image_height
- Test parsing handles missing images gracefully
- Test parsing performance with multiple images

Expected behavior: dimensions available after parsing, no textures loaded yet.

### Step 4.b: Implement

Tasks:
1. Add ImageManager field to parsing context
2. Update push_image_chunk_with_alignment to call load_metadata
3. Remove current full image loading code
4. Calculate display height from metadata dimensions
5. Set TextChunk.image_height from calculated value

Key considerations:
- Base path resolution for relative image paths
- Width constraints applied during height calculation
- Fallback to None if metadata load fails

### Success Criteria
- [ ] Parser creates ImageManager instance
- [ ] Image chunks have correct image_height values
- [ ] No texture loading during parsing phase
- [ ] Missing images handled without panic
- [ ] Parsing performance improved (header read only)
- [ ] All existing parser tests pass

---

## Step 5: Integrate ImageManager into Rendering

### Goal
Update renderer to use ImageManager for texture access instead of direct loading.

### Step 5.a: Write Tests

Add integration tests for rendering:
- Test renderer gets textures from ImageManager
- Test first render triggers texture load
- Test subsequent renders use cached texture
- Test missing images show error message

Expected behavior: textures loaded lazily only when scrolled into viewport.

### Step 5.b: Implement

Tasks:
1. Update Document struct to hold ImageManager reference
2. Update render_image function to call get_or_load_texture
3. Remove load_image_texture function (now obsolete)
4. Remove loaded_images HashMap from Document (replaced by ImageManager)
5. Pass egui context to get_or_load_texture

Key considerations:
- Texture loading happens in render path, not parse path
- Images scrolled past never rendered = never loaded
- Error handling maintains existing fallback behavior

### Success Criteria
- [ ] Renderer uses ImageManager for texture access
- [ ] First render of image loads texture
- [ ] Subsequent frames use cached texture
- [ ] Missing images display error message
- [ ] No duplicate image loading
- [ ] All existing rendering tests pass

---

## Step 6: Remove Legacy Image Loading Code

### Goal
Clean up obsolete code now replaced by ImageManager.

### Step 6.a: Write Tests

Verify all functionality still works after cleanup:
- Test end-to-end image rendering with various formats
- Test images with width constraints
- Test images with different alignments
- Test mixed documents with text and images

Expected behavior: identical to before, but with ImageManager implementation.

### Step 6.b: Implement

Tasks:
1. Remove load_image_dimensions helper from chunks.rs
2. Remove direct image crate usage from chunks.rs
3. Update all TextChunk construction sites to include image_height
4. Verify no dead code remains
5. Update documentation

Key considerations:
- All image loading centralized in ImageManager
- No regression in functionality
- Code cleaner and more maintainable

### Success Criteria
- [ ] Legacy image loading code removed
- [ ] No compilation warnings
- [ ] All tests pass
- [ ] Documentation updated
- [ ] No functional regressions

---

## Step 7: Performance Validation

### Goal
Verify lazy loading achieves performance goals for large documents.

### Step 7.a: Write Tests

Manual validation tests (not automated):
- Create test document with 50+ images
- Measure parsing time (should be fast, metadata only)
- Verify scroll performance remains 60fps
- Verify memory usage reasonable (only visible images loaded)

Expected behavior: fast parsing, smooth scrolling, memory proportional to visible images.

### Step 7.b: Validate

Tasks:
1. Create temporary test directory in /tmp
2. Symlink existing logo.png 50 times with different names
3. Generate markdown file referencing all 50 images in tmp dir
4. Open in mirror and measure parsing time
5. Scroll through document and verify smoothness
6. Monitor memory usage with Activity Monitor
7. Clean up tmp directory after validation
8. Document findings in commit message

Key considerations:
- Parsing should complete in <1 second for 50 images
- Scrolling maintains 60fps
- Memory doesn't spike until images rendered
- Images off-screen never loaded

### Success Criteria
- [ ] Parsing 50 images completes in <1 second
- [ ] Scrolling remains smooth (60fps)
- [ ] Memory usage proportional to rendered images
- [ ] No performance regressions vs baseline

---

## Step 8: Error Handling Hardening

### Goal
Ensure robust error handling for all edge cases.

### Step 8.a: Write Tests

Add tests for edge cases:
- Test very large images (5000x5000)
- Test tiny images (1x1)
- Test symlinked images
- Test images with unusual formats
- Test concurrent access patterns

Expected behavior: all cases handled gracefully with clear error messages or fallbacks.

### Step 8.b: Implement

Tasks:
1. Add size validation for extremely large images
2. Test symlink resolution
3. Add format validation if needed
4. Document limitations in code comments

Key considerations:
- No crashes or panics regardless of input
- Clear error messages for user
- Limitations documented

### Success Criteria
- [ ] Large images handled (or limited explicitly)
- [ ] Symlinks work correctly
- [ ] All formats supported by image crate work
- [ ] No panics or crashes
- [ ] All edge case tests pass

---

## Commit Strategy

Each step commits after successful test passage:

- `test(image): add metadata loading tests` (Step 1.a)
- `feat(image): implement ImageManager with metadata loading` (Step 1.b)
- `test(image): add lazy texture loading tests` (Step 2.a)
- `feat(image): add lazy texture loading to ImageManager` (Step 2.b)
- `test(image): add dimension query tests` (Step 3.a)
- `feat(image): add get_dimensions method` (Step 3.b)
- `test(image): add parser integration tests` (Step 4.a)
- `feat(image): integrate ImageManager into parser` (Step 4.b)
- `test(image): add renderer integration tests` (Step 5.a)
- `feat(image): integrate ImageManager into renderer` (Step 5.b)
- `refactor(image): remove legacy image loading code` (Step 6.b)
- `docs(image): validate performance with large documents` (Step 7.b)
- `test(image): add edge case handling tests` (Step 8.a)
- `feat(image): harden error handling` (Step 8.b)

---

## Integration Points

**Parser** (`src/parsing/chunks.rs`):
- Replace load_image_dimensions call with image_manager.load_metadata
- Pass metadata to TextChunk creation

**Renderer** (`src/rendering/image.rs`):
- Replace load_image_texture with image_manager.get_or_load_texture
- Remove texture caching logic (now in ImageManager)

**Document** (`src/models/document.rs` or `src/app.rs`):
- Replace loaded_images HashMap with ImageManager instance
- Pass ImageManager to parser and renderer

**TextChunk** (already done):
- image_height field populated during parsing

---

## Rollback Strategy

If ImageManager introduces issues:
1. Revert commits in reverse order
2. Legacy code preserved in git history
3. Can roll back to pre-ImageManager state cleanly

Each commit is atomic and tested, making bisection easy if regressions occur.

---

## Success Metrics

**Functional**:
- Zero duplicate image file reads
- Dimensions available immediately after parsing
- Textures loaded only when rendered
- All existing functionality preserved

**Performance**:
- Parsing speed improved (metadata vs full decode)
- Memory usage reduced (lazy texture loading)
- Scrolling remains smooth (60fps)

**Quality**:
- Full test coverage for ImageManager API
- No panics or crashes regardless of input
- Clean, maintainable code structure
- Clear documentation
