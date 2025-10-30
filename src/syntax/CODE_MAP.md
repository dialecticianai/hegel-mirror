# syntax/

Syntax highlighting for code blocks using syntect.

## Module Interface

### **mod.rs**
Public export: SyntaxHighlighter.

## Implementation

### **highlighter.rs**
SyntaxHighlighter wrapping syntect's SyntaxSet and ThemeSet. Loads default syntax definitions (newlines variant) and themes. Provides highlight_code method returning styled line ranges (Vec<Vec<(Style, String)>>) for rendering. Uses "base16-ocean.dark" theme. Falls back to plain text for unknown languages.
