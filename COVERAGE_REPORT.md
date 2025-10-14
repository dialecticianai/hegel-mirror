# Test Coverage Report

**Last Updated**: 2025-10-14 00:09
**Tool**: cargo-llvm-cov
**Overall Coverage**: **40.61%** lines | **36.92%** regions | **46.03%** functions

## Summary

```
TOTAL                            2400              1514    36.92%         126                68    46.03%        1795              1066    40.61%           0                 0         -
```

## Coverage by Module

| Module | Line Coverage | Region Coverage | Functions | Status |
|--------|--------------|-----------------|-----------|--------|
| `app.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `main.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `models/comment.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |
| `models/document.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `models/layout.rs` | 93.02% | 95.18% | 100.00% | 🟢 Excellent |
| `models/review_mode.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `models/selection.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |
| `models/table.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |
| `parsing/chunks.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |
| `parsing/html.rs` | 98.95% | 97.42% | 100.00% | 🟢 Excellent |
| `parsing/parser.rs` | 97.64% | 92.92% | 100.00% | 🟢 Excellent |
| `parsing/position.rs` | 100.00% | 98.11% | 100.00% | 🟢 Excellent |
| `rendering/chunk.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `rendering/code.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `rendering/comments.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `rendering/helpers.rs` | 87.61% | 79.17% | 66.67% | 🟡 Good |
| `rendering/image.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `rendering/table.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `rendering/text.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `rendering/ui.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `storage.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `syntax/highlighter.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |
| `theme/mod.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |

## Coverage Tiers

### 🟢 Excellent (≥90% lines)
- `models/comment.rs` - 100.00%
- `models/layout.rs` - 93.02%
- `models/selection.rs` - 100.00%
- `models/table.rs` - 100.00%
- `parsing/chunks.rs` - 100.00%
- `parsing/html.rs` - 98.95%
- `parsing/parser.rs` - 97.64%
- `parsing/position.rs` - 100.00%
- `syntax/highlighter.rs` - 100.00%
- `theme/mod.rs` - 100.00%

### 🟡 Good (70-89% lines)
- `rendering/helpers.rs` - 87.61%

### 🟠 Moderate (40-69% lines)

### 🔴 Needs Work (<40% lines)
- `app.rs` - 0.00%
- `main.rs` - 0.00%
- `models/document.rs` - 0.00%
- `models/review_mode.rs` - 0.00%
- `rendering/chunk.rs` - 0.00%
- `rendering/code.rs` - 0.00%
- `rendering/comments.rs` - 0.00%
- `rendering/image.rs` - 0.00%
- `rendering/table.rs` - 0.00%
- `rendering/text.rs` - 0.00%
- `rendering/ui.rs` - 0.00%
- `storage.rs` - 0.00%

## Coverage Targets

| Tier | Target | Current | Status |
|------|--------|---------|--------|
| Overall | ≥80% | 40.61% | ⏳ In Progress |
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
