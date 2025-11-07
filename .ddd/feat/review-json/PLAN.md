# Review Storage Detection Implementation Plan

TDD plan for dual-backend review storage with automatic Hegel project detection.

---

## Overview

**Goal:** Route review writes to project-global `.hegel/reviews.json` in Hegel projects, preserve sidecar `.review.N` files for standalone usage.

**Scope:** Add hegel-cli dependency, implement project detection at startup, create JSON storage backend, route writes based on project type.

**Priorities:**
1. Backward compatibility - standalone mode unchanged
2. Clean abstraction - storage backend selection isolated
3. Test coverage - verify both modes work correctly

**Methodology:** TDD where it drives feature development. Test project detection, JSON serialization, and dual-mode write routing. Skip exhaustive edge cases.

---

## Step 1: Add Hegel Dependency

### Goal
Enable access to Hegel's project detection logic without reinventing the wheel.

### Step 1.a: Write Tests
No tests needed - dependency addition is verified by successful compilation.

### Step 1.b: Implement
- Add hegel dependency to Cargo.toml with path to adjacent repo
- Run cargo check to verify dependency resolves
- Verify compilation succeeds with new dependency

### Success Criteria
- Cargo.toml includes hegel dependency with correct path
- cargo check passes without errors
- No import conflicts with existing dependencies

**Commit:** `feat(storage): add hegel-cli dependency for project detection`

---

## Step 2: Implement Project Detection

### Goal
Detect Hegel projects at startup and store project root for later use.

### Step 2.a: Write Tests
Create unit tests for project type detection:
- Test that Hegel projects are detected when .hegel directory exists
- Test that standalone mode is used when no .hegel found
- Test that project root path is correctly stored for Hegel projects

Use tempfile to create test directories with and without .hegel folders.

### Step 2.b: Implement
- Create ProjectType enum in storage module (Hegel with root path, Standalone)
- Add detect_project_type function that calls hegel FileStorage find_project_root
- Handle Ok result as Hegel project, store returned path
- Handle Err result as Standalone project
- Document that detection happens once at startup

### Success Criteria
- ProjectType enum defined with two variants
- Detection function correctly identifies Hegel projects
- Detection function falls back to Standalone when appropriate
- Tests pass for both project types
- No panics on missing .hegel directory

**Commit:** `feat(storage): implement Hegel project detection`

---

## Step 3: Create Hegel JSON Storage Format

### Goal
Define data structures for project-global reviews.json format.

### Step 3.a: Write Tests
Create serialization roundtrip tests:
- Test HegelReviewEntry serializes to expected JSON structure
- Test filename-keyed map serializes correctly
- Test empty comments array for LGTM case
- Test multiple reviews per file append correctly

Test relative path computation:
- Test that absolute paths convert to relative from project root
- Test paths already relative remain unchanged

### Step 3.b: Implement
- Define HegelReviewEntry struct with comments array, timestamp, session_id
- Define type alias for filename map (HashMap of filename to Vec of HegelReviewEntry)
- Add serde derives for JSON serialization
- Implement function to compute relative path from project root to file
- Add function to read existing reviews.json or return empty map
- Add function to write reviews map atomically to reviews.json

### Success Criteria
- HegelReviewEntry struct matches spec format
- Serialization produces clean JSON matching spec example
- Relative path computation handles nested directories correctly
- Read function handles missing file gracefully
- Write function creates .hegel directory if missing
- Roundtrip tests pass for all cases

**Commit:** `feat(storage): add Hegel reviews.json format and serialization`

---

## Step 4: Route Storage Writes by Project Type

### Goal
Conditionally route review writes to appropriate backend based on detection.

### Step 4.a: Write Tests
Create integration tests for dual-mode storage:
- Test immediate mode writes to .hegel/reviews.json in Hegel project
- Test immediate mode writes to sidecar .review.N in standalone project
- Test batch mode writes to .hegel/reviews.json in Hegel project
- Test batch mode writes to sidecar .review.N in standalone project
- Test LGTM creates entry with empty comments in Hegel project
- Test multi-file reviews create separate entries per file in Hegel project

Use tempfile to simulate both project types during testing.

### Step 4.b: Implement
- Modify Document struct to store detected ProjectType
- Pass ProjectType to Document during construction in main.rs
- Update append_comment function to route based on ProjectType
- Update write_review function to route based on ProjectType
- Update write_lgtm function to route based on ProjectType
- For Hegel mode: read existing reviews.json, append entry, write back
- For standalone mode: preserve existing behavior unchanged
- Ensure relative paths used for Hegel filenames

### Success Criteria
- Document carries ProjectType information
- All storage functions check ProjectType before writing
- Hegel projects write to .hegel/reviews.json with correct format
- Standalone projects continue using .review.N files
- Multi-file reviews work in both modes
- LGTM handling correct in both modes
- Integration tests pass for all scenarios

**Commit:** `feat(storage): route review writes by project type`

---

## Out of Scope

- Migration tool for existing .review.N to .hegel/reviews.json
- UI indicators showing which mode is active
- Concurrent write safety (assume single mirror instance)
- Implementing --json stdout output flag
- Reading and displaying historical reviews from .hegel/reviews.json
