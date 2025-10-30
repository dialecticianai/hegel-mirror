# src/

Root source directory for Mirror - ephemeral Markdown review UI.

## Entry Points

### **main.rs**
Binary entry point with CLI parsing (clap), font loading (Inter family with bold/italic), and egui initialization. Loads multiple markdown files into Document structs and launches the eframe application.

### **lib.rs**
Library exports for testing. Re-exports commonly used types from models, parsing, rendering, storage, and syntax modules.

### **app.rs**
Main application state (MarkdownReviewApp) implementing eframe::App trait. Orchestrates UI rendering with multi-file tab support, per-document LGTM approval flow, dual review modes (immediate/batched), and coordinates between rendering, selection, and storage systems.

### **storage.rs**
Review file persistence with JSONL format. Handles monotonic sequence numbering (.review.1, .review.2, etc.), immediate mode (append_comment), batched mode (write_review), and LGTM approval writes.

### **image_manager.rs**
Centralized image loading with two-phase strategy: metadata (dimensions) loaded during parsing for accurate viewport culling, textures loaded lazily on first render. Caches both metadata and GPU textures. Resolves relative paths against document base_path.

## Subdirectories

### **models/**
Data structures and types. See models/CODE_MAP.md.

### **parsing/**
Markdown parsing into positioned chunks. See parsing/CODE_MAP.md.

### **rendering/**
Trait-based rendering system with viewport culling and lazy loading. Implements ChunkRenderer trait pattern (Strategy pattern) for extensible chunk type handling. See rendering/CODE_MAP.md.

### **syntax/**
Syntax highlighting for code blocks via syntect. See syntax/CODE_MAP.md.

### **theme/**
Typography, spacing, colors, and layout configuration. See theme/CODE_MAP.md.
