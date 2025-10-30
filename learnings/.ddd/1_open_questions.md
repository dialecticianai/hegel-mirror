# Open Questions — Mirror Markdown Rendering Test Coverage

**Created**: October 29, 2025
**Purpose**: Bridge Research → Discovery with systematic test fixture creation roadmap
**Status**: 15 open, 3 answered, 18 total

---

## Quick Summary

**Study complete**: Comprehensive markdown feature catalog covering parsing, rendering, and architecture
**Open questions**: 15 (14 practice, 1 decision)
**Answered/decided**: 3 (during research)
**Primary blockers**: None - ready to proceed with Discovery

**Categories**:
1. Test Fixture Organization (2 open, 1 answered)
2. Missing Feature Tests (6 open, 0 answered)
3. Edge Case Validation (5 open, 1 answered)
4. Implementation Gaps (2 open, 1 answered)

---

## 1. Test Fixture Organization

### ✅ Answered Questions

**Q1.1**: How should we organize test fixtures?
- ✅ **ANSWERED**: Use `tests/fixtures/` directory structure
  - Source: `learnings/.ddd/0_markdown_features_research.md` - Decision 2
  - Rationale: Directory already exists, clean separation, easy to run individually
- **Next step**: Create fixtures following hierarchical structure

**Q1.2**: What naming convention?
- ✅ **ANSWERED**: Descriptive feature-based names
  - Pattern: `tests/fixtures/[feature_name].md`
  - Examples: `inline_formatting.md`, `headings.md`, `lists.md`
  - Source: Research assessment Decision 2
- **Next step**: Use convention consistently for all new fixtures

### Open Questions

**Q1.3**: Should each feature have its own file or combine related features?
- **Current approach**: Combine related features in single file (e.g., all inline formatting together)
- **Trade-off**: Single file = easier comparison, multiple files = isolated testing
- **Answer via**: **Discovery Phase** - Create first few fixtures, evaluate which approach feels more natural when reviewing in Mirror

---

## 2. Missing Feature Tests

**Theory source**: `learnings/markdown_features.md` - Test Coverage Gaps section

### Inline Code
**Q2.1**: Does inline code render with correct monospace font and pink color?
- **Theory**: `markdown_features.md` lines 28-31 - Should use monospace + pink color
- **Answer via**: **Toy 1** - Create `inline_code.md` with backtick examples, open in Mirror, verify font family and color match theme.colors.inline_code

### Headings
**Q2.2**: Do all heading levels (H1-H6) render with correct font sizes?
- **Theory**: `markdown_features.md` lines 41-50 - Font sizes [32, 28, 24, 20, 16, 14]px
- **Answer via**: **Toy 2** - Create `headings.md` with all 6 levels, open in Mirror, verify visual hierarchy matches documented sizes

**Q2.3**: How do headings handle very long text?
- **Theory**: `markdown_features.md` line 48 - "Heading text flows inline until newline"
- **Answer via**: **Toy 2** - Include long heading in `headings.md`, verify wrapping behavior

### Standard Images
**Q2.4**: Does standard markdown image syntax work?
- **Theory**: `markdown_features.md` lines 83-86 - Should load from filesystem, display at original size
- **Current gap**: Only HTML-enhanced images tested
- **Answer via**: **Toy 3** - Create `images.md` with `![alt](path)` syntax, verify loading and display

### Lists
**Q2.5**: Do unordered lists render correctly with bullets preserved?
- **Theory**: `markdown_features.md` lines 104-109 - Bullets preserved, no horizontal wrapping
- **Answer via**: **Toy 4** - Create `lists.md` with `- `, `* `, `+ ` variants, verify bullet rendering and vertical layout

**Q2.6**: Do ordered lists render with numbers preserved?
- **Theory**: `markdown_features.md` lines 111-113 - Numbers preserved in text
- **Answer via**: **Toy 4** - Include `1. `, `2. ` in `lists.md`, verify numbering displays correctly

### Blockquotes
**Q2.7**: How do blockquotes currently render?
- **Theory**: `markdown_features.md` lines 122-127 - Currently plain text, no styling
- **Known limitation**: No indentation/visual distinction
- **Answer via**: **Toy 5** - Create `blockquotes.md` with `> ` syntax, document actual rendering behavior

---

## 3. Edge Case Validation

**Theory source**: `learnings/markdown_features.md` - Edge Cases & Gotchas section

### Nested Formatting
**Q3.1**: Does nested bold/italic work in all combinations?
- **Example**: `**bold *and italic* bold**` - does italic properly nest inside bold?
- **Theory**: `markdown_features.md` line 221 - "Bold + italic works (nested events set both flags)"
- **Answer via**: **Toy 6** - Create `edge_cases.md` with nested combinations, verify all variations render correctly

**Q3.2**: Can you have bold code or italic code?
- **Example**: `***code***` with backticks - do code and formatting combine?
- **Answer via**: **Toy 6** - Test formatting + code combinations in `edge_cases.md`

### Table Edge Cases
**Q3.3**: Do tables handle empty cells correctly?
- **Theory**: `markdown_features.md` lines 69-78 - Table rendering documented
- **Unknown**: Empty cell rendering behavior
- **Answer via**: **Toy 7** - Create table with empty cells in `edge_cases.md`, verify grid layout

**Q3.4**: Do tables handle complex alignment combinations?
- **Example**: Left + center + right aligned columns in same table
- **Answer via**: **Toy 7** - Create multi-column table with different alignments per column

### ✅ Answered Questions

**Q3.5**: Do emoji in tables render differently than in body text?
- ✅ **ANSWERED**: Yes, tables use Label instead of EmojiLabel
  - Source: `markdown_features.md` lines 79, 223 - Grid layout constraint
  - Limitation: Loses colored emoji rendering
- **Next step**: Document this limitation in test fixtures with comment

### Error Handling
**Q3.6**: How do images fail gracefully when file missing?
- **Theory**: `markdown_features.md` line 96 - Image loading synchronous from disk
- **Unknown**: Error handling for missing files
- **Answer via**: **Toy 8** - Reference non-existent image in test fixture, observe behavior

### Wrapping Behavior
**Q3.7**: How do very long lines handle wrapping?
- **Theory**: `markdown_features.md` lines 176-192 - Inline batching controls flow
- **Unknown**: Actual pixel width threshold for wrapping
- **Answer via**: **Toy 9** - Create line with 200+ character text, verify horizontal_wrapped behavior

---

## 4. Implementation Gaps

**Theory source**: `learnings/markdown_features.md` - Features Not Implemented section

### ✅ Answered Questions

**Q4.1**: Should we implement horizontal rules?
- ✅ **ANSWERED**: Defer to post-MVP
  - Source: Research assessment - "Defer implementation, focus on testing"
  - Rationale: Low complexity but not blocking, focus on existing feature coverage first
- **Next step**: Document as known limitation in README

**Q4.2**: Should blockquotes get visual styling?
- ✅ **ANSWERED**: Defer to post-MVP
  - Source: Same as Q4.1
  - Alternative: Could add indentation + left border later
- **Next step**: Test current behavior (Q2.7), document limitation

### Decision Questions

**Q4.3**: Should we add clickable links?
- **Pending**: Wait for user feedback on MVP
- **Trade-off**: External dependency on URL handling vs useful feature
- **Current behavior**: Link URLs discarded, only text displayed
- **Answer via**: Discovery phase user testing - does anyone request this feature?

---

## Next Steps to Answer These Questions

### Discovery Phase 1: Create Core Test Fixtures (Answers Q2.1-Q2.7)
1. **Toy 1**: `tests/fixtures/inline_code.md` - Backtick code examples
2. **Toy 2**: `tests/fixtures/headings.md` - All H1-H6 levels + long heading
3. **Toy 3**: `tests/fixtures/images.md` - Standard markdown image syntax
4. **Toy 4**: `tests/fixtures/lists.md` - Unordered + ordered lists
5. **Toy 5**: `tests/fixtures/blockquotes.md` - Quote syntax and current rendering
6. Open each in Mirror, document rendering behavior

### Discovery Phase 2: Edge Case Testing (Answers Q3.1-Q3.7)
7. **Toy 6**: `tests/fixtures/edge_cases.md` - Nested formatting combinations
8. **Toy 7**: Add table edge cases to `tests/fixtures/tables.md` - Empty cells, complex alignment
9. **Toy 8**: Add missing image reference to `images.md` - Error handling
10. **Toy 9**: Add very long line to `edge_cases.md` - Wrapping behavior
11. Document all findings in comments within fixtures

### Discovery Phase 3: Documentation & Review (Answers Q1.3, Q4.3)
12. Evaluate fixture organization effectiveness (Q1.3)
13. Update README.md with test fixture usage guide
14. Document known limitations (horizontal rules, blockquotes, links)
15. Create summary document of all validated behaviors

---

## Cross-Reference Map

**Theory Documents**:
- `learnings/markdown_features.md` - Comprehensive feature catalog
- `learnings/.ddd/0_markdown_features_research.md` - Research insights and decisions
- `RESEARCH_PLAN.md` - Original study priorities

**Practice Outputs** (to be created in Discovery):
- `tests/fixtures/inline_code.md` - Q2.1
- `tests/fixtures/headings.md` - Q2.2, Q2.3
- `tests/fixtures/images.md` - Q2.4, Q3.6
- `tests/fixtures/lists.md` - Q2.5, Q2.6
- `tests/fixtures/blockquotes.md` - Q2.7
- `tests/fixtures/edge_cases.md` - Q3.1, Q3.2, Q3.7, Q3.9

**Existing Fixtures** (to be enhanced):
- `tests/fixtures/tables.md` - Q3.3, Q3.4

---

## Status: READY

All questions categorized and mapped to concrete toy experiments. No blockers identified. Research phase complete, Discovery phase roadmap clear.

**Primary insight**: Questions naturally organize into systematic test fixture creation. Each fixture answers 1-3 related questions.

**Recommended approach**: Create fixtures in priority order (core features → edge cases → documentation), validate in Mirror, document findings inline.
