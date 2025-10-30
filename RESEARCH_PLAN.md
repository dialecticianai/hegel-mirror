# Research Plan - Mirror Markdown Rendering Engine

**Goal**: Systematically catalog all markdown features Mirror currently parses and renders to identify test fixture gaps.

**Success criteria**:
- [ ] Priorities 0-2 complete (parsing + rendering analysis)
- [ ] Comprehensive feature catalog created
- [ ] Test coverage gaps documented
- [ ] Ready to create systematic test fixtures

---

## Priority 0: Parsing Layer Analysis (FOUNDATIONAL)

**Objective**: Understand what markdown features we extract from pulldown-cmark events

**Study targets**:
- `src/parsing/parser.rs` - Main event stream processor
- `src/parsing/chunks.rs` - TextChunk creation helpers
- `src/parsing/html.rs` - HTML block parsing for images
- `src/models/chunk.rs` - TextChunk structure definition

**Deliverable**: Catalog of all markdown elements we parse:
- Text features (bold, italic, code, headings)
- Block elements (code blocks, tables, images)
- Special handling (HTML alignment, width attributes)

**Target**: 0.5 session

---

## Priority 1: Rendering Layer Analysis (CORE)

**Objective**: Map parsed features to visual rendering capabilities

**Study targets**:
- `src/rendering/chunk_renderer.rs` - Trait implementations (Text/Code/Table/Image)
- `src/rendering/text_builder.rs` - Text styling system
- `src/rendering/code.rs` - Syntax highlighting
- `src/rendering/table.rs` - Table grid rendering
- `src/rendering/image.rs` - Image loading and alignment

**Deliverable**: Feature-to-renderer mapping:
- Which chunk types render which visual elements
- Styling capabilities (fonts, colors, sizes)
- Layout features (alignment, spacing, wrapping)

**Target**: 0.5 session

---

## Priority 2: Existing Test Analysis (PRACTICAL)

**Objective**: Identify what we already test vs gaps

**Study targets**:
- `tests/fixtures/basic.md` - Basic features
- `tests/fixtures/tables.md` - Table rendering
- `tests/fixtures/unicode.md` - Special characters
- Root test files: `test_bold.md`, `test_centered_image.md`, etc.

**Deliverable**: Coverage matrix:
- Features tested ✅
- Features untested ❌
- Edge cases missing 🔶

**Target**: 0.5 session

---

## Priority 3: Edge Case Discovery (DEFER TO DISCOVERY)

**Objective**: Find corner cases and rendering limits

**Examples** (study AFTER catalog):
- Nested bold/italic combinations
- Code blocks with unusual languages
- Tables with complex alignments
- Images with edge-case dimensions
- Unicode emoji edge cases

**Note**: Don't deep-dive here during research. Catalog enough to know what exists, defer detailed testing to Discovery phase.

---

## Priority 4: Performance Characteristics (OUT OF SCOPE)

**Objective**: Understand rendering performance (NOT THIS RESEARCH)

**Defer because**:
- Already documented in ARCHITECTURE.md (60fps, 11K lines)
- Not needed for test fixture creation
- Performance testing is separate concern

---

## Research Approach

**Method**: Code archaeology + synthesis

1. Read source files systematically (Priority 0, 1, 2 order)
2. Extract feature lists as I go
3. Synthesize into comprehensive catalog
4. Cross-reference with existing tests
5. Document gaps

**Output format**: Structured catalog with categories:
- Text formatting (inline)
- Block elements
- Special features (alignment, syntax highlighting)
- Known edge cases
- Test coverage status

**Time box**: 1.5 sessions total (0.5 per priority)

---

## Anti-Patterns to Avoid

❌ Reading all rendering code line-by-line (too detailed)
✅ Identify feature capabilities from public interfaces

❌ Testing features now (that's Discovery phase)
✅ Catalog what features exist

❌ Perfectionism about edge cases
✅ Good-enough catalog to guide test creation

---

## Success Check

Ready to exit research when:
- [x] Can list all markdown features we support
- [x] Know which are tested vs untested
- [x] Have structured catalog for reference
- [x] Understand rendering capabilities

Then: Discovery phase creates comprehensive test fixtures based on this research.
