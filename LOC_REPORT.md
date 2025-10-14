# Lines of Code Report

**Last Updated**: 2025-10-13 23:43
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 2,232 | 1,790 | 4,022 |
| **Comments** | 239 | - | 239 |
| **Blank Lines** | 262 | - | 262 |
| **Total Lines** | 2,733 | 1,790 | 4,523 |
| **Files** | 30 | 14 | 44 |

**Documentation Ratio**: 0.80 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            30            262            239           2232
-------------------------------------------------------------------------------
SUM:                            30            262            239           2232
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `app.rs` | 225 | 225 | 0 | 0.0% | ⚠️ Large |
| `lib.rs` | 13 | 13 | 0 | 0.0% | ✅ |
| `main.rs` | 96 | 96 | 0 | 0.0% | ✅ |
| `models/chunk.rs` | 47 | 47 | 0 | 0.0% | ✅ |
| `models/comment.rs` | 34 | 34 | 0 | 0.0% | ✅ |
| `models/document.rs` | 49 | 49 | 0 | 0.0% | ✅ |
| `models/layout.rs` | 71 | 71 | 0 | 0.0% | ✅ |
| `models/mod.rs` | 15 | 15 | 0 | 0.0% | ✅ |
| `models/review_mode.rs` | 14 | 14 | 0 | 0.0% | ✅ |
| `models/selection.rs` | 50 | 50 | 0 | 0.0% | ✅ |
| `models/table.rs` | 22 | 22 | 0 | 0.0% | ✅ |
| `parsing/chunks.rs` | 197 | 197 | 0 | 0.0% | ✅ |
| `parsing/html.rs` | 162 | 79 | 83 | 51.2% | ✅ |
| `parsing/mod.rs` | 6 | 6 | 0 | 0.0% | ✅ |
| `parsing/parser.rs` | 266 | 266 | 0 | 0.0% | ⚠️ Large |
| `parsing/position.rs` | 52 | 40 | 12 | 23.1% | ✅ |
| `rendering/chunk.rs` | 298 | 298 | 0 | 0.0% | ⚠️ Large |
| `rendering/code.rs` | 39 | 39 | 0 | 0.0% | ✅ |
| `rendering/comments.rs` | 201 | 201 | 0 | 0.0% | ⚠️ Large |
| `rendering/helpers.rs` | 159 | 55 | 104 | 65.4% | ✅ |
| `rendering/image.rs` | 80 | 80 | 0 | 0.0% | ✅ |
| `rendering/mod.rs` | 12 | 12 | 0 | 0.0% | ✅ |
| `rendering/table.rs` | 84 | 84 | 0 | 0.0% | ✅ |
| `rendering/text.rs` | 33 | 33 | 0 | 0.0% | ✅ |
| `rendering/ui.rs` | 115 | 115 | 0 | 0.0% | ✅ |
| `storage.rs` | 191 | 191 | 0 | 0.0% | ✅ |
| `syntax/highlighter.rs` | 51 | 51 | 0 | 0.0% | ✅ |
| `syntax/mod.rs` | 3 | 3 | 0 | 0.0% | ✅ |
| `theme/default.rs` | 45 | 45 | 0 | 0.0% | ✅ |
| `theme/mod.rs` | 103 | 103 | 0 | 0.0% | ✅ |

**⚠️ Warning:** 4 file(s) over 200 impl lines - consider splitting for maintainability

---

## Documentation Files

| File | Lines |
|------|-------|
| `CLAUDE.md` | 259 |
| `COVERAGE_REPORT.md` | 102 |
| `DDD.md` | 493 |
| `EMOJIS.md` | 139 |
| `LEXICON.md` | 84 |
| `LOC_REPORT.md` | 113 |
| `README.md` | 99 |
| `ROADMAP.md` | 416 |
| `test_centered_image.md` | 7 |
| `test_review.md` | 20 |
| `test_review2.md` | 20 |
| `tests/fixtures/basic.md` | 17 |
| `tests/fixtures/tables.md` | 12 |
| `tests/fixtures/unicode.md` | 9 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 0.80 | ✅ Excellent |
| README exists | Yes | ✅ | Met |
| ARCHITECTURE.md | Optional | ❌ | Optional |

---

## How to Update This Report

```bash
# Regenerate LOC report
./scripts/generate-loc-report.sh
```

---

*This report is auto-generated from `cloc` and `wc` output.*
*Updated automatically by pre-commit hook when source files change.*
