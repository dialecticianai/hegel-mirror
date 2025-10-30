# theme/

Typography, spacing, colors, and layout configuration system.

## Module Interface

### **mod.rs**
Theme struct aggregating Typography, Spacing, Colors, and Layout. Provides default_theme() accessor returning the default theme constant from default.rs.

## Theme Definition

### **default.rs**
Default theme constant (THEME) with clean, readable styling. Body text at 14px, six heading sizes (32-14px), code at 13px. Spacing includes paragraph gaps (4px), code block padding (10px), and table cell padding (8px). Colors use GitHub-style dark code backgrounds, professional text colors, and blue selection highlights. Layout enforces 900px max content width with 40px side margins for readable line length.
