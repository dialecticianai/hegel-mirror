# ROADMAP.md

**Hegel Mirror**: Ephemeral Markdown review UI for Dialectic-Driven Development.

---

## Vision

Mirror is the human-in-the-loop approval interface for DDD workflows. It bridges the gap between AI-generated artifacts and human oversight, providing a zero-friction review experience that agents can invoke programmatically and humans can use naturally.

**Core purpose**: Markdown preview and review. Launch, read, comment, submit. That's it.

---

## Current Status

**Phase 1: MVP** - Core rendering and interaction complete ✅

We've built a fully functional Markdown viewer with:
- Complete Markdown rendering (text, code blocks with syntax highlighting, tables, images)
- Line-precise text selection with visual highlighting
- Floating comment UI with smart positioning
- Lazy rendering with viewport culling (60fps on 11K+ line documents)
- Theme system for typography and layout
- ~1,500 lines of Rust across 24 modules

**Next:** Review file persistence (write comments to `.ddd/<filename>.review.N`)

---

## Guiding Principles

1. **Ephemerality as feature**: No persistent state. Launch → review → exit. Like `$EDITOR` for git commits.
2. **Agent-first, human-compatible**: Designed for programmatic invocation, delightful for humans.
3. **Zero friction**: Single binary, no install, no config. Just works.
4. **Append-only history**: Never overwrite reviews. Monotonic sequence provides full audit trail.
5. **Local-first**: No network, no cloud, no dependencies. Fully offline.
6. **Cross-platform**: macOS, Linux, Windows. Native performance everywhere.

---

## Phase 1: MVP - Markdown Review

### 1.1 Core UI (Milestone M1: Single-file review) ✅ **COMPLETED**

**Goal:** Launch mirror with single Markdown file, display rendered content, submit review to `.ddd/<filename>.review.1`.

**Implemented:**
- ✅ `egui` window with full Markdown rendering
  - Text paragraphs with wrapping
  - Code blocks with syntax highlighting (via `syntect`)
  - Tables with proper grid layout
  - Images (local file paths)
  - Headers, lists, blockquotes
- ✅ Line-precise text selection with mouse drag
  - Visual highlight bars for selected lines
  - Multi-chunk selection support
  - Hover indicators and click-to-clear
- ✅ Floating comment UI in right margin
  - Aligned with selection
  - Scroll indicators when off-screen
  - Text input field
- ✅ Lazy rendering with cached heights
  - Viewport culling for performance
  - Smooth 60fps scrolling on 11K+ line documents
- ✅ Theme system
  - Typography configuration (fonts, sizes, spacing)
  - Layout controls (margins, max width)
  - Swappable theme definitions
- ✅ CLI argument parsing (`--out-dir`, `--json`, `--headless`)

**Architecture:**
- `src/main.rs` - CLI entry point with `clap`
- `src/app.rs` - Main application struct and update loop
- `src/parsing/` - Markdown parsing into chunks
- `src/rendering/` - Rendering system (text, code, tables, images, comments)
- `src/models/` - Data structures (Chunk, Selection, Comment, LayoutMap)
- `src/syntax/` - Syntax highlighting via `syntect`
- `src/theme/` - Theme system

**Not yet implemented:**
- ❌ Review file persistence (`.ddd/<filename>.review.N`)
- ❌ Auto-exit on submit
- ❌ "Submit Review" button (UI scaffolding exists but no action)

**Testing:**
- Manual testing with various Markdown files
- Performance tested with 11K+ line documents

---

### 1.2 Review persistence (Milestone M2) 🚧 **IN PROGRESS**

**Goal:** Write comments to `.ddd/<filename>.review.N` files with monotonic sequence numbers.

**Features:**
- "Submit Review" button writes comments to disk
- JSONL format: one comment per line
- Monotonic sequence: `.review.1`, `.review.2`, etc.
- Include metadata: timestamp, session ID (from env), file path, selection range
- Auto-exit on submit (or ask user)

**Implementation:**
- `src/storage.rs` - JSONL writing, monotonic sequence logic
- Hook up "Submit Review" button in `src/rendering/comments.rs`
- Add exit logic after successful write

**Testing:**
- Integration test: Verify JSONL format
- Test monotonic sequence (multiple reviews of same file)
- Test session ID passthrough from environment

**Acceptance:**
```bash
export HEGEL_SESSION_ID="abc123"
mirror test.md --out-dir /tmp/reviews
# User selects text, adds comment, clicks submit
# Verify /tmp/reviews/test.review.1 exists and contains valid JSONL with session_id
```

---

### 1.3 Multi-file tabs (Milestone M3)

**Goal:** Support multiple Markdown files as tabs, independent comment queues per file.

**Features:**
- Tab bar at top of window
- Click tab to switch active document
- Each tab shows comment count: `SPEC.md (3)`
- Submit writes separate `.review.N` file per document

**Implementation:**
- `src/ui/tabs.rs` - Tab bar widget, tab switching logic
- Refactor `app.rs` to hold `Vec<Document>` with active index
- Each `Document` has independent comment queue and selection state

**Testing:**
- Integration test: `mirror file1.md file2.md`
- Verify separate `.review.N` files created

**Acceptance:**
```bash
mirror SPEC.md PLAN.md
# User reviews both, adds comments to each
# Submit → verify .ddd/SPEC.review.1 and .ddd/PLAN.review.1
```

---

### 1.4 Immediate vs batched review (Milestone M4)

**Goal:** Two modes - immediate commenting (default) and batched review.

**Features:**
- Default: Every comment saves immediately (append to `.review.N`)
- "Start Review" button: Enter batch mode
  - Comments queued in memory only
  - "Submit Review" button appears
  - Click submit → atomic write all comments → exit
- User can toggle between modes mid-session

**Implementation:**
- `src/review_mode.rs` - Enum: Immediate vs Batched
- Refactor comment handling to check mode before writing
- Add "Start Review" / "Submit Review" buttons conditionally

**Testing:**
- Test immediate: Verify `.review.N` grows on each comment
- Test batched: Verify `.review.N` empty until submit

**Acceptance:**
```bash
mirror SPEC.md
# Immediate: Each comment → immediate write
# Batched: "Start Review" → queue comments → "Submit" → atomic write
```

---

### 1.6 Keyboard shortcuts (Milestone M6)

**Goal:** Power-user navigation.

**Features:**
- Keyboard shortcuts:
  - `Ctrl+Tab` / `Ctrl+Shift+Tab` - Next/previous tab (when multi-file tabs implemented)
  - `Ctrl+Enter` - Submit review
  - `Escape` - Cancel/clear selection
  - `Ctrl+Q` - Quit without submitting (batch mode)

**Implementation:**
- `src/keyboard.rs` - Keyboard event handling
- Hook into egui input handling in `app.rs`

**Testing:**
- Manual testing for keyboard shortcuts

**Acceptance:**
```bash
mirror SPEC.md
# Test Ctrl+Enter, Escape shortcuts
```

---

## Phase 2: Polish and Usability

### 2.1 Enhanced Markdown rendering

**Goal:** Better rendering quality and navigation for large documents.

**Features:**
- Syntax highlighting for code blocks
- Image preview support
- Table of contents navigation
- Anchor links for headers
- Search within document

**Implementation:**
- Leverage existing `egui_markdown` capabilities
- `src/ui/toc.rs` - Table of contents sidebar
- `src/ui/search.rs` - In-document search widget

**Testing:**
- Integration tests with documents containing various Markdown features
- Manual testing for navigation

**Acceptance:**
```bash
mirror large-spec.md
# TOC sidebar shows all headers
# Click header → jump to section
# Ctrl+F → search within document
```

---

### 2.2 Diff view for Markdown revisions

**Goal:** Show diff between two versions of a Markdown document during review.

**Features:**
- `mirror --diff OLD.md NEW.md` mode
- Side-by-side or unified diff view
- Comment on diff hunks (additions/deletions)
- Review file includes diff context in metadata

**Implementation:**
- `src/diff.rs` - Diff computation (use `similar` crate)
- `src/ui/diff_view.rs` - Diff rendering widget
- Extend comment metadata to include diff hunk info

**Testing:**
- Integration tests with known diffs
- Verify comment anchoring to diff hunks

**Acceptance:**
```bash
mirror --diff SPEC.v1.md SPEC.v2.md
# Side-by-side view shows changes
# User comments on specific additions/deletions
# Review file includes diff context
```

---

## Phase 3: Advanced Features

### 3.1 Review templates

**Goal:** Pre-defined comment types with structured metadata.

**Features:**
- Comment types: Question, Suggestion, Blocker, Typo, Praise
- Each type has icon, color, and optional structured fields
- Templates: Load from `.ddd/review_templates.yaml`
- User selects template → structured form appears

**Implementation:**
- `src/review_templates.rs` - Template definitions, YAML loading
- `src/ui/template_picker.rs` - Template selection widget
- Extend comment metadata: `type` field, `structured_data` JSON blob

**Example template:**
```yaml
templates:
  - name: "Question"
    icon: "❓"
    color: "#FFA500"
    fields:
      - name: "severity"
        type: "select"
        options: ["minor", "major", "critical"]
  - name: "Typo"
    icon: "✏️"
    color: "#FF0000"
    fields: []
```

**Testing:**
- Unit tests for template loading
- Integration tests with sample templates

**Acceptance:**
```bash
mirror SPEC.md --templates .ddd/review_templates.yaml
# User selects "Question" template
# Structured form appears with severity dropdown
# Review file includes structured metadata
```

---

### 3.2 Export formats

**Goal:** Export reviews to HTML, PDF, annotated Markdown.

**Features:**
- `mirror --export-html SPEC.md` → `SPEC.review.html`
- `mirror --export-pdf SPEC.md` → `SPEC.review.pdf`
- `mirror --export-annotated SPEC.md` → `SPEC.annotated.md` (inline comments)

**Implementation:**
- `src/export/html.rs` - HTML generation with embedded CSS
- `src/export/pdf.rs` - PDF generation (use `printpdf` or shell to `wkhtmltopdf`)
- `src/export/annotated_md.rs` - Inject comments as blockquotes in Markdown

**Testing:**
- Integration tests for each export format
- Verify output correctness

**Acceptance:**
```bash
mirror --export-html SPEC.md
# Generates SPEC.review.html with comments highlighted
# Open in browser → verify formatting
```

---

### 3.3 Plugin system (stretch goal)

**Goal:** Extensible comment handlers - run linters, check constraints, invoke external tools.

**Features:**
- Plugin hooks: `on_comment_added(comment) -> Result<(), Error>`
- Load plugins from `.ddd/plugins/` directory
- Plugins are WASM modules (cross-platform, sandboxed)
- Built-in plugins: Spell checker, link validator

**Implementation:**
- `src/plugins/mod.rs` - Plugin loader, WASM runtime
- `src/plugins/runtime.rs` - WASM runtime via `wasmtime` crate
- Plugin API: WASM imports/exports for comment data

**Testing:**
- Unit tests for plugin loader
- Integration tests with sample WASM plugins

**Acceptance:**
```bash
mirror SPEC.md --plugins .ddd/plugins/
# User adds comment with typo
# Spell checker plugin runs
# Error message shown: "Spelling error detected"
```

---

## Milestones Summary

| Phase | Milestone | Description | Status |
|-------|-----------|-------------|--------|
| 1 | M1 | Single-file Markdown review | ✅ Complete |
| 1 | M2 | Review persistence (`.review.N` files) | 🚧 In Progress |
| 1 | M3 | Multi-file tabs | ⏳ Planned |
| 1 | M4 | Immediate vs batched review | ⏳ Planned |
| 1 | M5 | JSON output, env integration | ⏳ Planned (partial) |
| 1 | M6 | Keyboard shortcuts | ⏳ Planned |
| 2 | - | Enhanced Markdown rendering | ⏳ Planned |
| 2 | - | Diff view integration | ⏳ Planned |
| 3 | - | Review templates | ⏳ Planned |
| 3 | - | Export formats | ⏳ Planned |
| 3 | - | Plugin system (stretch) | ⏳ Planned |

**Current Focus:** M2 - Review persistence

---

## Success Metrics

**MVP (Phase 1):**
- Single binary, <10MB size
- Launch time: <500ms for documents up to 10k lines
- Cross-platform: macOS, Linux, Windows
- Zero config: `mirror FILE.md` just works
- Smooth scrolling: 60fps on large documents

**Phase 2:**
- Enhanced rendering: TOC, search, image preview
- Diff view: Side-by-side comparison for revisions

**Phase 3:**
- Review templates: Structured comment types
- Export: HTML, PDF, annotated Markdown
- Plugin system: WASM-based extensibility

---

*Ephemeral review UI for Markdown documents. Launch, read, comment, submit. Nothing more.*
