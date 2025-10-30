# Refactor Definitions

This directory contains refactor definitions for the `refactor.pl` script.

## Usage

```bash
# Dry-run (always do this first!)
./scripts/refactor.pl --refactor=NAME --dry-run

# Apply refactor
./scripts/refactor.pl --refactor=NAME

# Reverse a refactor
./scripts/refactor.pl --refactor=NAME --reverse --dry-run
./scripts/refactor.pl --refactor=NAME --reverse
```

## Creating a Refactor Definition

Create a new file `scripts/refactors/your_name.pl` with this structure:

```perl
{
    name => 'your_name',

    description => 'What this refactor does',

    files => [
        'path/to/file1.rs',
        'path/to/file2.rs',
    ],

    # Forward transformation
    forward_pattern => qr/
        old \s+ code \s+ pattern
    /x,

    forward_replacement => 'new code',

    # Optional: Reverse transformation (makes refactor reversible)
    reverse_pattern => qr/
        new \s+ code \s+ pattern
    /x,

    reverse_replacement => 'old code',
}
```

## Tips

- **Always test with --dry-run first** to see what will change
- Use `/x` flag for readable regex patterns (whitespace ignored)
- Escape special regex chars: `\(`, `\)`, `\.`, etc.
- Use `\s+` for flexible whitespace matching
- Make refactors reversible when possible (define reverse_pattern)

## Example: test_helpers

See `test_helpers.pl` for a complete example that:
- Replaces ImageManager boilerplate with helper function (forward)
- Can restore the original boilerplate (reverse)
- Processes 27 test functions across 2 files

## Philosophy

Like database migrations:
- Each refactor is isolated and versioned
- Refactors are reversible (when possible)
- Dry-run mode prevents accidents
- Clear audit trail of transformations
