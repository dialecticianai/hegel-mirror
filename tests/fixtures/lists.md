# Lists Test

This fixture validates list rendering with bullets and numbers preserved.

---

## Unordered Lists

### Dash Syntax
- First item
- Second item
- Third item

### Asterisk Syntax
* Alpha
* Beta
* Gamma

### Plus Syntax
+ One
+ Two
+ Three

---

## Ordered Lists

### Basic Numbered List
1. First step
2. Second step
3. Third step

### Starting from Different Number
5. Fifth item
6. Sixth item
7. Seventh item

---

## Nested Lists

- Outer item 1
  - Nested item 1.1
  - Nested item 1.2
- Outer item 2
  - Nested item 2.1
  - Nested item 2.2

---

## Mixed Content

- List item with `inline code`
- List item with **bold text**
- List item with *italic text*

1. Numbered item with `code`
2. Numbered item with **bold**
3. Numbered item with *italic*

---

## Long List Items

- This is a very long list item that extends beyond the normal window width to test how list items wrap when they contain a lot of text content that needs to flow to multiple lines
- Short item
- Another very long list item to validate consistent wrapping behavior across multiple list entries with extended text content

1. This is a numbered list item with very long text that should wrap to multiple lines while maintaining proper indentation and alignment with the number marker
2. Short numbered item
3. Another long numbered item to test wrapping consistency
