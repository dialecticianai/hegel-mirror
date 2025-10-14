# Doc-Driven Development for Mirror

**Mirror-adapted DDD methodology**: Emphasizes toy models for validating complex GUI patterns before production integration.

---

## Purpose

This document defines how Dialectic-Driven Development (DDD) applies to Mirror development. Mirror is a Markdown preview and review UI - launch, read, comment, submit. **Toy models are the primary risk-reduction mechanism** for validating uncertain GUI techniques before production.

For general DDD philosophy and economic foundations, see: https://github.com/dialecticianai/ddd-book/

---

## Economic Foundation (Mirror Context)

AI generation makes artifacts cheap; clarity and validated patterns are valuable.

**Mirror-specific implications:**
- UI widgets are cheap to regenerate → focus on UX clarity and interaction patterns
- Rendering code is cheap to write → focus on performance characteristics and edge cases
- Comment flows are cheap to implement → focus on user experience and data persistence

**Result:** Toy models validate complex patterns (egui widgets, selection tracking, lazy rendering) in isolation before building production features.

---

## Operational Modes

### Discovery Mode (Primary for Phase 2-3)

**When to use:**
- Novel GUI patterns (floating comment UI, selection anchoring, lazy rendering)
- Performance uncertainty (large file handling, viewport culling, frame rates)
- Integration patterns (Markdown parsing, syntax highlighting, image alignment)

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

Mirror's roadmap requires validating GUI patterns before production. Toys de-risk:

### GUI Patterns (egui/eframe)

**Examples:**
- `toy1_egui_selection` - Text selection with mouse, anchor tracking
- `toy2_markdown_render` - pulldown-cmark → egui rendering with lazy loading
- `toy3_comment_dialog` - Floating dialog, positioning, scroll indicators
- `toy4_tabs` - Multi-file tab bar, active state, comment counts
- `toy5_syntax_highlighting` - Code block highlighting with syntect
- `toy6_image_alignment` - HTML parsing, centered images, width constraints

**Testing:**
- Unit tests: Selection logic (char offsets, line/col mapping), parsing correctness
- Manual validation: Render in egui, observe interactions
- Document in LEARNINGS.md: UX observations, performance (frames/sec, memory)

### Performance Patterns

**Examples:**
- `toy7_large_files` - 10k+ line documents, frame rate testing
- `toy8_viewport_culling` - Lazy rendering, cached heights
- `toy9_image_loading` - Texture caching, memory management
- `toy10_selection_perf` - Multi-chunk selection, layout mapping

**Testing:**
- Performance tests: Frame rate benchmarks, memory profiling
- Edge cases: Very long lines, many images, rapid scrolling
- Manual: Use egui profiler, measure frame times

### Data Patterns

**Examples:**
- `toy11_review_format` - JSONL persistence, monotonic sequences
- `toy12_comment_state` - Immediate vs batched modes
- `toy13_session_tracking` - Environment variable integration
- `toy14_file_io` - Atomic writes, error handling

**Testing:**
- Unit tests: Parse → serialize → parse round-trip (lossless)
- Integration tests: File writes, JSON output, CLI flags
- Manual: Test failure modes, verify data integrity

---

## The Toy Cycle (Mirror Adaptation)

### 1. Define Learning Goals (LEARNINGS.md - First Pass)

Before implementation, document questions:
- **GUI questions**: "Can egui track text selection across frame redraws?" (docs unclear, test it)
- **Performance questions**: "Frame rate with 10k-line Markdown?" (UX threshold)
- **Integration questions**: "How to parse HTML in Markdown for image alignment?" (library support)

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
    fn parse_markdown_with_html_images() {
        let markdown = "<p align=\"center\"><img src=\"logo.png\" width=\"200\"></p>";
        let chunks = parse_markdown(markdown);
        assert_eq!(chunks[0].alignment, Some(Alignment::Center));
        assert_eq!(chunks[0].image_width, Some(200.0));
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
- Constraints discovered (egui limitations, rendering quirks)
- Working patterns ready for `src/` reuse
- Open questions answered or spawned

**Example:**
```markdown
## Validated
- egui tracks selection across frames: ✅ (stores in App state)
- 60fps with 11k-line Markdown: ✅ (lazy rendering + viewport culling)

## Challenged
- HTML parsing needs custom logic ⚠️
  - pulldown-cmark emits HTML as opaque text blocks
  - **Production impact**: Must parse HTML manually for image alignment
  - **Mitigation**: Simple string parsing for <p align> and <img> tags

## Patterns for Production
- Selection state: Store in App struct, not widget-local
- Text anchoring: Use (line_start, line_end) for stable selection
- Performance: Lazy rendering (only visible chunks), cached heights
- HTML handling: Custom parser for specific patterns we care about
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

---

## Axis Principle (Mirror Adaptation)

From DDD book: "A base toy isolates exactly one axis of complexity."

**For Mirror:**
- **GUI toys**: One widget pattern (selection, tabs, dialog)
- **Rendering toys**: One rendering challenge (lazy loading, syntax highlighting, alignment)
- **Data toys**: One persistence pattern (JSONL writes, atomic updates)
- **Integration toys**: Exactly two validated base toys (interaction is the axis)

**Examples:**
- ✅ `toy1_egui_selection` - Just text selection logic (one axis)
- ✅ `toy2_markdown_render` - Just Markdown rendering (one axis)
- ✅ `toy17_selection_comments` - Integration of toy1 + toy3 (selection → comment flow)
- ❌ `toy_bad` - Full review UI with tabs, selection, themes, persistence (four axes - split!)

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

### Unit Tests (Parsing, Logic, Data)

- Markdown parsing correctness (chunks, positions, metadata)
- JSONL review file reading/writing (correct schema, monotonic sequence)
- HTML parsing (image alignment, width extraction)
- Selection logic (line ranges, coordinate mapping)

**Tools:** `cargo test`, snapshot tests (`insta` crate) for parsed structures

### Manual Validation (GUI, UX, Performance)

- egui rendering (visual correctness, layout, theming)
- Interactions (selection, keyboard shortcuts, focus management)
- Performance (frame rate, memory, latency on large files)
- Edge cases (empty files, very long lines, rapid interactions)

**Tools:** Run binary, observe behavior, measure with egui profiler

### Integration Tests (CLI, File I/O)

- CLI argument parsing (`--json`, `--out-dir`, flags)
- Review file writing (atomic writes, correct paths, JSON output)
- Environment variable passthrough (`HEGEL_SESSION_ID`)

**Tools:** `cargo test --test integration_tests`

---

## Mandatory Refactoring

Not optional. Core discipline after every feature integration.

**Why this works for Mirror:**
- Economic inversion: Code regeneration is cheap (AI), clarity is valuable
- egui benefits from clean widget extraction (reusable components)
- Rendering code benefits from helper functions (reduce duplication)

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
- Invariants (selection stability, comment ordering)
- Operations (parse, render, select, comment, submit)
- Validation rules (file format checks, error handling)
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
- Logical grouping (UI widgets, parsers, storage, CLI)
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
  app.rs
  parsing/
    CODE_MAP.md
    mod.rs, parser.rs, chunks.rs, position.rs
  rendering/
    CODE_MAP.md
    mod.rs, chunk.rs, text.rs, code.rs, image.rs, table.rs, comments.rs
  models/
    CODE_MAP.md
    mod.rs, chunk.rs, selection.rs, comment.rs, layout.rs, table.rs
  syntax/
    CODE_MAP.md
    mod.rs, highlighter.rs
  theme/
    CODE_MAP.md
    mod.rs, default.rs

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

### Phase 1 (MVP - Markdown Review) ✅ COMPLETE

**Low uncertainty, mostly execution mode:**
- Few toys needed (egui selection, markdown rendering)
- Focus on production quality and refactoring
- CODE_MAP.md central artifact

**Key toys built:**
- `toy1_egui_selection` - Text selection with line precision
- `toy2_markdown_render` - Lazy rendering with cached heights
- `toy3_comment_ui` - Floating comment dialog

### Phase 2 (Polish and Usability)

**Medium uncertainty, mixed mode:**
- Toys for enhanced Markdown features (TOC, search, image preview)
- Diff view toys (side-by-side comparison)
- Performance optimization toys (very large documents)

### Phase 3 (Advanced Features)

**Medium uncertainty, focused toys:**
- Review template toys (structured metadata)
- Export format toys (HTML, PDF generation)
- Plugin system toys (WASM runtime, plugin API)

---

## North Star

**Toys are reconnaissance, not construction.**

Scout uncertain GUI patterns without production constraints. Focus: understanding rendering performance and discovering viable UX approaches. Result: validated patterns applied to `src/`, toys kept as reference artifacts.

When theory meets egui's immediate mode rendering or pulldown-cmark's event stream, update the theory.

---

## References

- **DDD.md**: Core methodology (https://github.com/dialecticianai/ddd-book/)
- **ROADMAP.md**: Mirror's 3-phase development plan
- **CLAUDE.md**: Mirror-specific development context and philosophy
