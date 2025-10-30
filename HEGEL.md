# Using Hegel for Workflow Orchestration

**Hegel** orchestrates Dialectic-Driven Development through state-based workflows. Use it for structured development cycles, command guardrails, AST-aware code transformations, and metrics collection.

**Core principle:** Use when structure helps, skip when it doesn't. The user always knows best.

---

## Command Reference

All commands support `--help` for detailed options. Use `hegel <command> --help` for specifics.

**State directory override:** All commands accept `--state-dir <path>` flag or `HEGEL_STATE_DIR` env var to override default `.hegel/` location. Useful for testing, multi-project workflows, or CI/CD.

### Initialization

```bash
hegel init          # Smart detection: greenfield or retrofit workflow
hegel config list   # View all configuration
hegel config get <key>
hegel config set <key> <value>
```

**Config keys:**
- `code_map_style` - `monolithic` or `hierarchical` (default: hierarchical)
- `use_reflect_gui` - Auto-launch review GUI: `true` or `false` (default: true)

**Init workflows:**
- **Greenfield** (no code): Creates CLAUDE.md, VISION.md, ARCHITECTURE.md, initializes git
- **Retrofit** (existing code): Analyzes structure, creates CODE_MAP.md, integrates DDD patterns

### Meta-Modes & Workflows

```bash
hegel meta <learning|standard>  # Declare meta-mode (required first step)
hegel meta                      # View current meta-mode

hegel start <workflow> [node]   # Load workflow (optionally at specific node)
hegel status                    # Show current state
hegel next                      # Advance to next phase (auto-infers completion claim)
hegel restart                   # Return to SPEC phase (restart cycle, keep same workflow)
hegel repeat                    # Re-display current prompt
hegel abort                     # Abandon workflow entirely (required before starting new one)
hegel reset                     # Clear all state
```

**Meta-modes:**
- `learning` - Research ↔ Discovery loop (starts with research)
- `standard` - Discovery ↔ Execution (starts with discovery)

**Workflows:**
- `init-greenfield` - CUSTOMIZE_CLAUDE → VISION → ARCHITECTURE → GIT_INIT (new projects)
- `init-retrofit` - DETECT_EXISTING → CODE_MAP → CUSTOMIZE_CLAUDE → VISION → ARCHITECTURE → GIT_COMMIT (existing projects)
- `research` - PLAN → STUDY → ASSESS → QUESTIONS (external knowledge gathering)
- `discovery` - SPEC → PLAN → CODE → LEARNINGS → README (toy experiments)
- `execution` - Production-grade rigor with code review phase
- `minimal` - Simplified for quick iterations

**Starting at custom nodes:**
```bash
# Start at default beginning
hegel start discovery           # Starts at 'spec' node

# Start at specific node (skip earlier phases)
hegel start discovery plan      # Start directly at plan phase
hegel start execution code      # Start directly at code phase
```

**Custom start nodes are useful for:**
- Resuming interrupted workflows
- Testing specific workflow phases
- Skipping phases you've already completed manually

**What happens:**
- `hegel start` prints the first phase prompt with embedded guidance
- `hegel start <workflow> <node>` starts at specified node (validates node exists)
- `hegel next` advances and prints the next phase prompt - **follow these instructions**
- `hegel repeat` re-displays current prompt if you need to see it again
- `hegel restart` returns to SPEC phase (same workflow, fresh cycle)
- `hegel abort` abandons workflow entirely (required before starting different workflow)

**Guardrails:**
- Cannot start new workflow while one is active → run `hegel abort` first
- Invalid start node returns error with list of available nodes
- Prevents accidental loss of workflow progress

### Code Operations

```bash
hegel astq [options] [path]     # AST-based search/transform (wraps ast-grep)
```

**Critical:** Use `hegel astq --help` for pattern syntax and examples. ALWAYS prefer astq over grep/rg for code search (AST-aware, ignores comments/strings, explicit "no matches" feedback).

### Document Review

```bash
hegel reflect <file.md> [files...]      # Launch Markdown review GUI
hegel reflect <file.md> --out-dir <dir> # Custom output location
```

Reviews saved to `.ddd/<filename>.review.N` (JSONL format). Read with `cat .ddd/SPEC.review.1 | jq -r '.comment'`.

### Command Guardrails

```bash
hegel git <args>        # Git with safety checks and audit logging
hegel docker <args>     # Docker with safety checks and audit logging
```

Configuration: `.hegel/guardrails.yaml` (see example below). All invocations logged to `.hegel/command_log.jsonl`.

### Metrics

```bash
hegel top               # Real-time TUI dashboard (4 tabs: Overview, Phases, Events, Files)
hegel analyze           # Static summary (tokens, activity, workflow graph, per-phase metrics)
hegel hook <event>      # Process Claude Code hook events (stdin JSON)
```

Dashboard shortcuts: `q` (quit), `Tab` (switch tabs), `↑↓`/`j`/`k` (scroll), `g`/`G` (top/bottom), `r` (reload).

---

## When to Use Workflows

**Use DDD workflows for:**
- Hard problems requiring novel solutions
- Complex domains where mistakes are expensive
- Learning-dense exploration

**Skip workflows for:**
- Straightforward implementations agents handle autonomously
- Simple CRUD or routine features
- User hasn't requested structured methodology

**When in doubt:** Check `hegel status` at session start. If no active workflow and user hasn't requested structure, proceed without Hegel orchestration.

---

## Integration Patterns

### Session Start

```bash
hegel meta              # Check meta-mode
hegel status            # Check active workflow
# If workflow active and relevant, continue with `hegel next`
# If user requests structure but no workflow, run `hegel meta <mode>`
```

### During Development

```bash
hegel astq -p 'pattern' src/        # AST-aware code search (NOT grep)
hegel git add . && hegel git commit # Safe git ops in workflows
hegel top                           # Monitor metrics
```

### Advancing Workflow

```bash
hegel next              # Completed current phase (infers happy-path claim)
hegel restart           # Return to SPEC phase
hegel abort             # Abandon workflow entirely
```

### Document Review

```bash
hegel reflect SPEC.md
# User reviews in GUI, submits
cat .ddd/SPEC.review.1 | jq -r '.comment'  # Read feedback
```

---

## State Files

```
.hegel/
├── state.json          # Current workflow (def, node, history, session metadata)
├── metamode.json       # Meta-mode declaration
├── config.toml         # User configuration
├── hooks.jsonl         # Claude Code events (tool usage, file mods, timestamps)
├── states.jsonl        # Workflow transitions (from/to, mode, workflow_id)
├── command_log.jsonl   # Wrapped command invocations (success/failure, blocks)
└── guardrails.yaml     # Command safety rules (patterns, reasons)
```

**JSONL format:** One JSON object per line (newline-delimited)
**Atomicity:** `state.json` uses atomic writes (write temp, rename)
**Correlation:** `workflow_id` (ISO 8601 timestamp) links hooks/states/transcripts

---

## Example Guardrails Configuration

`.hegel/guardrails.yaml`:

```yaml
git:
  blocked:
    - pattern: "reset --hard"
      reason: "Destructive: permanently discards uncommitted changes"
    - pattern: "clean -fd"
      reason: "Destructive: removes untracked files/directories"
    - pattern: "commit.*--no-verify"
      reason: "Bypasses pre-commit hooks"
    - pattern: "push.*--force"
      reason: "Force push can overwrite remote history"

docker:
  blocked:
    - pattern: "rm -f"
      reason: "Force remove containers blocked"
    - pattern: "system prune -a"
      reason: "Destructive: removes all unused containers, networks, images"
```

---

## Error Handling

| Error | Solution |
|-------|----------|
| "No workflow loaded" | `hegel start <workflow>` |
| "Cannot start workflow while one is active" | `hegel abort` then `hegel start <workflow>` |
| "Stayed at current node" (unexpected) | Check `hegel status`, verify not at terminal node, use `hegel restart` |
| "⛔ Command blocked by guardrails" | Review reason, edit `.hegel/guardrails.yaml`, or find alternative |

---

## Best Practices

**DO:**
- ✅ Check `hegel status` at session start
- ✅ Use `hegel astq` for code search (NOT grep/rg - AST-aware)
- ✅ Use `hegel git` for git operations in structured workflows
- ✅ Preview `astq` transformations before applying
- ✅ Read review files after `hegel reflect`
- ✅ Defer to `hegel <command> --help` for detailed syntax

**DON'T:**
- ❌ Start workflow if user hasn't requested structure
- ❌ Skip guardrails with raw `git` commands
- ❌ Use `astq --apply` without previewing
- ❌ Ignore workflow prompts (contain phase-specific guidance)
- ❌ Reset workflow without user confirmation
- ❌ Use grep/rg for code search (use `hegel astq`)

---

## Quick Reference

```bash
# Initialization
hegel init
hegel config list|get|set <key> [<value>]

# Meta-mode (required before workflows)
hegel meta <learning|standard>
hegel meta

# Workflows
hegel start <discovery|execution|research|minimal>
hegel next|restart|abort|repeat|status|reset

# Commands
hegel git <args>
hegel docker <args>

# Code
hegel astq [options] [path]     # See: hegel astq --help

# Review
hegel reflect <files...>

# Metrics
hegel top
hegel analyze
```

---

**For detailed command syntax, always use:** `hegel <command> --help`

