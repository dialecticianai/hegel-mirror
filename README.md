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

**Part of the Hegel ecosystem:** Mirror is the review UI companion to [Hegel CLI](https://github.com/dialecticianai/hegel-cli), providing human-in-the-loop approval for AI-generated artifacts in Dialectic-Driven Development workflows.

---

## Features

### Markdown Rendering
- Full Markdown support: text, headers, lists, blockquotes
- **Bold and italic text** with proper font rendering (Inter font family)
- **Colored emoji support** (via Twemoji assets)
- Syntax-highlighted code blocks (via `syntect`)
- Tables with proper grid layout
- Images (local file paths)
- Lazy rendering with viewport culling (60fps on 11K+ line documents)

### Review Workflow
- **Line-precise text selection** - Click and drag to select text
- **Floating comment UI** - Smart positioning with scroll indicators
- **Multi-file tabs** - Review multiple documents simultaneously
- **Independent comment queues** - Each file has its own review state
- **Dual review modes**:
  - **Immediate mode** (default) - Each comment saves instantly
  - **Batched mode** - Queue comments, submit atomically

### Review Persistence
- **Dual-mode storage routing**:
  - **Hegel projects** (auto-detected via `.hegel/` directory) → project-global `.hegel/reviews.json`
  - **Standalone mode** → per-file sidecar `.review.N` files
- JSONL format with monotonic sequence numbers (never overwrites previous reviews)
- Full metadata: timestamp, session ID, file, selection range, text snippet
- Hegel mode: relative paths from project root, multiple reviews per file in single JSON map
- Standalone mode: separate review files per document in same directory as reviewed file

### Integration
- CLI: `--out-dir` (standalone mode only), `--json`, `--headless`
- Environment: `HEGEL_SESSION_ID` passthrough
- Exit codes: 0 (success), 1 (error), 2 (cancelled)
- Hegel project detection: automatic via `.hegel/` directory lookup

See [ROADMAP.md](ROADMAP.md) for future enhancements.

---

## Quick Start

```bash
# Build
cargo build --release

# Review single file
./target/release/mirror SPEC.md

# Review multiple files
./target/release/mirror SPEC.md PLAN.md

# With session tracking
export HEGEL_SESSION_ID="session-123"
./target/release/mirror SPEC.md --out-dir .reviews/

# Standalone mode: reviews written to .reviews/SPEC.review.1
# Hegel mode: reviews written to .hegel/reviews.json (project-global)
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

Server Side Public License v1 (SSPL)
