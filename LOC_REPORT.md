# Lines of Code Report

**Last Updated**: 2025-10-13 19:41
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 1,551 | 3,715 | 5,266 |
| **Comments** | 164 | - | 164 |
| **Blank Lines** | 164 | - | 164 |
| **Total Lines** | 1,879 | 3,715 | 5,594 |
| **Files** | 24 | 20 | 44 |

**Documentation Ratio**: 2.40 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            24            164            164           1551
-------------------------------------------------------------------------------
SUM:                            24            164            164           1551
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `app.rs` | 134 | 134 | 0 | 0.0% | ✅ |
| `main.rs` | 75 | 75 | 0 | 0.0% | ✅ |
| `models/chunk.rs` | 44 | 44 | 0 | 0.0% | ✅ |
| `models/comment.rs` | 34 | 34 | 0 | 0.0% | ✅ |
| `models/layout.rs` | 71 | 71 | 0 | 0.0% | ✅ |
| `models/mod.rs` | 11 | 11 | 0 | 0.0% | ✅ |
| `models/selection.rs` | 49 | 49 | 0 | 0.0% | ✅ |
| `models/table.rs` | 22 | 22 | 0 | 0.0% | ✅ |
| `parsing/chunks.rs` | 164 | 164 | 0 | 0.0% | ✅ |
| `parsing/mod.rs` | 5 | 5 | 0 | 0.0% | ✅ |
| `parsing/parser.rs` | 231 | 231 | 0 | 0.0% | ⚠️ Large |
| `parsing/position.rs` | 51 | 39 | 12 | 23.5% | ✅ |
| `rendering/chunk.rs` | 319 | 319 | 0 | 0.0% | ⚠️ Large |
| `rendering/code.rs` | 39 | 39 | 0 | 0.0% | ✅ |
| `rendering/comments.rs` | 137 | 137 | 0 | 0.0% | ✅ |
| `rendering/image.rs` | 40 | 40 | 0 | 0.0% | ✅ |
| `rendering/mod.rs` | 10 | 10 | 0 | 0.0% | ✅ |
| `rendering/table.rs` | 84 | 84 | 0 | 0.0% | ✅ |
| `rendering/text.rs` | 33 | 33 | 0 | 0.0% | ✅ |
| `rendering/ui.rs` | 119 | 119 | 0 | 0.0% | ✅ |
| `syntax/highlighter.rs` | 51 | 51 | 0 | 0.0% | ✅ |
| `syntax/mod.rs` | 3 | 3 | 0 | 0.0% | ✅ |
| `theme/default.rs` | 48 | 48 | 0 | 0.0% | ✅ |
| `theme/mod.rs` | 105 | 105 | 0 | 0.0% | ✅ |

**⚠️ Warning:** 2 file(s) over 200 impl lines - consider splitting for maintainability

---

## Documentation Files

| File | Lines |
|------|-------|
| `CLAUDE.md` | 259 |
| `COVERAGE_REPORT.md` | 94 |
| `DDD.md` | 543 |
| `EMOJIS.md` | 139 |
| `LEXICON.md` | 84 |
| `LOC_REPORT.md` | 115 |
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
| Docs/Code Ratio | ≥0.3 | 2.40 | ✅ Excellent |
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
