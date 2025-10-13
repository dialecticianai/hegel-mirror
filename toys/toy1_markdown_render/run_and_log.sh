#!/bin/bash

# Run toy1_markdown_render and capture all debug output to a file
# Usage: ./run_and_log.sh [markdown_file]

MARKDOWN_FILE="${1:-test.md}"
LOG_FILE="debug_$(date +%Y%m%d_%H%M%S).log"

echo "Starting toy1_markdown_render with file: $MARKDOWN_FILE"
echo "Debug output will be saved to: $LOG_FILE"
echo "Press Ctrl+C or close the window when done."
echo ""

# Run cargo and capture all output (stdout + stderr) to log file
cargo run -- "$MARKDOWN_FILE" 2>&1 | tee "$LOG_FILE"

echo ""
echo "Process ended. Debug output saved to: $LOG_FILE"
