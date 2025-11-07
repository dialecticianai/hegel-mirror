# syntax/

Syntax highlighting for code blocks using syntect.

## Structure

```
syntax/
├── mod.rs              Public export: SyntaxHighlighter
└── highlighter.rs      SyntaxHighlighter - wraps syntect, returns styled line ranges
```
