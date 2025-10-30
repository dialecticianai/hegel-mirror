# Edge Cases Test

This fixture validates unusual markdown combinations and edge cases.

---

## Nested Formatting Combinations

**Bold with *italic inside* bold**

*Italic with **bold inside** italic*

***Bold and italic together***

**Bold with `code inside` bold**

*Italic with `code inside` italic*

`Code with **bold attempt** inside` (bold should not work inside code)

---

## Very Long Lines (Wrapping Test)

This is a very long line of regular text that extends far beyond the normal window width to test how the text wrapping behavior works when you have extremely long sentences that need to flow across multiple lines in the markdown viewer application and we want to make sure it wraps properly without breaking the layout or causing horizontal scrolling issues.

**This is a very long line of bold text that extends far beyond the normal window width to test how bold text wrapping behavior works when you have extremely long sentences in bold formatting that need to flow across multiple lines.**

*This is a very long line of italic text that extends far beyond the normal window width to test how italic text wrapping behavior works when you have extremely long sentences in italic formatting that need to flow across multiple lines.*

---

## Empty Elements

**Bold with nothing inside:**

*Italic with nothing inside:*

`Empty code:`

---

## Multiple Spaces and Special Whitespace

This has    multiple    spaces    between    words.

This has		tabs		between		words.

This has both   spaces  and	tabs	mixed.

---

## Special Characters

Ampersand: &

Less than: <

Greater than: >

Quotes: "double" and 'single'

Brackets: [square] and (round) and {curly}

Asterisks without formatting: * and **

Underscores without formatting: _ and __

Backticks without code: ` and ``

---

## Mixed List Markers

- Dash item
* Asterisk item
+ Plus item
- Dash again

1. First numbered
2. Second numbered
- Now unordered
3. Back to numbered?

---

## Emoji Edge Cases

Single emoji: 🚀

Multiple emoji: 🚀 🎉 ✨ 💡 🔥

Emoji with text: This 🚀 has emoji 🎉 inline ✨

**Bold emoji:** 🚀🎉

*Italic emoji:* 🚀🎉

`Code emoji:` 🚀🎉

---

## Combined Inline Elements

**Bold** *italic* `code` regular **bold again** text.

This has **bold *and italic* nested** followed by `code` and more text.

Emoji 🚀 with **bold** and *italic* and `code` all together.
