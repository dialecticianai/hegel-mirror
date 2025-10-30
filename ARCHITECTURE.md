# Mirror Architecture

Tech stack and architectural decisions for ephemeral Markdown review UI.

---

## Technology Stack

**Language**: Rust (Edition 2021)
**Rationale**: Memory safety, zero-cost abstractions, strong type system, native performance, cross-platform compilation.

**Core Dependencies**:
- **egui/eframe 0.33** - Immediate-mode GUI framework
  - Native performance without web dependencies
  - Cross-platform (macOS, Linux, Windows)
  - Simple mental model (no retained widget tree)
- **pulldown-cmark 0.13** - Markdown parsing
  - CommonMark spec compliance
  - Event stream API for incremental processing
  - Stable, widely-used in Rust ecosystem
- **syntect 5** - Syntax highlighting
  - TextMate grammar support
  - Rich language coverage
  - Offline operation
- **clap 4** - CLI argument parsing
  - Derive macros for ergonomic API
  - Comprehensive help generation
- **serde/serde_json 1** - Serialization
  - JSONL format for review files
  - Zero-copy deserialization where possible
- **anyhow 1** - Error handling
  - Context propagation without boilerplate
  - Compatible with std::error::Error

**Vendored Dependencies**:
- **egui-twemoji** (local vendor/) - Colored emoji rendering
  - Twemoji assets for consistent cross-platform emoji display
  - Vendored for version control and offline builds

**Dev Dependencies**:
- **tempfile 3.8** - Test file isolation
- **egui_kittest 0.33** - UI testing utilities

---

## Core Architectural Decisions

### Decision: Immediate-Mode GUI (egui)

**Choice**: egui for all UI rendering
**Rationale**:
- No retained widget tree simplifies state management
- Native performance without browser overhead
- Cross-platform with single codebase
- Minimal dependencies (no Qt, GTK, or Electron)
- Perfect fit for ephemeral, stateless application model

**Tradeoffs**:
- Limited to egui's widget set (no arbitrary HTML/CSS)
- Redraws every frame (acceptable for this workload)
- Testing requires manual validation (limited egui test infrastructure)

**Alternatives considered**:
- **Tauri/web**: Heavier, requires web dependencies
- **iced**: More complex retained architecture
- **GTK/Qt**: Platform-specific builds, larger binaries

### Decision: Trait-Based Rendering System

**Choice**: ChunkRenderer trait with specialized implementations (TextRenderer, CodeRenderer, TableRenderer, ImageRenderer)

**Rationale**:
- Eliminates duplication in viewport culling logic
- Consistent height estimation and caching across chunk types
- Easy to extend with new chunk types
- Clear separation of concerns (rendering vs layout vs selection)

**Pattern**:
```rust
pub trait ChunkRenderer {
    fn estimate_height(&self, chunk: &TextChunk, theme: &Theme) -> f32;
    fn render_visible(&self, ctx: &mut RenderContext) -> (f32, bool);
}
```

**Introduced**: October 2025 refactor (e6aae82)

### Decision: Viewport Culling with Cached Heights

**Choice**: Lazy rendering with per-chunk height caching

**Rationale**:
- 60fps scrolling on 11K+ line documents
- Prevents flicker during scrolling (cached heights provide stable layout)
- Minimal memory overhead (single f32 per chunk)
- Viewport buffer (1000px below visible area) for smooth scrolling

**Tradeoffs**:
- First render slower (must measure all visible chunks)
- Height cache invalidation needed if font/theme changes (currently not supported)

**Performance**: Tested with 11K-line Markdown, maintains 60fps

### Decision: File-Based Review Persistence

**Choice**: JSONL files in `.ddd/<filename>.review.N` with monotonic sequence

**Rationale**:
- Inspectable (users can examine reviews directly)
- Local-first (no network, no database)
- Append-only history (never overwrites previous reviews)
- Git-friendly (plain text, diff-able)
- Monotonic sequence provides audit trail

**Format**:
```jsonl
{"timestamp":"2025-10-14T03:32:15Z","session_id":"abc123","file":"SPEC.md","selection":{"start":{"line":15,"col":0},"end":{"line":18,"col":0}},"text":"snippet","comment":"feedback"}
```

**Tradeoffs**:
- No concurrent access guarantees (single-user assumption)
- No indexing (sequential scan for queries)
- Manual consistency (no transactions)

**Alternatives considered**:
- **SQLite**: More robust but opaque, adds dependency
- **In-memory only**: Loses data on crash
- **Single .review file**: No audit trail, overwrite semantics

### Decision: Dual Review Modes (Immediate vs Batched)

**Choice**: Support both immediate append and batched atomic writes

**Rationale**:
- **Immediate mode**: Incremental review, no data loss risk
- **Batched mode**: Atomic submission for formal reviews, auto-exit on submit
- User choice based on workflow context

**Implementation**: ReviewMode enum + conditional storage calls

### Decision: Embedded Fonts (Inter Family)

**Choice**: Embed Inter Regular/Bold/Italic/BoldItalic in binary

**Rationale**:
- Consistent typography across platforms
- No system font dependencies or fallback complexity
- Professional appearance (Inter is clean, readable)
- Supports bold/italic markdown rendering

**Tradeoffs**:
- Larger binary size (~800KB for 4 font variants)
- No user font customization (acceptable for MVP)

**Alternative**: System fonts would reduce binary size but complicate bold/italic rendering

---

## System Boundaries

**Internal**:
- Markdown parsing (pulldown-cmark events → positioned TextChunks)
- Trait-based rendering with viewport culling
- Selection state management and line mapping
- JSONL serialization for review files
- Theme system (typography, colors, layout)

**External** (integration points):
- **Filesystem**: Read markdown files, write review files to `.ddd/`
- **Environment**: `HEGEL_SESSION_ID` for workflow correlation
- **CLI**: `--out-dir`, `--json`, `--headless` flags for automation
- **OS**: Font rendering, file I/O, window management (via egui/eframe)

**Not in scope**:
- Network operations (fully offline)
- Database connections
- Version control integration (passive .ddd/ files)
- Real-time collaboration

---

## Known Constraints

**Platform**: macOS, Linux, Windows (native compilation per platform)

**Performance**:
- 60fps target for scrolling (achieved via viewport culling)
- <500ms launch time for documents up to 10k lines
- Smooth rendering for 11K+ line documents

**Compatibility**:
- Must work with existing Hegel CLI integration (HEGEL_SESSION_ID env var)
- Review files must be parseable by external tools (JSONL format)
- No destructive operations (append-only review history)

**Security**:
- Local files only (no network access)
- No code execution (markdown rendering only)
- No credential storage

**Memory**:
- Lazy rendering keeps memory bounded
- Texture cache for loaded images (cleared on document close)
- Reasonable for documents up to 50K lines

---

## Testing Strategy

**Current Approach**:
- **Manual testing**: Primary validation method for GUI correctness
- **Integration tests**: `tests/` directory with 4 test files (models, parsing, syntax, UI)
- **Inline unit tests**: Embedded `#[cfg(test)]` in 6 source files (html.rs, position.rs, helpers.rs, inline_batcher.rs, text_builder.rs, viewport.rs)
- **Coverage tracking**: `scripts/generate-coverage-report.sh` generates COVERAGE_REPORT.md

**Test Categories**:
- **Unit tests**: Parsing logic, position tracking, HTML parsing, selection helpers
- **Integration tests**: End-to-end markdown rendering, storage I/O, CLI arg parsing
- **Manual tests**: Visual correctness, emoji rendering, selection UX, performance

**Coverage Target**: No explicit target, but focus on business logic (parsing, storage, position tracking)

**Why manual testing dominates**:
- egui's immediate-mode model limits automated UI testing
- Visual correctness requires human validation
- Performance characteristics (fps, smoothness) need interactive assessment

**Test fixtures**: `tests/fixtures/` with basic.md, tables.md, unicode.md

**Future improvements**:
- Snapshot tests for parsed chunk structures
- Property-based tests for position tracking (quickcheck/proptest)
- Performance benchmarks (criterion)

---

## Module Organization

See [src/CODE_MAP.md](src/CODE_MAP.md) for detailed structure. High-level organization:

**Entry points**:
- `main.rs` - CLI parsing, font loading, eframe initialization
- `lib.rs` - Public exports for testing

**Core modules**:
- `app.rs` - Application state machine, tab management, review mode orchestration
- `storage.rs` - JSONL review file I/O with monotonic sequence
- `models/` - Data structures (TextChunk, Selection, Comment, Document, LayoutMap, ReviewMode, Table)
- `parsing/` - Markdown → TextChunks with position tracking
- `rendering/` - Trait-based rendering system (14 files, largest module)
- `syntax/` - Syntax highlighting via syntect
- `theme/` - Typography and layout configuration

**Rendering subsystem** (trait-based refactor, Oct 2025):
- `chunk_renderer.rs` - ChunkRenderer trait + implementations
- `viewport.rs` - ViewportCuller for lazy rendering
- `selection_manager.rs` - Centralized selection logic
- `inline_batcher.rs` - Horizontal text flow optimization
- `text_builder.rs` - Styled text with emoji support
- `comments.rs` - Floating comment UI

**Design pattern**: Immediate-mode GUI with trait-based extensibility for chunk types

---

## Open Questions

**Phase 2 considerations**:
- [ ] Diff view implementation: Side-by-side or unified diff?
- [ ] Table of contents: Sidebar or inline?
- [ ] Search: In-document or cross-file?
- [ ] Image preview: Lightbox or inline zoom?

**Performance unknowns**:
- [ ] Memory characteristics for very large documents (50K+ lines)?
- [ ] Texture cache eviction strategy for many images?
- [ ] Optimal viewport buffer size (currently 1000px)?

**Refactoring opportunities**:
- [ ] Extract theme system to separate crate for reuse?
- [ ] Consolidate font loading into theme module?
- [ ] Abstract storage layer for alternative backends (SQLite)?

**Phase 3 architectural questions**:
- [ ] Plugin system: WASM-based or dynamic library?
- [ ] Export formats: Generate in-process or shell to external tools?
- [ ] Review templates: YAML schema or Rust types?

---

## Non-Functional Requirements

**Reliability**:
- Append-only writes minimize data loss risk
- Atomic file operations where possible (temp file + rename)
- Graceful degradation (missing fonts, images, etc.)

**Performance**:
- 60fps scrolling (viewport culling)
- Sub-second startup for typical documents
- Minimal memory footprint (lazy rendering)

**Maintainability**:
- File size guideline: ≤200 lines per implementation file
- Clear module boundaries (models/parsing/rendering/syntax/theme/storage)
- Trait-based abstractions for extensibility
- Inline documentation for non-obvious logic

**Testability**:
- Pure functions where possible (parsing, position tracking)
- Integration tests for storage I/O
- Manual testing protocol for GUI (documented in test process)

**Portability**:
- Cross-platform via Rust + egui
- No platform-specific code except egui/eframe abstractions
- Embedded fonts for consistent typography

---

## Build & Distribution

**Build**:
```bash
cargo build --release
./scripts/build-and-install.sh         # Build + version bump + install
./scripts/build-and-install.sh --skip-bump  # Build + install (no version change)
```

**Installation**: Binary installed to `~/.cargo/bin/mirror`

**Release targets**:
- `x86_64-apple-darwin` (Intel Mac)
- `aarch64-apple-darwin` (Apple Silicon)
- `x86_64-unknown-linux-gnu` (Linux)
- `x86_64-pc-windows-msvc` (Windows)

**Binary size**: ~8-10MB (includes fonts, syntect themes, egui)

**Distribution**: GitHub releases with pre-compiled binaries

---

## Future Architectural Evolution

**Phase 2** (Polish):
- Enhanced markdown features (TOC, search, image preview)
- Diff view subsystem (new module: `src/diff/`)
- Keyboard shortcuts system

**Phase 3** (Advanced):
- Review template system (YAML → structured metadata)
- Export subsystem (HTML, PDF, annotated Markdown)
- Plugin architecture (WASM-based extensibility)

**Constraints to preserve**:
- Zero-friction launch (no config files)
- Local-first (no network)
- Single binary distribution
- Ephemeral state model (no persistent app state)

---

**Last updated**: 2025-10-29
**Status**: Phase 1 MVP complete, Phase 2 planning

See [ROADMAP.md](ROADMAP.md) for feature timeline and [src/CODE_MAP.md](src/CODE_MAP.md) for implementation details.
