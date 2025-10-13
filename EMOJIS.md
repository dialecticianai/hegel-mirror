# Emoji Support Research

**Status**: Not implemented (emojis currently render in black/white)

**Date**: 2025-10-13

---

## Problem

egui's default text rendering only supports **monochrome emojis**. While egui fonts support ~1216 emojis, they render without color, making them look like black and white glyphs.

---

## Solution: egui-twemoji

Third-party crate that provides colored emoji support by rendering Twitter's Twemoji assets as images instead of font glyphs.

**Crate**: https://docs.rs/egui-twemoji
**GitHub Issue**: https://github.com/emilk/egui/issues/2551

---

## Implementation Plan

### 1. Add Dependencies

```toml
# Cargo.toml
[dependencies]
egui_extras = { version = "0.33", features = ["image"] }
egui-twemoji = "0.4"  # Check latest version
```

### 2. Install Image Loaders

In `src/main.rs`, during eframe app creation:

```rust
egui_extras::install_image_loaders(&cc.egui_ctx);
```

### 3. Replace Text Rendering

**Current approach**: `ui.label(text)`

**New approach**: Use `EmojiLabel` for text containing emojis

```rust
use egui_twemoji::EmojiLabel;

// Instead of:
ui.label("⭐ Hello 🚀");

// Use:
EmojiLabel::new("⭐ Hello 🚀").show(ui);
```

### 4. Files to Update

- **src/main.rs** - Install image loaders in app setup
- **src/rendering/text.rs** - Replace `ui.label()` with `EmojiLabel`
- **src/rendering/table.rs** - Update cell rendering to use `EmojiLabel`
- **src/rendering/code.rs** - Handle emojis in code comments (optional)

### 5. Emoji Detection Strategy

**Option A (Simple)**: Always use `EmojiLabel` for all text
- Pro: Simple, works everywhere
- Con: Slight performance overhead for non-emoji text

**Option B (Optimized)**: Detect emojis first, use `EmojiLabel` only when needed
- Pro: Better performance
- Con: Need unicode segmentation to detect emojis

**Recommended**: Start with Option A (simple), optimize later if needed.

---

## Tradeoffs

### Pros
- ✅ Full color emoji support
- ✅ Consistent rendering across platforms
- ✅ ~1216+ emojis supported via Twemoji
- ✅ Compatible with all egui widgets

### Cons
- ❌ Binary size increase (Twemoji assets bundled)
- ❌ Slight performance overhead (image rendering vs font glyphs)
- ❌ Additional dependency maintenance
- ❌ Need to modify all text rendering code

---

## Effort Estimate

**Total: ~1 hour**

- Dependencies & setup: 10 min
- Text rendering updates: 30 min
- Table cell updates: 15 min
- Testing: 15 min

**Complexity**: Medium (touches most rendering code)

---

## Test Case

```markdown
# Emoji Test

Regular text with emojis: 🎉 🚀 ✨ 🔥 💻 🎨

In a table:

| Name | Emoji |
|------|-------|
| Party | 🎉 |
| Rocket | 🚀 |

In **bold** 🎉 and *italic* 🚀 and `code` ✨
```

Test file saved at: `/tmp/test_emoji.md`

---

## References

- egui-twemoji docs: https://docs.rs/egui-twemoji
- egui emoji discussion: https://github.com/emilk/egui/discussions/1934
- Color emoji issue: https://github.com/emilk/egui/issues/2551
- Twemoji assets: https://github.com/twitter/twemoji

---

**Decision**: Deferred to future implementation. Research complete, implementation straightforward when needed.
