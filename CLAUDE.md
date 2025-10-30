# CLAUDE.md

**Hegel Mirror**: Ephemeral Markdown review UI for Dialectic-Driven Development. Zero-bloat, instant-launch GUI for in-line document review that agents can invoke and humans can approve/annotate.

---

## Hegel Workflow Orchestration

This project uses **Hegel CLI** for structured development workflows. Quick reference:

```bash
hegel status            # Check active workflow
hegel next              # Advance to next phase
hegel reflect FILE.md   # Launch review GUI
hegel astq 'pattern'    # AST-aware code search (prefer over grep)
```

**See [HEGEL.md](HEGEL.md) for complete command reference and integration patterns.**

---

## Architecture

**Core**: `src/{main,app,models,parsing,rendering,storage,syntax,theme}` - Trait-based rendering with viewport culling
**UI**: `egui`/`eframe` - Immediate-mode GUI, native performance, cross-platform
**Markdown**: `pulldown-cmark` - Parse → positioned TextChunks with line/col tracking
**Rendering**: Trait-based system (ChunkRenderer) with lazy loading and cached heights for 60fps on 11K+ line documents
**Storage**: `.ddd/<filename>.review.N` - Append-only review logs, monotonic sequence numbers
**Integration**: Environment passthrough (`HEGEL_STATE_DIR`, `HEGEL_SESSION_ID`) for seamless Hegel workflow integration

**See [src/CODE_MAP.md](src/CODE_MAP.md) for detailed module documentation.**

---

## Philosophy (Inherited from Hegel LEXICON.md)

**Context is king** - Line counts are physics, not style. Token overhead is immediate cost. Refactor on pattern, not pain.

**Artifacts disposable, clarity durable** - Code rewrites. Insights don't. Generation cheap, understanding valuable.

**Infrastructure compounds** - Helpers, submodules, test patterns save context forever. Build once, reuse infinitely.

**Test density is infrastructure** - Verbose patterns = compounding friction. Extract early, compress aggressively.

**Remember you're not human** - No cost to thoroughness. 18x token waste is real waste, not hypothetical debt.

**The human always knows best** - Execute instructions. Don't editorialize. Questions are literal, not criticism.

**Refactor early, not late** - Structure for reading efficiency, not writing comfort. 200+ line files trigger immediate split.

**Mirror-specific additions:**

**Never ask the human to run commands** - Always execute commands directly. Never say "try running X" or "test by running Y". Run it yourself and report results.

**Never propose deferring work** - Everything is in scope unless explicitly declared out of scope. Don't suggest "defer to Phase 2" or "save for later" unless the problem is literally impossible (NASA-level hard with 10+ years effort). Solve problems now.

**NEVER delete code to make tests pass** - If tests fail, fix the tests or fix the implementation. NEVER remove functionality to resolve test failures. Deletion to achieve passing tests is forbidden. No exceptions.

**Concise beats comprehensive** - In specs, docs, and communication: precise and short beats verbose and complete. Cut ruthlessly.

**Ephemerality is a feature** - No persistent project state, no configuration files, no hidden state. Launch → review → write → exit. Like `$EDITOR` for git commits.

**Agent-first, human-compatible** - Designed for `hegel reflect` invocation by AI agents, but equally delightful for human use. The UI is the API.

**Zero friction** - Single binary, no install, no dependencies, no config. `mirror FILE.md` just works.

---

## Core UX Flow

### Launch
```bash
# From Hegel workflows
hegel reflect SPEC.md PLAN.md

# Standalone
mirror DISCOVERY.md --out-dir .ddd/
```

### Review Modes

**1. Immediate commenting (default)**
- Select text → comment dialog appears
- Write comment → auto-saves to `.ddd/<filename>.review.N`
- Continue selecting/commenting (append-only)
- Close window when done

**2. Batched review**
- Click "Start Review" → enters review mode
- Select text → queue comment (not saved yet)
- All comments held in memory
- Click "Submit Review" → atomic write to `.ddd/<filename>.review.N`
- Auto-exit on submit

### Multi-file tabs
- Multiple files open as tabs
- Each file has independent comment queue
- Tab label shows comment count: `SPEC.md (3)`
- Submit writes separate `.review.N` file per document

---

## CLI/API Contract

### Basic invocation
```bash
mirror FILE1.md [FILE2.md ...]
```

### Flags
- `--out-dir PATH` - Where to write `.review.N` files (default: `.ddd/`)
- `--json` - Emit JSON with review file paths on stdout at exit
- `--headless` - No-op mode, just create placeholder `.review.N` (for CI/testing)

### Environment variables (Hegel integration)
- `HEGEL_STATE_DIR` - Pass to mirror for context (optional)
- `HEGEL_SESSION_ID` - Include in review metadata (optional)

### Exit behavior
- Exit code 0: Review submitted successfully
- Exit code 1: Error (file not found, write failed, etc.)
- Exit code 2: User cancelled (closed without submitting in batch mode)

---

## Review File Format

### `.ddd/<filename>.review.N`

Append-only JSONL format, one comment per line:

```jsonl
{"timestamp":"2025-10-13T02:45:00Z","session_id":"abc123","file":"SPEC.md","selection":{"start":{"line":15,"col":0},"end":{"line":18,"col":42}},"text":"selected text snippet","comment":"This constraint needs clarification - what happens if X?"}
{"timestamp":"2025-10-13T02:46:15Z","session_id":"abc123","file":"SPEC.md","selection":{"start":{"line":42,"col":0},"end":{"line":42,"col":80}},"text":"another snippet","comment":"Typo: 'recieve' → 'receive'"}
```

### Monotonic sequence
- First review: `SPEC.review.1`
- Second review: `SPEC.review.2`
- Etc.
- Never overwrites previous reviews (historical record)

---

## Critical Patterns

**Single binary distribution**: Compile to static binary, distribute via GitHub releases. No install script, no package manager, just download and run.

**Immediate mode UI**: `egui` immediate mode paradigm - no retained widget tree, redraw on every frame. Simple mental model, easy to reason about.

**Minimal state**: Only state is current selection + comment queue. Everything else derived from files on disk.

**Graceful degradation**: If `.ddd/` doesn't exist, create it. If write fails, show error but don't crash. Always try to preserve user's work.

**Testing strategy**:
- Unit tests for review file parsing/writing
- Integration tests for CLI argument parsing
- Manual testing for UI (egui testing is limited)
- Snapshot tests for JSON output

---

## Development Constraints

**Platform**: Cross-platform (macOS, Linux, Windows) via Rust + egui
**Language**: Rust stable
**UI Framework**: egui/eframe (immediate mode, native, no web dependencies)
**Dependencies**: Minimal
  - `egui` + `eframe` - UI
  - `pulldown-cmark` - Markdown parsing
  - `egui_markdown` - Markdown rendering in egui
  - `serde` + `serde_json` - Review file serialization
  - `anyhow` - Error handling
  - `clap` - CLI argument parsing

**Build targets**:
- `x86_64-apple-darwin` (Intel Mac)
- `aarch64-apple-darwin` (Apple Silicon)
- `x86_64-unknown-linux-gnu` (Linux)
- `x86_64-pc-windows-msvc` (Windows)

---

## Integration with Hegel

### Agent invocation pattern
```yaml
# In hegel workflows/discovery.yaml
nodes:
  spec:
    prompt: |
      Write SPEC.md with technical rigor.

      After writing, invoke reflection:

      ```bash
      hegel reflect SPEC.md
      ```

      This will launch the review UI. When the human submits their
      review, you'll find their comments in .ddd/SPEC.review.N.

      Read those comments and revise SPEC.md accordingly.
```

### Human approval workflow
1. Agent writes SPEC.md
2. Agent runs `hegel reflect SPEC.md`
3. Mirror launches, human reviews/comments
4. Human clicks "Submit Review" → mirror writes `.ddd/SPEC.review.1` and exits
5. Agent reads `.ddd/SPEC.review.1` and revises
6. Agent runs `hegel reflect SPEC.md` again
7. Human reviews revision → `.ddd/SPEC.review.2`
8. Iterate until human closes without commenting (approval signal)

---

## Non-Goals (MVP)

**NO code display/diff** - Markdown only. Code review is a different tool.

**NO network/cloud** - Fully local, fully offline.

**NO persistent project state** - No `.mirror/` directory, no config files, no hidden state.

**NO syntax highlighting** - Markdown is already formatted. Keep it simple.

**NO real-time collaboration** - Single user, single session. Review files provide async collaboration.

**NO diff view** - Just display current document. Historical reviews are in `.ddd/` if needed.

---

## Future Enhancements (Maybe)

**Enhanced rendering**: Table of contents, in-document search, improved image handling.

**Diff view**: Side-by-side comparison for Markdown revisions.

**Theming**: Dark mode, customizable fonts, accessibility options.

**Keyboard shortcuts**: Power-user navigation (j/k, vim-style, Emacs bindings).

**Review templates**: Pre-defined comment types (question, suggestion, blocker, typo) with structured metadata.

**Export formats**: HTML, PDF, annotated Markdown with inline comments.

**Plugin system**: WASM-based extensible comment handlers (spell check, link validation).

---

## External Documentation Cache

**Prefer cargo doc, fallback to .webcache/**

**For Rust crates**:
- Use `cargo doc --no-deps -p <crate_name>` to generate local docs
- Docs appear in `target/doc/<crate>/`
- Use `lynx -dump -nolist target/doc/<crate>/path.html` to read them
- Much faster than web fetches, always version-correct

**For non-Rust docs** (.webcache/ fallback):
- **Purpose**: Cache external web references locally
- **Location**: `.webcache/` (gitignored)
- **Download**: `curl -s <url> -o .webcache/<filename>.html`
- **Why**:
  - Offline access during development
  - Version stability (web docs change)
  - Faster lookup than repeated web fetches

**Reading cached HTML files**:
- Use `lynx -dump -nolist /path/to/file.html` to convert HTML → clean text
- Strips HTML tags, formats tables, preserves structure, easier to parse than raw HTML

---

**Remember**: Mirror is ephemeral. Launch → review → write → exit. No ceremony, no friction, no state. Just pure review flow.
