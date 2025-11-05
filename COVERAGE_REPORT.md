# Test Coverage Report

**Last Updated**: 2025-11-05 14:14
**Tool**: cargo-llvm-cov
**Overall Coverage**: **42.89%** lines | **41.88%** regions | **44.67%** functions

## Summary

```
TOTAL                                     3238              1882    41.88%         197               109    44.67%        2301              1314    42.89%           0                 0         -
```

## Coverage by Module

| Module | Line Coverage | Region Coverage | Functions | Status |
|--------|--------------|-----------------|-----------|--------|
| `src/app.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/image_manager.rs` | 80.53% | 78.45% | 90.91% | 🟡 Good |
| `src/main.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/models/comment.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |
| `src/models/document.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/models/layout.rs` | 93.02% | 95.18% | 100.00% | 🟢 Excellent |
| `src/models/review_mode.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/models/selection.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |
| `src/models/table.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |
| `src/parsing/chunks.rs` | 97.81% | 95.12% | 100.00% | 🟢 Excellent |
| `src/parsing/html.rs` | 98.95% | 97.42% | 100.00% | 🟢 Excellent |
| `src/parsing/parser.rs` | 97.72% | 92.62% | 100.00% | 🟢 Excellent |
| `src/parsing/position.rs` | 100.00% | 98.11% | 100.00% | 🟢 Excellent |
| `src/rendering/chunk_renderer.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/rendering/chunk.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/rendering/code.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/rendering/comments.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/rendering/helpers.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |
| `src/rendering/image.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/rendering/inline_batcher.rs` | 99.19% | 99.48% | 100.00% | 🟢 Excellent |
| `src/rendering/selection_manager.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/rendering/table.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/rendering/text_builder.rs` | 73.81% | 76.80% | 83.33% | 🟡 Good |
| `src/rendering/text.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/rendering/ui.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/rendering/viewport.rs` | 57.89% | 63.10% | 60.00% | 🟠 Moderate |
| `src/storage.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `src/syntax/highlighter.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |
| `src/theme/mod.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |

## Coverage Tiers

### 🟢 Excellent (≥90% lines)
- `src/models/comment.rs` - 100.00%
- `src/models/layout.rs` - 93.02%
- `src/models/selection.rs` - 100.00%
- `src/models/table.rs` - 100.00%
- `src/parsing/chunks.rs` - 97.81%
- `src/parsing/html.rs` - 98.95%
- `src/parsing/parser.rs` - 97.72%
- `src/parsing/position.rs` - 100.00%
- `src/rendering/helpers.rs` - 100.00%
- `src/rendering/inline_batcher.rs` - 99.19%
- `src/syntax/highlighter.rs` - 100.00%
- `src/theme/mod.rs` - 100.00%

### 🟡 Good (70-89% lines)
- `src/image_manager.rs` - 80.53%
- `src/rendering/text_builder.rs` - 73.81%

### 🟠 Moderate (40-69% lines)
- `src/rendering/viewport.rs` - 57.89%

### 🔴 Needs Work (<40% lines)
- `src/app.rs` - 0.00%
- `src/main.rs` - 0.00%
- `src/models/document.rs` - 0.00%
- `src/models/review_mode.rs` - 0.00%
- `src/rendering/chunk.rs` - 0.00%
- `src/rendering/chunk_renderer.rs` - 0.00%
- `src/rendering/code.rs` - 0.00%
- `src/rendering/comments.rs` - 0.00%
- `src/rendering/image.rs` - 0.00%
- `src/rendering/selection_manager.rs` - 0.00%
- `src/rendering/table.rs` - 0.00%
- `src/rendering/text.rs` - 0.00%
- `src/rendering/ui.rs` - 0.00%
- `src/storage.rs` - 0.00%

## Coverage Targets

| Tier | Target | Current | Status |
|------|--------|---------|--------|
| Overall | ≥80% | 42.89% | ⏳ In Progress |
| Critical Paths | ≥95% | Check modules above | Policy |
| New Modules | ≥80% | - | Policy |

## How to Update This Report

```bash
# Regenerate coverage report
./scripts/generate-coverage-report.sh
```

## Quick Commands

```bash
# Run tests with coverage
cargo llvm-cov --html      # Detailed HTML
cargo llvm-cov --summary-only  # Terminal summary

# Update this markdown report
./scripts/generate-coverage-report.sh
```

---

*This report is auto-generated from `cargo llvm-cov` output.*
