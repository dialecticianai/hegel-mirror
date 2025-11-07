# Review Storage Detection Specification

Automatic Hegel project detection with dual storage backends for review persistence.

---

## Overview

**What it does:** Detects Hegel projects and routes review writes to project-global `.hegel/reviews.json` instead of per-file sidecar `.review.N` files.

**Key principles:**
- Zero configuration - detection is automatic
- Backwards compatible - non-Hegel projects unchanged
- Single source of truth - `.hegel/reviews.json` is project-global singleton
- Reuse Hegel's detection logic - depend on hegel-cli, don't reinvent

**Scope:** Add hegel-cli dependency, detect project type at launch, route storage writes to appropriate backend.

**Integration context:** Mirror already reads `HEGEL_SESSION_ID` env var and writes `.review.N` files via `src/storage.rs`. This adds project detection and alternate storage path.

---

## Data Model

### Existing Types (MODIFIED)

**src/storage.rs::ReviewComment** - No changes needed, already serializable

**src/storage.rs functions** - Add conditional routing:
- `append_comment()` - route to Hegel JSON or sidecar JSONL based on detection
- `write_review()` - route to Hegel JSON or sidecar JSONL based on detection

### New Types

**ProjectType enum** (src/storage.rs or new src/project_detection.rs):
```rust
enum ProjectType {
    Hegel { root: PathBuf },  // .hegel/ detected, store root path
    Standalone,                // no .hegel/, use sidecar files
}
```

**HegelReviewEntry struct** (for `.hegel/reviews.json`):
```rust
struct HegelReviewEntry {
    comments: Vec<ReviewComment>,
    timestamp: String,         // ISO 8601
    session_id: Option<String>,
}
```

**HegelReviewsFile** (for `.hegel/reviews.json` read/write):
```rust
// In-memory representation
HashMap<String, Vec<HegelReviewEntry>>
// where String = relative path from project root
```

---

## Core Operations

### 1. Project Detection

**Trigger:** On startup, before loading documents

**Behavior:**
- Call hegel-cli API to detect if current working directory is within a Hegel project
- If detected: store project root path
- If not: use standalone mode

**Integration:** Use hegel-cli's existing project detection logic (exact API TBD - depends on hegel-cli exports)

### 2. Review Write (Hegel Projects)

**Trigger:** Same as current - immediate mode comment, batch submit, LGTM click

**Behavior:**
- Compute relative path from project root to reviewed file
- Read existing `.hegel/reviews.json` (or create empty map)
- Append new `HegelReviewEntry` to filename's array
- Write atomically back to `.hegel/reviews.json`

**Format:**
```json
{
  "src/SPEC.md": [
    {
      "comments": [
        {
          "timestamp": "2025-10-13T02:45:00Z",
          "session_id": "abc123",
          "file": "src/SPEC.md",
          "selection": {"start": {"line": 15, "col": 0}, "end": {"line": 18, "col": 42}},
          "text": "selected text snippet",
          "comment": "This constraint needs clarification"
        }
      ],
      "timestamp": "2025-10-13T02:45:00Z",
      "session_id": "abc123"
    }
  ],
  ".ddd/feat/foo/PLAN.md": [
    {
      "comments": [],
      "timestamp": "2025-10-13T03:00:00Z",
      "session_id": "abc123"
    }
  ]
}
```

**LGTM handling:** Write entry with empty `comments` array (signals approval)

### 3. Review Write (Standalone Projects)

**Behavior:** No changes - continue using current `.review.N` sidecar files in same directory as reviewed file (or `--out-dir` if specified)

### 4. CLI Flag Behavior

**`--out-dir`:** Ignored in Hegel projects (always use `.hegel/reviews.json`), respected in standalone mode

**`--json`:** (Out of scope for this feature - flag exists but not implemented yet)

**`--headless`:** No changes - continues to no-op

---

## Test Scenarios

### Simple: Hegel Project Detection

**Setup:** Initialize git repo with `.hegel/` directory

**Action:** Launch `mirror SPEC.md`

**Expected:** Project detected as Hegel, review writes go to `.hegel/reviews.json`

### Complex: Multi-file Review in Hegel Project

**Setup:** Hegel project with `src/SPEC.md` and `.ddd/PLAN.md`

**Action:**
1. Launch `mirror src/SPEC.md .ddd/PLAN.md`
2. Add comment to SPEC.md
3. Add comment to PLAN.md
4. Submit review

**Expected:**
- `.hegel/reviews.json` created with both filenames as keys
- Paths are relative: `"src/SPEC.md"` and `".ddd/PLAN.md"`
- Each has one review entry with respective comments

### Simple: Standalone Project

**Setup:** Directory with no `.hegel/` folder

**Action:** Launch `mirror README.md`, add comment, submit

**Expected:** `README.review.1` created in same directory (current behavior preserved)

### Error: Missing .hegel Directory

**Setup:** Hegel detection returns project root, but `.hegel/` doesn't exist

**Action:** Launch mirror, submit review

**Expected:** Create `.hegel/reviews.json` (graceful creation like current `.ddd/` behavior)

---

## Success Criteria

- Hegel-cli added as Cargo dependency
- `cargo build` succeeds
- `cargo test` passes
- Hegel project detection implemented using hegel-cli API
- Review writes route to `.hegel/reviews.json` when Hegel detected
- Review writes route to sidecar `.review.N` when standalone
- Relative paths computed correctly from project root
- JSON structure matches specified format
- LGTM writes empty comments array to Hegel JSON
- Standalone mode behavior unchanged (backward compatible)

---

## Out of Scope

- Implementing `--json` stdout output (separate feature)
- Migration tool for existing `.review.N` files to `.hegel/reviews.json`
- UI changes or indicators showing project type
- Diff view or historical review display in UI
- Review file cleanup or rotation
- Validation of review file integrity
- Concurrent write safety (assume single mirror process per project)
