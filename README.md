# Mirror

**Ephemeral Markdown review UI for Dialectic-Driven Development**

[![Built with DDD](https://img.shields.io/badge/built_with-DDD-blue)](https://github.com/dialecticianai/ddd-book/)

---

## What is Mirror?

Mirror is a zero-friction GUI for reviewing Markdown documents (and eventually other structured artifacts) in human-AI collaborative development workflows. Designed for programmatic invocation by agents and natural use by humans.

**Core concept:** Launch → review → comment → exit. No persistent state, no configuration, just pure review flow.

---

## Status

**Phase:** 0 (Scaffolding) - MVP not yet implemented

See [ROADMAP.md](ROADMAP.md) for development plan.

---

## Quick Start (Placeholder)

```bash
# Install
cargo install --path .

# Run (placeholder - not yet functional)
mirror SPEC.md PLAN.md
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

- [ROADMAP.md](ROADMAP.md) - 5-phase development plan (MVP → maximal vision)
- [CLAUDE.md](CLAUDE.md) - Development context and patterns
- [DDD.md](DDD.md) - Dialectic-Driven Development methodology (toy-focused)
- [COVERAGE_REPORT.md](COVERAGE_REPORT.md) - Test coverage metrics
- [LOC_REPORT.md](LOC_REPORT.md) - Lines of code breakdown

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
