# Doc-Driven Development for Mirror

**Mirror-adapted DDD methodology**: Emphasizes toy models for validating complex GUI, AST, and transform features before production integration.

---

## Purpose

This document defines how Dialectic-Driven Development (DDD) applies to Mirror development. Mirror's roadmap spans from simple Markdown review (Phase 1 MVP) to complex AST substrates and transformation pipelines (Phases 4-5). **Toy models are the primary risk-reduction mechanism** for validating uncertain techniques before production.

For general DDD philosophy and economic foundations, see: https://github.com/dialecticianai/ddd-book/

---

## Economic Foundation (Mirror Context)

AI generation makes artifacts cheap; clarity and validated patterns are valuable.

**Mirror-specific implications:**
- UI widgets are cheap to regenerate → focus on UX clarity and interaction patterns
- AST parsers are cheap to write → focus on node selection semantics and stability
- Transforms are cheap to implement → focus on correctness guarantees and composition rules

**Result:** Toy models validate complex patterns (egui widgets, Tree-sitter integration, XSLT transforms) in isolation before building production features.

---

## Operational Modes

### Discovery Mode (Primary for Phases 2-5)

**When to use:**
- Novel GUI patterns (foldable AST trees, diff views, selection anchoring)
- AST integration uncertainty (Tree-sitter, `syn`, node ID stability)
- Transform pipelines (XSLT, astq integration, preview mechanisms)
- Cross-language patterns (polyglot AST selection)

**Artifacts:**
- SPEC.md, PLAN.md, LEARNINGS.md per toy
- Toy implementations in `toys/toyN_name/`
- Extracted patterns ready for `src/` integration

**Discipline:**
- One complexity axis per toy (egui selection widget, NOT "entire review UI")
- Integration toys combine two validated base toys
- Test-driven when possible (unit tests for parsing, manual for GUI rendering)

**Output:**
- Working patterns with known constraints
- LEARNINGS.md with cycle counts, memory usage, UX observations
- Reference implementations kept as intermediate artifacts

### Execution Mode (Phase 1 MVP, refinements)

**When to use:**
- Proven patterns and established codebase structure
- Feature additions to mature modules
- Refactoring and quality improvements

**Artifacts:**
- CODE_MAP.md (living architecture map)
- Production codebase (`src/`)
- LEARNINGS.md (optional, only for unexpected insights)

**Discipline:**
- Mandatory refactoring after features
- CODE_MAP.md sync before structural commits
- Focus on orchestration and quality maintenance

---

## Toy Models: Mirror-Specific Patterns

Mirror's roadmap requires validating complex patterns before production. Toys de-risk:

### GUI Patterns (egui/eframe)

**Examples:**
- `toy1_egui_selection` - Text selection with mouse, anchor tracking
- `toy2_markdown_render` - pulldown-cmark → egui_markdown rendering
- `toy3_comment_dialog` - Modal dialog, keyboard shortcuts, focus management
- `toy4_tabs` - Multi-file tab bar, active state, comment counts

**Testing:**
- Unit tests: Selection logic (char offsets, line/col mapping)
- Manual validation: Render in egui, observe interactions
- Document in LEARNINGS.md: UX observations, performance (frames/sec, memory)

### AST Integration

**Examples:**
- `toy5_syn_parsing` - Rust AST via `syn`, node ID generation
- `toy6_tree_sitter` - Python/TypeScript parsing, unified node addressing
- `toy7_ast_selection` - Click node → highlight in UI, anchor to node ID
- `toy8_xml_serialization` - AST → XML → XPath queries

**Testing:**
- Unit tests: Parse → serialize → parse round-trip (lossless)
- Golden tests: Known source → expected AST structure
- Manual: Render AST tree in egui, test node selection

### Transform Pipelines

**Examples:**
- `toy9_xslt_preview` - Load XSLT, apply to XML AST, show before/after
- `toy10_astq_integration` - Shell to `astq` binary, parse output
- `toy11_transform_chain` - Sequential transforms, intermediate results
- `toy12_node_stability` - Node IDs stable across edits (insert/delete/move)

**Testing:**
- Unit tests: Transform correctness, idempotency
- Integration tests: Chain multiple transforms, verify composition
- Manual: Preview in UI, compare before/after diffs

### Structured Data Formats

**Examples:**
- `toy13_xml_render` - XML tree view, collapsible nodes
- `toy14_yaml_parse` - YAML → structured display, selection anchoring
- `toy15_json_tree` - JSON → egui tree widget, expand/collapse
- `toy16_schema_validation` - JSON Schema validation, error highlighting

**Testing:**
- Unit tests: Parse valid/invalid documents, schema validation
- Manual: Render in UI, test selection and comment anchoring

---

## The Toy Cycle (Mirror Adaptation)

### 1. Define Learning Goals (LEARNINGS.md - First Pass)

Before implementation, document questions:
- **GUI questions**: "Can egui track text selection across frame redraws?" (docs unclear, test it)
- **AST questions**: "Are Tree-sitter node IDs stable after edits?" (critical for comment anchoring)
- **Transform questions**: "Can XSLT handle Rust AST → modified AST round-trip?" (core assumption)
- **Performance questions**: "Frame rate with 10k-line AST rendered?" (UX threshold)

**Success criteria:** What measurements prove the pattern works for production?

### 2. Write Specifications (SPEC.md + PLAN.md)

- **SPEC**: What the toy must do, observable outputs, success criteria
- **PLAN**: Test-first steps (unit tests → impl → manual validation → refactor)
- **One axis**: Isolate single complexity (egui selection, NOT "entire review UI with tabs and themes")

### 3. Test-First Implementation

#### For Parsers/Logic (Unit Tests)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rust_ast_round_trip() {
        let source = "fn main() {}";
        let ast = parse(source).unwrap();
        let xml = ast_to_xml(&ast);
        let ast2 = xml_to_ast(&xml).unwrap();
        assert_eq!(ast, ast2); // Lossless round-trip
    }
}
```

**Workflow:**
1. Write failing test (Red)
2. Implement minimal logic (Green)
3. Refactor for clarity
4. Commit: `feat(toyN): Step X - description`

#### For GUI Rendering (Manual Validation)

**Tools:**
- Run toy binary, interact with UI
- Observe frame rate (egui built-in profiler)
- Test edge cases (empty input, large files, rapid interactions)

**Document in LEARNINGS.md:**
- Frame rate measurements (60fps target)
- Memory usage (heap allocations, egui retained state)
- UX observations (laggy selection, keyboard shortcuts feel)
- Edge cases discovered (crash on empty file, etc.)

### 4. Extract Learnings (LEARNINGS.md - Final Pass)

Finalize with findings:
- Performance measurements (frame rate, memory, latency)
- Constraints discovered (egui limitations, Tree-sitter quirks)
- Working patterns ready for `src/` reuse
- Open questions answered or spawned

**Example:**
```markdown
## Validated
- egui tracks selection across frames: ✅ (stores in App state)
- 60fps with 5k-line Markdown: ✅ (egui_markdown efficient)

## Challenged
- Tree-sitter node IDs NOT stable after edits ⚠️
  - Node IDs are byte offsets, change on insert/delete
  - **Production impact**: Must use line/col anchoring, not node IDs
  - **Mitigation**: Store node path (e.g., `fn::init::block::stmt[3]`)

## Patterns for Production
- Selection state: Store in App struct, not widget-local
- Text anchoring: Use (line, col, text_snippet) triple for stability
- Performance: Lazy rendering (only visible lines), not full document
```

---

## Timeboxing & Partial Validation

**CRITICAL**: Not all toys achieve 100% success. Partial validation delivers value.

### The 3-Attempt Rule

When tests fail after implementation:
1. **Attempt 1**: Debug obvious issues (logic bugs, API misunderstanding)
2. **Attempt 2**: Deep investigation (read docs, trace execution, check assumptions)
3. **Attempt 3**: Final debug pass or clean rebuild

**After 3 attempts: STOP and document.**

### Partial Validation Is Complete

A toy is complete when:
1. ✅ All tests passing (100% validation)
2. ⏱️ 3 debugging attempts exhausted (partial validation)
3. 🎯 Learning goals answered (even if implementation incomplete)

**Value delivered:**
- Isolates working parts (4/8 passing proves infrastructure)
- Documents gotchas (what DOESN'T work is knowledge)
- Prevents rabbit holes (timeboxing protects productivity)
- Unblocks other work (don't wait for perfection)

### Document in LEARNINGS.md

```markdown
**Duration**: 60 min | **Status**: Complete (partial) ✅ | **Result**: 3/5 tests passing

### ⚠️ Challenged

**Timeboxed after 3 debugging attempts:**
- Attempt 1: Fixed egui state initialization bug
- Attempt 2: Investigated selection coordinate mapping (found off-by-one)
- Attempt 3: Tried alternative approach (egui built-in text edit widget)
- **Decision**: Document findings, use egui built-in for MVP

**What works (3/5 tests):**
- Single-line selection: ✅
- Multi-line selection: ✅
- Copy to clipboard: ✅

**What fails (2/5 tests):**
- Selection across wrapped lines: Returns wrong char offsets
- Right-to-left selection: Doesn't highlight correctly

**Value delivered:**
- ✅ Validated core selection logic
- ✅ Found egui built-in text edit widget (better UX, less code)
- ✅ Partial validation > 0% validation
- ✅ Decision: Use egui::TextEdit for MVP, defer custom selection to Phase 2
```

---

## Axis Principle (Mirror Adaptation)

From DDD book: "A base toy isolates exactly one axis of complexity."

**For Mirror:**
- **GUI toys**: One widget pattern (selection, tabs, dialog)
- **AST toys**: One parser or one operation (parse, serialize, query)
- **Transform toys**: One transform type or one composition rule
- **Integration toys**: Exactly two validated base toys (interaction is the axis)

**Examples:**
- ✅ `toy1_egui_selection` - Just text selection logic (one axis)
- ✅ `toy5_syn_parsing` - Just Rust AST parsing (one axis)
- ✅ `toy17_selection_ast` - Integration of toy1 + toy5 (GUI selection over AST nodes)
- ❌ `toy_bad` - Full review UI with tabs, selection, AST, themes (four axes - split into separate toys)

---

## Relationship to Production Code

### Before Production Feature

1. **Identify uncertainty**: "How does egui handle large file rendering?"
2. **Build toy**: `toy_egui_large_file` - 100k-line Markdown rendering test
3. **Extract patterns**: Lazy rendering, viewport culling, chunk loading
4. **Document constraints**: Max file size, frame rate thresholds

### During Production Integration

1. **Reference toy LEARNINGS.md**: Copy validated patterns to `src/`
2. **Apply constraints**: Enforce file size limits, use lazy rendering
3. **Don't reopen toys**: Toy is reference artifact, not living code
4. **If new uncertainty emerges**: Build new toy, don't modify old one

### After Production Complete

1. **Update CODE_MAP.md**: Document which toy patterns informed which `src/` modules
2. **Keep toys as reference**: Permanent artifacts, allowed dead code
3. **Cite in commit messages**: `feat(ui): lazy Markdown rendering (pattern from toy2_markdown_render)`

---

## Testing Strategy (Mirror)

### Unit Tests (Parsers, Logic, Storage)

- Parse/serialize round-trips (AST → XML → AST, lossless)
- JSONL review file reading/writing (correct schema, monotonic sequence)
- Node ID generation (stable, unique, deterministic)
- Transform correctness (same input → same output, idempotent)

**Tools:** `cargo test`, snapshot tests (`insta` crate) for AST structures

### Manual Validation (GUI, UX, Performance)

- egui rendering (visual correctness, layout, theming)
- Interactions (selection, keyboard shortcuts, focus management)
- Performance (frame rate, memory, latency on large files)
- Edge cases (empty files, very long lines, rapid interactions)

**Tools:** Run binary, observe behavior, measure with egui profiler

### Integration Tests (CLI, File I/O, Subprocesses)

- CLI argument parsing (`--json`, `--out-dir`, flags)
- Review file writing (atomic writes, correct paths, JSON output)
- Environment variable passthrough (`HEGEL_SESSION_ID`)
- Subprocess invocation (shell to `astq` binary, parse output)

**Tools:** `cargo test --test integration_tests`, shell script tests (Perl if complex)

---

## Mandatory Refactoring

Not optional. Core discipline after every feature integration.

**Why this works for Mirror:**
- Economic inversion: Code regeneration is cheap (AI), clarity is valuable
- egui benefits from clean widget extraction (reusable components)
- AST code benefits from helper functions (reduce token waste in prompts)

**Triggers:**
- After toy → production integration (extract patterns, eliminate duplication)
- After feature completion (simplify, reduce file sizes, improve naming)
- Before structural commits (keep CODE_MAP.md sync, maintain clarity)

---

## Core Artifacts

### README.md (per module/toy)

**Purpose:** 100-200 words context refresh for AI - what it does, key API, gotchas

**Must contain:**
- One-liner header
- 2-3 sentence purpose
- 3-5 essential function signatures
- Core concepts
- Gotchas/caveats
- Quick test command

### SPEC.md (per toy/feature)

**Purpose:** Comprehensive behavioral contract

**Must contain:**
- Input/output formats (JSON schemas, GUI interactions)
- Invariants (node ID stability, selection anchoring semantics)
- Operations (parse, render, select, comment, submit)
- Validation rules (schema checks, error handling)
- Success criteria (frame rate thresholds, memory limits)

### PLAN.md (per toy/feature)

**Purpose:** Strategic roadmap with test-first steps

**Must contain:**
- Test vs manual validation split
- Order of steps (unit tests → impl → manual → refactor)
- Timeboxing (3-attempt rule for debugging)
- Dependencies (which toys must complete first)
- Risks (unknowns, assumptions to validate)

### LEARNINGS.md (per toy, optional for production)

**Purpose:** Capture patterns, constraints, measurements, and insights

**Required for:** Discovery mode (toys)
**Optional for:** Execution mode (production, only if unexpected insights)

**Must contain:**
- Performance measurements (frame rate, memory, latency)
- Constraints discovered (platform limits, library quirks)
- Working patterns (code ready for reuse)
- Challenged assumptions (theory vs reality)
- Open questions (spawned during implementation)

### CODE_MAP.md (per directory)

**Purpose:** Living architectural map

**Must contain:**
- File descriptions (current directory only, non-recursive)
- Logical grouping (UI widgets, AST parsers, storage, CLI)
- Integration points (which modules depend on which)

**Update trigger:** Before structural commits (add/remove/rename files)

---

## Repository Layout (Mirror)

### Discovery Mode (Toys)
```
toys/
  toy1_egui_selection/
    SPEC.md, PLAN.md, LEARNINGS.md, README.md
    Cargo.toml, src/main.rs
  toy2_markdown_render/
    SPEC.md, PLAN.md, LEARNINGS.md, README.md
    Cargo.toml, src/main.rs
  ...
```

### Execution Mode (Production)
```
src/
  main.rs
  ui/
    CODE_MAP.md
    mod.rs, tabs.rs, selection.rs, comment.rs
  ast/
    CODE_MAP.md
    mod.rs, rust.rs, tree_sitter.rs, xml.rs
  storage.rs
  review.rs
  cli.rs

CODE_MAP.md  (top-level src/ structure)
```

---

## Napkin Physics Mode (Upstream Simplification)

Use before SPEC/PLAN to encourage parsimony.

**Output structure:**
- **Problem:** One sentence
- **Assumptions:** 3-5 bullets
- **Invariant/Contract:** One precise property
- **Mechanism:** ≤5 bullets, stdlib or minimal deps
- **First Try:** One paragraph, simplest path

**Prohibitions:**
- No frameworks (egui is the framework, don't add layers)
- No new abstractions unless two deleted
- No new dependencies unless justified in SPEC

---

## Success Criteria (per slice)

- Minimal spike demonstrates core mechanism end-to-end
- Tests pass (unit tests) or manual validation complete
- LEARNINGS.md captures at least one insight/constraint
- CODE_MAP.md in sync (for production work)
- Complexity guardrails respected (file sizes, function lengths)
- Toy patterns extracted and ready for production (Discovery mode)

---

## Roadmap-Specific Toy Guidance

### Phase 1 (MVP - Markdown Review)

**Low uncertainty, mostly execution mode:**
- Few toys needed (egui selection, markdown rendering)
- Focus on production quality and refactoring
- CODE_MAP.md central artifact

### Phase 2 (Structured Data)

**Medium uncertainty, mixed mode:**
- Toys for XML/YAML/JSON rendering widgets
- Schema validation toys (JSON Schema, XSD)
- Diff view integration toys

### Phase 3 (AST Integration)

**High uncertainty, primarily Discovery mode:**
- AST parsing toys (`syn`, Tree-sitter)
- Node ID stability toys (edit operations)
- XML serialization toys (AST → XML → XPath)
- astq integration toys (shell invocation, output parsing)
- GUI integration toys (AST tree rendering, node selection)

### Phase 4 (Advanced Features)

**Medium uncertainty, focused toys:**
- Review template toys (structured metadata)
- Export format toys (HTML, PDF generation)
- Plugin system toys (WASM runtime, plugin API)

### Phase 5 (Persistent Substrate)

**Extreme uncertainty, extensive Discovery mode:**
- Memory-mapped graph toys (`lmdb`, `sled`)
- Incremental parsing toys (file watcher → partial reparse)
- Daemon IPC toys (Unix socket, JSON-RPC)
- Transform registry toys (XSLT chaining, parameterization)
- AST-level VCS toys (Git hook integration, provenance metadata)

**Expect 10-20 toys for Phase 5 alone.** Many will be partial validation (timeboxed). This is expected and valuable.

---

## North Star

**Toys are reconnaissance, not construction.**

Scout uncertain patterns without production constraints. Focus: understanding constraints and discovering viable approaches. Result: validated patterns applied to `src/`, toys kept as reference artifacts.

When theory meets egui's retained mode rendering or Tree-sitter's node IDs, update the theory.

---

## References

- **DDD.md**: Core methodology (https://github.com/dialecticianai/ddd-book/)
- **ROADMAP.md**: Mirror's 5-phase development plan
- **CLAUDE.md**: Mirror-specific development context and philosophy
