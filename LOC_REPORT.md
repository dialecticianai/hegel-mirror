# Lines of Code Report

**Last Updated**: 2025-10-29 21:14
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 2,803 | 5,001 | 7,804 |
| **Comments** | 356 | - | 356 |
| **Blank Lines** | 386 | - | 386 |
| **Total Lines** | 3,545 | 5,001 | 8,546 |
| **Files** | 36 | 37 | 73 |

**Documentation Ratio**: 1.78 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            36            386            356           2803
Markdown                         6             64              0            109
-------------------------------------------------------------------------------
SUM:                            42            450            356           2912
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `app.rs` | 306 | 306 | 0 | 0.0% | ⚠️ Large |
| `image_manager.rs` | 207 | 105 | 102 | 49.3% | ✅ |
| `lib.rs` | 14 | 14 | 0 | 0.0% | ✅ |
| `main.rs` | 152 | 152 | 0 | 0.0% | ✅ |
| `models/chunk.rs` | 49 | 49 | 0 | 0.0% | ✅ |
| `models/comment.rs` | 34 | 34 | 0 | 0.0% | ✅ |
| `models/document.rs` | 51 | 51 | 0 | 0.0% | ✅ |
| `models/layout.rs` | 71 | 71 | 0 | 0.0% | ✅ |
| `models/mod.rs` | 15 | 15 | 0 | 0.0% | ✅ |
| `models/review_mode.rs` | 14 | 14 | 0 | 0.0% | ✅ |
| `models/selection.rs` | 50 | 50 | 0 | 0.0% | ✅ |
| `models/table.rs` | 22 | 22 | 0 | 0.0% | ✅ |
| `parsing/chunks.rs` | 225 | 225 | 0 | 0.0% | ⚠️ Large |
| `parsing/html.rs` | 162 | 79 | 83 | 51.2% | ✅ |
| `parsing/mod.rs` | 6 | 6 | 0 | 0.0% | ✅ |
| `parsing/parser.rs` | 271 | 271 | 0 | 0.0% | ⚠️ Large |
| `parsing/position.rs` | 52 | 40 | 12 | 23.1% | ✅ |
| `rendering/chunk_renderer.rs` | 218 | 218 | 0 | 0.0% | ⚠️ Large |
| `rendering/chunk.rs` | 50 | 50 | 0 | 0.0% | ✅ |
| `rendering/code.rs` | 44 | 44 | 0 | 0.0% | ✅ |
| `rendering/comments.rs` | 181 | 181 | 0 | 0.0% | ✅ |
| `rendering/helpers.rs` | 45 | 19 | 26 | 57.8% | ✅ |
| `rendering/image.rs` | 80 | 80 | 0 | 0.0% | ✅ |
| `rendering/inline_batcher.rs` | 190 | 75 | 115 | 60.5% | ✅ |
| `rendering/mod.rs` | 16 | 16 | 0 | 0.0% | ✅ |
| `rendering/selection_manager.rs` | 117 | 117 | 0 | 0.0% | ✅ |
| `rendering/table.rs` | 93 | 93 | 0 | 0.0% | ✅ |
| `rendering/text_builder.rs` | 153 | 106 | 47 | 30.7% | ✅ |
| `rendering/text.rs` | 33 | 33 | 0 | 0.0% | ✅ |
| `rendering/ui.rs` | 123 | 123 | 0 | 0.0% | ✅ |
| `rendering/viewport.rs` | 125 | 74 | 51 | 40.8% | ✅ |
| `storage.rs` | 174 | 174 | 0 | 0.0% | ✅ |
| `syntax/highlighter.rs` | 51 | 51 | 0 | 0.0% | ✅ |
| `syntax/mod.rs` | 3 | 3 | 0 | 0.0% | ✅ |
| `theme/default.rs` | 45 | 45 | 0 | 0.0% | ✅ |
| `theme/mod.rs` | 103 | 103 | 0 | 0.0% | ✅ |

**⚠️ Warning:** 4 file(s) over 200 impl lines - consider splitting for maintainability

---

## Documentation Files

| File | Lines |
|------|-------|
| `.ddd/feat/image_manager/PLAN.md` | 420 |
| `.ddd/feat/image_manager/SPEC.md` | 303 |
| `ARCHITECTURE.md` | 367 |
| `CLAUDE.md` | 277 |
| `COVERAGE_REPORT.md` | 56 |
| `DDD.md` | 493 |
| `EMOJIS.md` | 139 |
| `HEGEL.md` | 287 |
| `LAZY_RENDERING_ANALYSIS.md` | 345 |
| `learnings/.ddd/0_markdown_features_research.md` | 214 |
| `learnings/.ddd/1_open_questions.md` | 216 |
| `learnings/markdown_features.md` | 277 |
| `LEXICON.md` | 84 |
| `LOC_REPORT.md` | 141 |
| `README.md` | 119 |
| `RESEARCH_PLAN.md` | 139 |
| `ROADMAP.md` | 427 |
| `src/CODE_MAP.md` | 34 |
| `src/models/CODE_MAP.md` | 31 |
| `src/parsing/CODE_MAP.md` | 26 |
| `src/rendering/CODE_MAP.md` | 56 |
| `src/syntax/CODE_MAP.md` | 13 |
| `src/theme/CODE_MAP.md` | 13 |
| `test_bold.md` | 11 |
| `test_centered_image.md` | 7 |
| `test_review.md` | 20 |
| `test_review2.md` | 20 |
| `tests/fixtures/basic.md` | 17 |
| `tests/fixtures/blockquotes.md` | 58 |
| `tests/fixtures/edge_cases.md` | 109 |
| `tests/fixtures/headings.md` | 50 |
| `tests/fixtures/images.md` | 59 |
| `tests/fixtures/inline_code.md` | 39 |
| `tests/fixtures/lists.md` | 71 |
| `tests/fixtures/tables.md` | 12 |
| `tests/fixtures/unicode.md` | 9 |
| `vendor/egui-twemoji/README.md` | 42 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 1.78 | ✅ Excellent |
| README exists | Yes | ✅ | Met |
| ARCHITECTURE.md | Optional | ✅ | Optional |

---

## How to Update This Report

```bash
# Regenerate LOC report
./scripts/generate-loc-report.sh
```

---

*This report is auto-generated from `cloc` and `wc` output.*
*Updated automatically by pre-commit hook when source files change.*
