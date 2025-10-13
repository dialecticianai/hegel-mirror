# Lines of Code Report

**Last Updated**: 2025-10-13 17:20
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 806 | 3,555 | 4,361 |
| **Comments** | 59 | - | 59 |
| **Blank Lines** | 79 | - | 79 |
| **Total Lines** | 944 | 3,555 | 4,499 |
| **Files** | 18 | 19 | 37 |

**Documentation Ratio**: 4.41 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            18             79             59            806
-------------------------------------------------------------------------------
SUM:                            18             79             59            806
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `app.rs` | 66 | 66 | 0 | 0.0% | ✅ |
| `main.rs` | 75 | 75 | 0 | 0.0% | ✅ |
| `models/chunk.rs` | 37 | 37 | 0 | 0.0% | ✅ |
| `models/comment.rs` | 34 | 34 | 0 | 0.0% | ✅ |
| `models/mod.rs` | 7 | 7 | 0 | 0.0% | ✅ |
| `models/selection.rs` | 27 | 27 | 0 | 0.0% | ✅ |
| `parsing/mod.rs` | 4 | 4 | 0 | 0.0% | ✅ |
| `parsing/parser.rs` | 267 | 267 | 0 | 0.0% | ⚠️ Large |
| `parsing/position.rs` | 32 | 20 | 12 | 37.5% | ✅ |
| `rendering/code.rs` | 39 | 39 | 0 | 0.0% | ✅ |
| `rendering/image.rs` | 40 | 40 | 0 | 0.0% | ✅ |
| `rendering/mod.rs` | 6 | 6 | 0 | 0.0% | ✅ |
| `rendering/text.rs` | 31 | 31 | 0 | 0.0% | ✅ |
| `rendering/ui.rs` | 134 | 134 | 0 | 0.0% | ✅ |
| `syntax/highlighter.rs` | 51 | 51 | 0 | 0.0% | ✅ |
| `syntax/mod.rs` | 3 | 3 | 0 | 0.0% | ✅ |
| `theme/default.rs` | 28 | 28 | 0 | 0.0% | ✅ |
| `theme/mod.rs` | 63 | 63 | 0 | 0.0% | ✅ |

**⚠️ Warning:** 1 file(s) over 200 impl lines - consider splitting for maintainability

---

## Documentation Files

| File | Lines |
|------|-------|
| `CLAUDE.md` | 259 |
| `COVERAGE_REPORT.md` | 82 |
| `DDD.md` | 543 |
| `LEXICON.md` | 84 |
| `LOC_REPORT.md` | 106 |
| `README.md` | 79 |
| `ROADMAP.md` | 738 |
| `toys/toy1_markdown_render/DEP_REVIEW.md` | 72 |
| `toys/toy1_markdown_render/LEARNINGS.md` | 90 |
| `toys/toy1_markdown_render/README.md` | 30 |
| `toys/toy1_markdown_render/SPEC.md` | 85 |
| `toys/toy1_markdown_render/target/doc/static.files/SourceSerif4-LICENSE-a2cfd9d5.md` | 98 |
| `toys/toy1_markdown_render/test.md` | 32 |
| `toys/toy2_md_render/LEARNINGS.md` | 500 |
| `toys/toy2_md_render/README.md` | 239 |
| `toys/toy2_md_render/target/doc/static.files/SourceSerif4-LICENSE-a2cfd9d5.md` | 98 |
| `toys/toy2_md_render/test.md` | 97 |
| `toys/toy3_floem/LEARNINGS.md` | 225 |
| `toys/toy3_floem/target/doc/static.files/SourceSerif4-LICENSE-a2cfd9d5.md` | 98 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 4.41 | ✅ Excellent |
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
