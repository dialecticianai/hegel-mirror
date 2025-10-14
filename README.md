<p align="center">
  <img src="logo.png" alt="Mirror Logo" width="200">
</p>

# Mirror

**Ephemeral Markdown review UI for Dialectic-Driven Development**

[![Built with DDD](https://img.shields.io/badge/built_with-DDD-blue)](https://github.com/dialecticianai/ddd-book/)

---

## What is Mirror?

Mirror is a zero-friction GUI for reviewing Markdown documents in human-AI collaborative development workflows. Designed for programmatic invocation by agents and natural use by humans.

**Core concept:** Launch → review → comment → exit. No persistent state, no configuration, just pure review flow.

---

## Status

**Phase:** 1 (MVP) - Core Markdown review implemented

**Completed:**
- ✅ M1: Single-file Markdown review
  - Markdown parsing and rendering (text, code blocks, tables, images)
  - Line-precise text selection with visual highlighting
  - Floating comment UI with scroll indicators
  - Syntax highlighting for code blocks
  - Lazy rendering with cached heights (60fps on 11K+ line documents)
  - Theme system for typography and styling
  - CLI argument parsing (`--out-dir`, `--json`, `--headless`)

**In Progress:**
- M2: Multi-file tabs
- M3: Comment persistence (write to `.ddd/<filename>.review.N`)

See [ROADMAP.md](ROADMAP.md) for full development plan.

---

## Quick Start

```bash
# Build
cargo build --release

# Run
./target/release/mirror SPEC.md

# Select text by clicking and dragging
# Add comments in the floating UI
# (Comment persistence coming soon)
```

---

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Generate coverage report
./scripts/generate-coverage-report.sh

# Generate LOC report
./scripts/generate-loc-report.sh
```

---

## Documentation

- [ROADMAP.md](ROADMAP.md) - 3-phase development plan (MVP → advanced features)
- [CLAUDE.md](CLAUDE.md) - Development context and patterns
- [DDD.md](DDD.md) - Dialectic-Driven Development methodology (toy-focused)

---

## Philosophy

Mirror embodies three principles:

1. **Ephemerality as feature** - No persistent state. Launch → review → exit.
2. **Agent-first, human-compatible** - Designed for `hegel reflect` invocation, delightful for humans.
3. **Zero friction** - Single binary, no install, no config. Just works.

See [CLAUDE.md](CLAUDE.md) for full philosophy.

---

## License

MIT
