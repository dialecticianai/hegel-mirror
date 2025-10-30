# Mirror Markdown Feature Catalog

**Purpose**: Comprehensive catalog of all markdown features Mirror parses and renders

**Audience**: AI agents creating test fixtures, developers adding features

**Sources**: Mirror codebase (src/parsing/, src/rendering/, src/models/)

---

## Text Formatting (Inline)

**Bold** (`**text**` or `__text__`)
- Parsed: Event::Strong → sets `chunk.bold = true`
- Rendered: Font family "Bold" via text_builder.rs
- Coverage: ✅ test_bold.md

**Italic** (`*text*` or `_text_`)
- Parsed: Event::Emphasis → sets `chunk.italic = true`
- Rendered: Font family "Italic" via text_builder.rs
- Coverage: ✅ test_bold.md

**Bold + Italic** (`***text***` or `___text___`)
- Parsed: Event::Strong + Event::Emphasis (nested) → sets both flags
- Rendered: Font family "BoldItalic"
- Coverage: ✅ test_bold.md

**Inline code** (`` `code` ``)
- Parsed: Event::Code → sets `chunk.code = true`
- Rendered: Monospace font, pink color (theme.colors.inline_code), smaller size
- Coverage: ❌ **MISSING**

**Emoji** (Unicode emoji like 🚀)
- Parsed: Regular text, no special event
- Rendered: egui-twemoji converts to colored images via EmojiLabel
- Coverage: ✅ EMOJIS.md

---

## Headings

**H1-H6** (`# Heading` through `###### Heading`)
- Parsed: Event::Start(Heading) → sets `chunk.heading_level = Some(1..=6)`
- Rendered: Larger font sizes (32px down to 14px), heading color, bold styling
- Coverage: ❌ **MISSING systematic test**

**Constraints**:
- Heading text flows inline until newline
- No nesting (heading stops at line end)
- Font sizes: [32.0, 28.0, 24.0, 20.0, 16.0, 14.0]

---

## Block Elements

### Code Blocks

**Fenced code blocks** (` ```lang ... ``` `)
- Parsed: Event::Start(CodeBlock(Fenced)) → creates chunk with `code_block_lang`
- Rendered: Syntax highlighting via syntect, dark background, padding, rounded corners
- Languages: All syntect-supported (Rust, Python, JavaScript, etc.)
- Coverage: ✅ Tested in manual workflow

**Gotcha**: Language name must be on same line as opening fence
**Constraint**: Uses "base16-ocean.dark" theme (hardcoded in syntax/highlighter.rs)

### Tables

**GFM tables** (pipe-delimited with header)
- Parsed: Event::Start(Table) → collects rows, creates Table struct with alignments
- Rendered: egui::Grid with striped rows, alignment support (left/center/right)
- Coverage: ✅ tests/fixtures/tables.md

**Constraints**:
- First row = header (styled bold)
- Alignment row (`:---`, `:---:`, `---:`) sets per-column alignment
- Cell padding: 8px

**Gotcha**: Emoji in tables uses regular Label (not EmojiLabel) due to Grid layout constraints

### Images

**Standard markdown** (`![alt](path)`)
- Parsed: Event::Start(Image) → creates chunk with `image_path`
- Rendered: Loads from filesystem, displays at original size or constrained width
- Coverage: ❌ **MISSING basic test**

**HTML-enhanced** (`<p align="center"><img src="path" width="400"></p>`)
- Parsed: Event::Html → html.rs extracts src/alignment/width
- Rendered: Supports left/center/right alignment, width constraints
- Coverage: ✅ test_centered_image.md

**Constraints**:
- Image paths relative to markdown file location
- Supported formats: PNG, JPG, etc. (via `image` crate)
- Texture cache: HashMap<path, TextureHandle>
- Single-line selection on click

**Gotcha**: HTML parsing expects specific structure (p/img tags, quoted attributes)

---

## Lists

**Unordered lists** (`- item`, `* item`, `+ item`)
- Parsed: Event::Start(List) → text chunks with bullet detection
- Rendered: Bullets preserved in text, no special list styling
- Coverage: ❌ **MISSING**

**Ordered lists** (`1. item`)
- Parsed: Event::Start(List) with start number
- Rendered: Numbers preserved in text
- Coverage: ❌ **MISSING**

**Constraint**: Lists use inline_batcher.rs to prevent horizontal wrapping
- Bullet detection: starts_with_bullet() checks for "- ", "* ", "+ "
- Prevents batching list items into horizontal_wrapped layout

---

## Block Quotes

**Blockquotes** (`> quote`)
- Parsed: Event::Start(BlockQuote) → text chunks (no special flag)
- Rendered: Regular text (no visual distinction)
- Coverage: ❌ **MISSING**

**Limitation**: Currently renders as plain text, no indentation/styling

---

## Links

**Inline links** (`[text](url)`)
- Parsed: Event::Start(Link) → text extraction only
- Rendered: Plain text (no clickable links)
- Coverage: ❌ **MISSING**

**Limitation**: Link URLs discarded, only link text displayed

---

## Horizontal Rules

**Thematic breaks** (`---`, `***`, `___`)
- Parsed: Event::Rule → (not implemented)
- Rendered: Not supported
- Coverage: ❌ **NOT IMPLEMENTED**

---

## Special Handling

### Line Breaks

**Hard breaks** (`<br>`, two spaces + newline)
- Parsed: Event::HardBreak → creates separate chunk
- Rendered: Forces new line via `chunk.newline_after = true`
- Coverage: ✅ Implicit in all tests

**Soft breaks** (single newline)
- Parsed: Event::SoftBreak → whitespace in text
- Rendered: Flows inline unless at paragraph boundary
- Coverage: ✅ Implicit

### Paragraphs

**Paragraph spacing**
- Parsed: Event::End(Paragraph) → sets `chunk.newline_after = true`
- Rendered: Adds paragraph spacing (4px gap)
- Coverage: ✅ Implicit

---

## Rendering Architecture

### Inline Text Batching

**Pattern**: Consecutive inline chunks batch into horizontal_wrapped layout

**Batchable**:
- Regular text (no heading, no image, no code block, no table)
- No `newline_after` flag
- Doesn't start with list bullet

**Unbatchable**:
- Headings (stand alone)
- Images, code blocks, tables (block elements)
- List items (bullet detection)
- Chunks ending with `newline_after`

**Why**: Allows bold/italic text to flow naturally inline without line breaks

### Font System

**Families**:
- Proportional (default) - Inter Regular
- Bold - Inter Bold
- Italic - Inter Italic
- BoldItalic - Inter Bold Italic
- Monospace - System monospace (for code)

**Loaded**: main.rs embeds fonts via include_bytes!

---

## Position Tracking

**Byte ranges**: Every TextChunk has `byte_range: Range<usize>`

**Line/col tracking**:
- `line_start`, `col_start`, `line_end`, `col_end` (1-indexed)
- Computed via LineOffsets (O(log n) binary search)

**Purpose**: Review file comments include precise selection positions

---

## Edge Cases & Gotchas

**Nested formatting**: Bold + italic works (nested events set both flags)

**Emoji in tables**: Uses Label not EmojiLabel (Grid layout constraint)

**Image loading**: Synchronous from disk, cached in HashMap

**Syntax highlighting**: Falls back to plain text for unknown languages

**HTML parsing**: Very specific format required (html.rs uses string parsing, not full HTML parser)

**Viewport culling**: First render of chunk is slower (must measure), subsequent renders use cached height

---

## Test Coverage Gaps

### Critical Missing Tests

❌ **Inline code** - No test file for backtick code
❌ **Headings** - No systematic H1-H6 test
❌ **Standard images** - Only HTML-enhanced images tested
❌ **Lists** - No unordered/ordered list tests
❌ **Blockquotes** - Not tested
❌ **Links** - Not tested (though not clickable)

### Edge Cases Untested

🔶 Nested bold/italic edge cases (e.g., `**bold *and italic* bold**`)
🔶 Code blocks with unusual languages
🔶 Tables with complex alignment combinations
🔶 Images with missing files (error handling)
🔶 Very long lines (wrapping behavior)
🔶 Unicode edge cases beyond basic emoji

### Features Not Implemented

❌ Horizontal rules (`---`)
❌ Clickable links
❌ Blockquote styling (indentation, border)
❌ Nested lists
❌ Task lists (`- [ ]` / `- [x]`)

---

## Sources

- `src/parsing/parser.rs` - Main pulldown-cmark event processor
- `src/parsing/chunks.rs` - TextChunk creation helpers
- `src/parsing/html.rs` - HTML image parsing
- `src/models/chunk.rs` - TextChunk structure
- `src/rendering/chunk_renderer.rs` - Rendering trait system
- `src/rendering/text_builder.rs` - Text styling with fonts
- `src/rendering/inline_batcher.rs` - Horizontal text flow
- Existing test files: `test_bold.md`, `test_centered_image.md`, `tests/fixtures/*.md`

**Created**: October 2025 (Research mode)
**Last updated**: October 2025
