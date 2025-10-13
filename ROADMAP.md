# ROADMAP.md

**Hegel Mirror**: Ephemeral Markdown review UI for Dialectic-Driven Development.

---

## Vision

Mirror is the human-in-the-loop approval interface for DDD workflows. It bridges the gap between AI-generated artifacts and human oversight, providing a zero-friction review experience that agents can invoke programmatically and humans can use naturally.

The maximal vision: **A universal reflection substrate for structured artifacts** - not just Markdown review, but AST-level code inspection, schema-aware data editing, and composable transformation pipelines. Mirror becomes the visual frontend for `astq` (jq-for-AST), turning deterministic structural edits into interactive, human-guided operations.

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

### 1.1 Core UI (Milestone M1: Single-file review)

**Goal:** Launch mirror with single Markdown file, display rendered content, submit review to `.ddd/<filename>.review.1`.

**Features:**
- Basic `egui` window with rendered Markdown
- Text selection with mouse
- Comment dialog on selection
- Single "Submit Review" button
- Write JSONL to `.ddd/<filename>.review.1`
- Auto-exit on submit

**Implementation:**
- `src/main.rs` - CLI argument parsing, launch UI
- `src/ui/mod.rs` - egui application struct
- `src/ui/markdown.rs` - Markdown rendering via `egui_markdown`
- `src/ui/selection.rs` - Text selection tracking
- `src/ui/comment.rs` - Comment dialog widget
- `src/review.rs` - Review data structures (Comment, Selection)
- `src/storage.rs` - JSONL writing, monotonic sequence logic

**Testing:**
- Unit tests for JSONL serialization
- Integration tests for CLI invocation
- Manual testing for UI interactions

**Acceptance:**
```bash
mirror test.md
# User selects text, adds comment, clicks submit
# Verify .ddd/test.review.1 exists and contains valid JSONL
```

---

### 1.2 Multi-file tabs (Milestone M2)

**Goal:** Support multiple Markdown files as tabs, independent comment queues per file.

**Features:**
- Tab bar at top of window
- Click tab to switch active document
- Each tab shows comment count: `SPEC.md (3)`
- Submit writes separate `.review.N` file per document
- Tab close button (or just submit for all and exit)

**Implementation:**
- `src/ui/tabs.rs` - Tab bar widget, tab switching logic
- Refactor `ui/mod.rs` to hold `Vec<Document>` with active index
- Each `Document` has independent comment queue

**Testing:**
- Integration test: `mirror file1.md file2.md`
- Verify separate `.review.1` files created

**Acceptance:**
```bash
mirror SPEC.md PLAN.md
# User reviews both, adds comments to each
# Submit → verify .ddd/SPEC.review.1 and .ddd/PLAN.review.1
```

---

### 1.3 Immediate vs batched review (Milestone M3)

**Goal:** Two modes - immediate commenting (default) and batched review.

**Features:**
- Default: Every comment saves immediately (append to `.review.N`)
- "Start Review" button: Enter batch mode
  - Comments queued in memory only
  - "Submit Review" button appears
  - Click submit → atomic write all comments → exit
- User can toggle between modes mid-session

**Implementation:**
- `src/ui/review_mode.rs` - Enum: Immediate vs Batched
- Refactor `comment.rs` to check mode before writing
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

### 1.4 JSON output and environment integration (Milestone M4)

**Goal:** JSON output for Hegel integration, environment variable passthrough.

**Features:**
- `--json` flag: Emit JSON on stdout at exit with review file paths
- `--out-dir PATH` flag: Custom output directory (default `.ddd/`)
- Read `HEGEL_STATE_DIR` and `HEGEL_SESSION_ID` from environment
- Include session ID in review metadata

**Implementation:**
- `src/cli.rs` - Add flags to `clap` struct
- `src/storage.rs` - Accept `out_dir` parameter, read env vars
- `src/main.rs` - Print JSON on exit if `--json` flag set

**Testing:**
- Integration test: `mirror --json test.md`
- Verify JSON output format matches spec

**Acceptance:**
```bash
export HEGEL_SESSION_ID="abc123"
mirror --json SPEC.md --out-dir /tmp/reviews
# Verify JSON output: {"files":{"SPEC.md":"/tmp/reviews/SPEC.review.1"}}
# Verify session ID in review file metadata
```

---

### 1.5 Keyboard shortcuts and theming (Milestone M5)

**Goal:** Power-user navigation and visual polish.

**Features:**
- Keyboard shortcuts:
  - `Ctrl+Tab` / `Ctrl+Shift+Tab` - Next/previous tab
  - `Ctrl+Enter` - Submit review
  - `Escape` - Cancel current comment dialog
  - `Ctrl+Q` - Quit without submitting (batch mode)
- Theme support:
  - Auto-detect system dark/light mode
  - Manual toggle: `--theme dark` or `--theme light`
- Font size adjustment: `Ctrl+Plus` / `Ctrl+Minus`

**Implementation:**
- `src/ui/keyboard.rs` - Keyboard event handling
- `src/ui/theme.rs` - Theme definitions, system detection
- `egui` built-in theme support (egui::Visuals)

**Testing:**
- Manual testing for keyboard shortcuts
- Unit tests for theme detection logic

**Acceptance:**
```bash
mirror --theme dark SPEC.md
# Verify dark mode applied
# Test Ctrl+Tab, Ctrl+Enter, Escape shortcuts
```

---

## Phase 2: Structured Data Review

### 2.1 Multi-format support

**Goal:** Review not just Markdown, but XML, YAML, TOML, JSON - any structured text.

**Features:**
- Auto-detect file format from extension
- Format-specific rendering:
  - Markdown: Current rendering
  - XML: Syntax-highlighted tree view
  - YAML/TOML: Indented key-value display
  - JSON: Collapsible tree widget
- Selection anchoring adapts to format (line/col for all, node ID for structured)

**Implementation:**
- `src/formats/mod.rs` - Format enum, detection logic
- `src/formats/xml.rs` - XML rendering widget
- `src/formats/yaml.rs` - YAML rendering widget
- `src/formats/json.rs` - JSON tree widget
- Refactor `ui/markdown.rs` → `ui/formats.rs` with trait-based rendering

**Testing:**
- Integration tests for each format
- Verify comment selection anchors correctly

**Acceptance:**
```bash
mirror config.yaml schema.xml data.json
# Review each format with appropriate rendering
# Comments anchor to correct lines/nodes
```

---

### 2.2 Schema-aware validation

**Goal:** Load schema definitions (JSON Schema, XML Schema, etc.) and validate during review.

**Features:**
- `--schema PATH` flag to load schema definition
- Real-time validation: Highlight invalid nodes/fields
- Validation errors shown in sidebar
- Comments can reference validation errors

**Implementation:**
- `src/validation/mod.rs` - Validation trait, error types
- `src/validation/json_schema.rs` - JSON Schema validator
- `src/validation/xml_schema.rs` - XML Schema (XSD) validator
- `src/ui/validation.rs` - Validation error sidebar widget

**Libraries:**
- `jsonschema` crate for JSON Schema validation
- `roxmltree` + manual XSD logic (or find XSD validation crate)

**Testing:**
- Unit tests with valid/invalid documents
- Integration tests with schema files

**Acceptance:**
```bash
mirror --schema config.schema.json config.json
# Invalid fields highlighted in red
# Sidebar shows validation errors
# User can comment on errors
```

---

### 2.3 Diff view integration

**Goal:** Show diff between two versions of a document during review.

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

## Phase 3: AST Integration (Mirror ↔ astq)

### 3.1 AST-level code review

**Goal:** Review Rust code (or other languages) at AST level, not raw text.

**Features:**
- Parse Rust → AST via `syn`
- Display code with foldable AST nodes (functions, structs, modules)
- Select AST nodes, not text ranges
- Comments anchored to node IDs (stable across edits)

**Implementation:**
- `src/ast/rust.rs` - Rust AST parsing via `syn`
- `src/ui/ast_view.rs` - Foldable AST tree widget
- Extend comment metadata: `node_id` field instead of `line/col`
- Review file format: Include AST node path (e.g., `fn::init::block::stmt[3]`)

**Testing:**
- Unit tests for AST parsing
- Integration tests with sample Rust files
- Verify node ID stability

**Acceptance:**
```bash
mirror src/main.rs
# Display Rust code with foldable functions/structs
# User clicks function name → node selected
# Comment anchored to function node ID
```

---

### 3.2 Cross-language AST via Tree-sitter

**Goal:** Universal AST review for any language (Python, TypeScript, Go, etc.).

**Features:**
- Use Tree-sitter grammars for polyglot parsing
- Unified AST selection interface across languages
- Comments always anchored to node IDs (language-agnostic)

**Implementation:**
- `src/ast/tree_sitter.rs` - Tree-sitter integration
- Grammar loading from Tree-sitter shared libraries
- Extend `ast_view.rs` to render Tree-sitter nodes

**Libraries:**
- `tree-sitter` crate
- Individual grammar crates: `tree-sitter-python`, `tree-sitter-typescript`, etc.

**Testing:**
- Integration tests for each supported language
- Verify consistent node ID format

**Acceptance:**
```bash
mirror main.py app.ts server.go
# Each file displayed with AST tree view
# Comments anchor to Tree-sitter node IDs
# Cross-language consistent UX
```

---

### 3.3 Live astq transform preview

**Goal:** Preview `astq` transformations in Mirror before applying.

**Features:**
- `mirror --preview-transform TRANSFORM.xslt src/main.rs` mode
- Show before/after side-by-side
- Apply transform in memory, display result
- User can approve → write transformed file
- Or reject → discard changes

**Implementation:**
- `src/transform/mod.rs` - Transform engine (call `astq` binary or link library)
- `src/ui/transform_preview.rs` - Before/after diff widget
- Approval flow: "Accept" button → write file, "Reject" → discard

**Integration with astq:**
- Shell out to `astq` binary (MVP)
- Or link `astq` as library if feasible

**Testing:**
- Integration tests with sample transforms
- Verify before/after diff correctness

**Acceptance:**
```bash
mirror --preview-transform refactor.xslt src/lib.rs
# Side-by-side: original vs transformed
# User reviews changes
# Click "Accept" → astq writes transformed file
# Click "Reject" → no changes
```

---

## Phase 4: Advanced Features

### 4.1 Review templates

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

### 4.2 Export formats

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

### 4.3 Plugin system

**Goal:** Extensible comment handlers - run linters, check constraints, invoke external tools.

**Features:**
- Plugin hooks: `on_comment_added(comment) -> Result<(), Error>`
- Load plugins from `.ddd/plugins/` directory
- Plugins are WASM modules (cross-platform, sandboxed)
- Built-in plugins: Spell checker, link validator, constraint checker

**Implementation:**
- `src/plugins/mod.rs` - Plugin loader, WASM runtime
- `src/plugins/runtime.rs` - WASM runtime via `wasmtime` crate
- Plugin API: WASM imports/exports for comment data

**Example plugin:**
```rust
// Plugin: spell_checker.rs
#[no_mangle]
pub extern "C" fn on_comment_added(comment_json: *const u8, len: usize) -> i32 {
    let comment = parse_json(comment_json, len);
    if has_spelling_errors(&comment.text) {
        return -1; // Error
    }
    0 // OK
}
```

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

### 4.4 Real-time collaboration (stretch goal)

**Goal:** Multiple reviewers can comment simultaneously (same session).

**Features:**
- `mirror --server SPEC.md` launches server mode
- Other reviewers connect: `mirror --connect localhost:8080`
- Comments sync in real-time via WebSocket
- Conflict resolution: Last-write-wins, but show all reviewers' comments

**Implementation:**
- `src/server.rs` - WebSocket server (use `tokio` + `tokio-tungstenite`)
- `src/client.rs` - WebSocket client
- Comment sync protocol: JSON messages over WebSocket
- UI shows connected reviewers in sidebar

**Libraries:**
- `tokio` for async runtime
- `tokio-tungstenite` for WebSocket
- `serde_json` for message serialization

**Testing:**
- Integration tests with multiple clients
- Verify comment sync correctness

**Acceptance:**
```bash
# Terminal 1:
mirror --server SPEC.md

# Terminal 2:
mirror --connect localhost:8080

# Both reviewers see each other's comments in real-time
```

---

## Phase 5: Persistent Memory Substrate (Maximal Vision)

### 5.1 Memory-mapped AST graph

**Goal:** Move beyond ephemeral in-memory AST to persistent, memory-mapped graph database.

**Features:**
- Parse all files → persistent AST graph (via `lmdb` or `sled`)
- Incremental updates: Only reparse changed files
- Fast queries: XPath over entire codebase, not single file
- Mirror displays AST from graph, not re-parsing on launch

**Implementation:**
- `src/ast/graph.rs` - Memory-mapped graph storage
- `src/ast/incremental.rs` - Incremental parsing logic
- File watcher: Detect changes, trigger re-parse
- Mirror reads from graph instead of parsing on demand

**Benefits:**
- Instant launch (no re-parsing)
- Codebase-wide queries (cross-file references)
- Diff at AST level (structural diff, not text diff)

**Testing:**
- Unit tests for incremental updates
- Integration tests with file watching

**Acceptance:**
```bash
# Background daemon maintains AST graph
hegeld --watch ~/Code/my-project

# Mirror queries graph (instant launch)
mirror --query '//fn[@name="init"]' src/
# Shows all functions named "init" across codebase
```

---

### 5.2 Daemon mode (`mirrord`)

**Goal:** Long-running daemon maintains AST graph, handles file watches, serves Mirror UI on demand.

**Features:**
- `mirrord start` - Launch daemon
- `mirrord stop` - Stop daemon
- `mirror FILE.md` - Connect to daemon, instant launch
- Daemon maintains AST graph, handles incremental updates
- Multiple Mirror instances connect to same daemon

**Implementation:**
- `src/daemon/mod.rs` - Daemon server (Unix socket or TCP)
- `src/daemon/watcher.rs` - File watcher (via `notify` crate)
- `src/client/daemon.rs` - Mirror client connects to daemon
- Communication: JSON-RPC over socket

**Testing:**
- Integration tests with daemon start/stop
- Verify multiple clients connect successfully

**Acceptance:**
```bash
mirrord start --watch ~/Code/my-project
# Daemon maintains AST graph in background

mirror src/main.rs
# Instant launch (no parsing delay)

mirror src/lib.rs
# Second instance, same daemon, instant launch
```

---

### 5.3 Transformation registry

**Goal:** Declarative transformation pipelines with reproducible, composable codemod chains.

**Features:**
- `.hegel/transforms/` directory with XSLT stylesheets
- `mirror --transform-chain refactor.yaml src/`
- Transformation chain: Multiple transforms applied sequentially
- Parameterized transforms: Pass variables to XSLT
- Version-controlled: Transforms are code, committed to git

**Example chain:**
```yaml
# refactor.yaml
transforms:
  - name: "rename_function"
    xslt: "transforms/rename.xslt"
    params:
      old_name: "init"
      new_name: "initialize"
  - name: "add_logging"
    xslt: "transforms/add_logging.xslt"
    params:
      log_level: "info"
```

**Implementation:**
- `src/transform/chain.rs` - Transformation chain execution
- `src/transform/registry.rs` - Load transforms from `.hegel/transforms/`
- Integrate with `astq` for AST-level transforms

**Testing:**
- Integration tests with sample transform chains
- Verify idempotency (same input → same output)

**Acceptance:**
```bash
mirror --transform-chain refactor.yaml src/
# Preview all transforms in sequence
# User approves → transforms applied, files written
```

---

### 5.4 AST-level version control

**Goal:** Git pre-commit hooks operate on transformed ASTs, not raw text. Machine-verifiable provenance for all changes.

**Features:**
- Git hook: Pre-commit runs `astq` transforms
- Store transform provenance in commit metadata
- `git log --ast` shows AST-level diffs, not text diffs
- Rollback: Revert AST-level change, regenerate text

**Implementation:**
- `src/vcs/git_hook.rs` - Pre-commit hook logic
- `src/vcs/provenance.rs` - Store transform metadata in commit
- Integrate with Git: `git config core.hooksPath .hegel/hooks/`

**Testing:**
- Integration tests with git commits
- Verify provenance metadata in commit messages

**Acceptance:**
```bash
# .hegel/hooks/pre-commit runs astq transforms
git commit -m "Refactor init function"
# Commit metadata includes:
# - Transform chain applied
# - Input AST hash
# - Output AST hash
# - XSLT stylesheet version

git log --ast
# Shows AST-level diff:
# - Renamed fn::init → fn::initialize
# - Added logging call to fn::initialize::block
```

---

## Milestones Summary

| Phase | Milestone | Description | Target |
|-------|-----------|-------------|--------|
| 1 | M1 | Single-file Markdown review | Week 1 |
| 1 | M2 | Multi-file tabs | Week 2 |
| 1 | M3 | Immediate vs batched review | Week 3 |
| 1 | M4 | JSON output, env integration | Week 4 |
| 1 | M5 | Keyboard shortcuts, theming | Week 5 |
| 2 | - | Multi-format support | Month 2 |
| 2 | - | Schema-aware validation | Month 2 |
| 2 | - | Diff view integration | Month 2 |
| 3 | - | AST-level Rust review | Month 3 |
| 3 | - | Tree-sitter polyglot AST | Month 3 |
| 3 | - | Live astq preview | Month 3 |
| 4 | - | Review templates | Month 4 |
| 4 | - | Export formats | Month 4 |
| 4 | - | Plugin system | Month 4 |
| 4 | - | Real-time collaboration | Month 5 |
| 5 | - | Memory-mapped AST graph | Month 6 |
| 5 | - | Daemon mode (mirrord) | Month 6 |
| 5 | - | Transformation registry | Month 7 |
| 5 | - | AST-level version control | Month 8 |

---

## Success Metrics

**MVP (Phase 1):**
- Single binary, <10MB size
- Launch time: <500ms
- Cross-platform: macOS, Linux, Windows
- Zero config: `mirror FILE.md` just works

**Phase 2-3:**
- Multi-format support: Markdown, XML, YAML, JSON, Rust, Python, TypeScript
- AST-level review: Comments anchored to node IDs
- Integration with astq: Live transform preview

**Phase 4-5 (Maximal):**
- Persistent AST graph: <1s query across 100k-line codebase
- Daemon mode: <50ms launch time (connect to daemon)
- Transformation registry: Reproducible, composable codemod chains
- AST-level version control: Machine-verifiable provenance

---

*Thesis: Ephemeral review UI. Antithesis: Persistent AST substrate. Synthesis: Mirror.*
