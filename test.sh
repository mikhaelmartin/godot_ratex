#!/bin/bash
set -euo pipefail

# test.sh — Run Rust tests for the godot_ratex GDExtension
#
# Usage:
#   ./test.sh                          Run all tests (debug)
#   ./test.sh --release                Run all tests in release mode
#   ./test.sh --verbose                Show detailed test output (--nocapture)
#   ./test.sh --test <name>            Run only tests whose name contains <name>
#   ./test.sh --test parse --verbose   Combine filter + verbose
#   ./test.sh --release --test parse   Combine filter + release
#   ./test.sh --help                   Show this help

usage() {
    sed -n '/^# Usage:/,/^$/p' "$0" | sed 's/^# //'
    exit 0
}

FLAGS=""
TEST_FILTER=""
CARGO_EXTRA=""

while [[ "$#" -gt 0 ]]; do
    case "$1" in
        --release)
            FLAGS="$FLAGS --release"
            ;;
        --verbose)
            CARGO_EXTRA="$CARGO_EXTRA -- --nocapture"
            ;;
        --test)
            if [[ -z "${2:-}" || "$2" == -* ]]; then
                echo "Error: --test requires a test name filter argument."
                exit 1
            fi
            TEST_FILTER="$2"
            shift
            ;;
        -h|--help)
            usage
            ;;
        *)
            echo "Error: Unknown parameter '$1'"
            usage
            ;;
    esac
    shift
done

echo "========================================"
echo " godot_ratex — Test Runner"
echo "========================================"
echo " Profile : ${FLAGS#--release }${FLAGS:+release}${FLAGS:-debug}"
echo " Filter  : ${TEST_FILTER:-<all>}"
echo "========================================"
echo ""

CMD="cargo test $FLAGS"

if [[ -n "$TEST_FILTER" ]]; then
    CMD="$CMD -- $TEST_FILTER"
fi
if [[ -n "$CARGO_EXTRA" ]]; then
    CMD="$CMD $CARGO_EXTRA"
fi

echo "Running: $CMD"
echo ""

eval "$CMD"

echo ""
echo "Done."
