# Lines of Code Report

**Last Updated**: 2025-10-13 22:13
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 1,872 | 3,406 | 5,278 |
| **Comments** | 211 | - | 211 |
| **Blank Lines** | 218 | - | 218 |
| **Total Lines** | 2,301 | 3,406 | 5,707 |
| **Files** | 27 | 24 | 51 |

**Documentation Ratio**: 1.82 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            27            218            211           1872
-------------------------------------------------------------------------------
SUM:                            27            218            211           1872
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `app.rs` | 134 | 134 | 0 | 0.0% | ✅ |
| `lib.rs` | 11 | 11 | 0 | 0.0% | ✅ |
| `main.rs` | 75 | 75 | 0 | 0.0% | ✅ |
| `models/chunk.rs` | 47 | 47 | 0 | 0.0% | ✅ |
| `models/comment.rs` | 34 | 34 | 0 | 0.0% | ✅ |
| `models/layout.rs` | 71 | 71 | 0 | 0.0% | ✅ |
| `models/mod.rs` | 11 | 11 | 0 | 0.0% | ✅ |
| `models/selection.rs` | 50 | 50 | 0 | 0.0% | ✅ |
| `models/table.rs` | 22 | 22 | 0 | 0.0% | ✅ |
| `parsing/chunks.rs` | 197 | 197 | 0 | 0.0% | ✅ |
| `parsing/html.rs` | 162 | 79 | 83 | 51.2% | ✅ |
| `parsing/mod.rs` | 6 | 6 | 0 | 0.0% | ✅ |
| `parsing/parser.rs` | 266 | 266 | 0 | 0.0% | ⚠️ Large |
| `parsing/position.rs` | 52 | 40 | 12 | 23.1% | ✅ |
| `rendering/chunk.rs` | 298 | 298 | 0 | 0.0% | ⚠️ Large |
| `rendering/code.rs` | 39 | 39 | 0 | 0.0% | ✅ |
| `rendering/comments.rs` | 141 | 141 | 0 | 0.0% | ✅ |
| `rendering/helpers.rs` | 159 | 55 | 104 | 65.4% | ✅ |
| `rendering/image.rs` | 80 | 80 | 0 | 0.0% | ✅ |
| `rendering/mod.rs` | 12 | 12 | 0 | 0.0% | ✅ |
| `rendering/table.rs` | 84 | 84 | 0 | 0.0% | ✅ |
| `rendering/text.rs` | 33 | 33 | 0 | 0.0% | ✅ |
| `rendering/ui.rs` | 115 | 115 | 0 | 0.0% | ✅ |
| `syntax/highlighter.rs` | 51 | 51 | 0 | 0.0% | ✅ |
| `syntax/mod.rs` | 3 | 3 | 0 | 0.0% | ✅ |
| `theme/default.rs` | 45 | 45 | 0 | 0.0% | ✅ |
| `theme/mod.rs` | 103 | 103 | 0 | 0.0% | ✅ |

**⚠️ Warning:** 2 file(s) over 200 impl lines - consider splitting for maintainability

---

## Documentation Files

| File | Lines |
|------|-------|
| `CLAUDE.md` | 259 |
| `COVERAGE_REPORT.md` | 96 |
| `DDD.md` | 493 |
| `EMOJIS.md` | 139 |
| `LEXICON.md` | 84 |
| `LOC_REPORT.md` | 116 |
| `README.md` | 99 |
| `ROADMAP.md` | 411 |
| `test_centered_image.md` | 7 |
| `tests/fixtures/basic.md` | 17 |
| `tests/fixtures/tables.md` | 12 |
| `tests/fixtures/unicode.md` | 9 |
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
| Docs/Code Ratio | ≥0.3 | 1.82 | ✅ Excellent |
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
