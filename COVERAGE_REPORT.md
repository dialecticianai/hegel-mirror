# Test Coverage Report

**Last Updated**: 2025-10-13 18:57
**Tool**: cargo-llvm-cov
**Overall Coverage**: **2.35%** lines | **2.46%** regions | **3.12%** functions

## Summary

```
TOTAL                            1220              1190     2.46%          64                62     3.12%         895               874     2.35%           0                 0         -
```

## Coverage by Module

| Module | Line Coverage | Region Coverage | Functions | Status |
|--------|--------------|-----------------|-----------|--------|
| `app.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `main.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `models/chunk.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `models/comment.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `models/layout.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `models/selection.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `models/table.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `parsing/parser.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `parsing/position.rs` | 100.00% | 100.00% | 100.00% | 🟢 Excellent |
| `rendering/code.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `rendering/image.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `rendering/table.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `rendering/text.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `rendering/ui.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `syntax/highlighter.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |
| `theme/mod.rs` | 0.00% | 0.00% | 0.00% | 🔴 Needs Work |

## Coverage Tiers

### 🟢 Excellent (≥90% lines)
- `parsing/position.rs` - 100.00%

### 🟡 Good (70-89% lines)

### 🟠 Moderate (40-69% lines)

### 🔴 Needs Work (<40% lines)
- `app.rs` - 0.00%
- `main.rs` - 0.00%
- `models/chunk.rs` - 0.00%
- `models/comment.rs` - 0.00%
- `models/layout.rs` - 0.00%
- `models/selection.rs` - 0.00%
- `models/table.rs` - 0.00%
- `parsing/parser.rs` - 0.00%
- `rendering/code.rs` - 0.00%
- `rendering/image.rs` - 0.00%
- `rendering/table.rs` - 0.00%
- `rendering/text.rs` - 0.00%
- `rendering/ui.rs` - 0.00%
- `syntax/highlighter.rs` - 0.00%
- `theme/mod.rs` - 0.00%

## Coverage Targets

| Tier | Target | Current | Status |
|------|--------|---------|--------|
| Overall | ≥80% | 2.35% | ⏳ In Progress |
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
