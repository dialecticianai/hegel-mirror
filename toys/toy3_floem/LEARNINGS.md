# toy3_floem - LEARNINGS

**Status:** Planning | **Framework:** floem v0.2.0 | **Goal:** Evaluate reactive UI paradigm for markdown review

---

## Mission

Explore floem's reactive signal-based architecture as an alternative to egui's immediate mode for the Hegel Mirror markdown review UI.

**Core question:** Can floem's reactive system + text editor provide better ergonomics for selection tracking and comment management than toy2's manual chunk system?

---

## Learning Goals

### 1. Reactive State Management
**Goal:** Understand signal-based reactivity vs toy2's manual state.

**Questions:**
- How do `RwSignal`s work for comment state?
- Does auto-subscription reduce boilerplate vs toy2's `&mut self.comments`?
- Can we model selection as a signal that drives comment UI?
- Performance of fine-grained updates vs egui's redraw-everything?

**Test:** Counter example → comment list that updates reactively

### 2. Text Rendering & Selection
**Goal:** Evaluate floem's text editor for markdown display with source position tracking.

**Critical questions:**
- Does `text_editor` module expose selection ranges (char offsets)?
- Can we map rendered text → source line:col like toy2's chunks?
- Is `rich_text` sufficient for markdown styling (bold, italic, headings)?
- Can we make text read-only for review (not editing)?

**Test:** Display markdown with styled headings/bold → capture click position

### 3. Custom View Composition
**Goal:** Build markdown chunk view with selectable regions.

**Questions:**
- Can we create custom `View` trait impl for markdown chunks?
- How to attach source positions (line:col) to rendered text spans?
- Is `virtual_stack` useful for large documents (like toy2's viewport culling)?
- Can we highlight selected text reactively?

**Test:** Render markdown → click text → show line:col in label (reactive)

### 4. Layout & Styling
**Goal:** Compare floem's flexbox layout to egui's manual positioning.

**Questions:**
- Is `v_stack` + `h_stack` easier than egui's `ui.horizontal()` / `ui.vertical()`?
- Does style inheritance simplify theming vs toy2's per-widget styles?
- Can we style code blocks with dark backgrounds like toy2?
- How does `scroll` compare to egui's `ScrollArea`?

**Test:** Layout like toy2 (scrollable markdown → comment list at bottom)

### 5. Comment UI Integration
**Goal:** Build comment input that syncs with selection state.

**Questions:**
- Can `text_input` widget appear conditionally on selection?
- Does signal-based state make comment submission cleaner?
- Can we validate toy2's "immediate commenting" vs "batched review" modes?
- Is the reactive model better for multi-file tabs (separate signals per file)?

**Test:** Select text → input appears → type comment → syncs to signal → displays in list

### 6. Performance Characteristics
**Goal:** Measure if reactive updates beat immediate mode for review UI.

**Benchmark:**
- Parse 1000-line markdown → render → scroll performance
- Add 50 comments → update latency
- Compare to toy2's egui viewport culling

**Hypothesis:** Floem's fine-grained updates may be faster for comment additions (only update comment list, not full redraw).

---

## Architecture Questions

### Position Tracking Strategy

**toy2 approach:** Parse markdown → chunks with baked-in line:col → click chunk → have position

**floem approach (TBD):**
- Option A: Same chunk model, but render chunks as floem views with signal state
- Option B: Use floem's text editor selection API, map char offsets → source line:col
- Option C: Hybrid - parse to chunks, but use reactive signals for selection state

**Decision criteria:** Which gives us exact source positions with less code?

### State Management Pattern

**toy2 state:** `App` struct owns `Vec<TextChunk>`, `Option<Selection>`, `Vec<Comment>`

**floem state options:**
- Option A: Same struct, but fields are `RwSignal<Vec<Comment>>`, etc.
- Option B: Flatten - each chunk has its own selection signal
- Option C: Global state store (like Redux) with signal selectors

**Decision criteria:** Does reactivity reduce boilerplate or add complexity?

---

## Success Metrics

**Must demonstrate:**
1. ✅ Markdown rendering with basic styling (headings, bold, italic)
2. ✅ Click text → capture position (needs line:col, not just char offset)
3. ✅ Reactive comment list (add comment → auto-updates UI)
4. ✅ Selection state management (clear on submit, persist across renders)

**Nice to have:**
5. Code block syntax highlighting (compare to toy2's syntect integration)
6. Image rendering (compare to toy2's lazy loading)
7. Multi-file tabs with separate comment signals

**Comparison deliverable:**
- Side-by-side: toy2 (egui) vs toy3 (floem) for same features
- Lines of code, complexity, ergonomics
- Performance (frame time, memory, compile time)

---

## Open Questions

### Markdown Parsing
- **Q:** Do we re-use toy2's `pulldown-cmark` → chunks pipeline?
- **A (hypothesis):** Yes, parsing strategy is independent of UI framework. Chunks still useful for position tracking.

### Text Editor API
- **Q:** Does floem's `text_editor` expose enough internals for read-only markdown review?
- **A (unknown):** Need to read `floem::views::text_editor` docs and examples.

### Signal Overhead
- **Q:** Is creating a signal per chunk overkill? (1000-line doc = ~500 chunks)
- **A (unknown):** Need to profile. May want single signal for selection state, not per-chunk.

### Integration with Hegel
- **Q:** Does floem's reactivity help with Hegel workflow integration?
- **A (hypothesis):** Signals could expose "review complete" state more cleanly than toy2's exit-on-close logic.

---

## Next Steps (When Ready)

### Phase 1: Hello World
1. Basic floem app with counter (validate reactive system)
2. Display static markdown text (no styling yet)
3. Add click handler to text → print position to console

### Phase 2: Markdown Rendering
4. Port toy2's markdown parser (chunks with line:col)
5. Render chunks as floem views with rich text styling
6. Implement selection state as `RwSignal<Option<Selection>>`

### Phase 3: Comment UI
7. Add text input that appears on selection
8. Store comments in `RwSignal<Vec<Comment>>`
9. Display comment list (reactive updates)

### Phase 4: Comparison
10. Implement same features as toy2 (code blocks, images)
11. Write LEARNINGS.md with findings
12. Create decision matrix: egui vs floem for Hegel Mirror

---

## Dependencies to Explore

From floem docs:
- `floem::views::text_editor` - Rich text editing/selection
- `floem::views::rich_text` - Styled text spans
- `floem::reactive::RwSignal` - Mutable reactive state
- `floem::views::virtual_stack` - Lazy loading for large lists
- `pulldown-cmark` (reuse from toy2) - Markdown parsing

---

## Risk Assessment

**High risk:**
- floem v0.2.0 is very new → API instability, limited examples
- Text editor API may not expose selection → position mapping
- Reactivity may add complexity without benefit for "render once, comment, exit" workflow

**Medium risk:**
- Learning curve for signals (new paradigm vs egui's immediate mode)
- 393-package dependency tree vs egui's lighter footprint
- Less mature ecosystem (fewer Stack Overflow answers)

**Low risk:**
- Can always fall back to toy2's egui approach if floem doesn't fit
- Parsing logic is reusable (already validated in toy2)

---

## Decision Framework

**Choose floem if:**
- Reactive updates feel cleaner for comment state management
- Text editor API provides good selection → position mapping
- Flexbox layout is significantly easier than egui
- Performance is comparable or better

**Stick with egui (toy2) if:**
- Position tracking requires too much floem-specific plumbing
- Signals add complexity without clear benefit
- Text editor is too heavyweight (we just need read-only display)
- Compilation time or binary size is significantly worse

---

## Meta Notes

**Remember:** This is a toy for exploration, not production code. Goal is to learn trade-offs, not build a complete app.

**Time box:** If we can't get basic position tracking working in ~2 hours, floem may not be the right fit for this use case.

**Document everything:** Capture "aha!" moments and "wtf?" moments equally. Both inform the final decision.
