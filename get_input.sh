#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 2 ]; then
    echo "Usage: $0 <year> <day>" >&2
    exit 2
fi

YEAR=$1
DAY=$2
CACHE_DIR="${TMPDIR:-/tmp}/aoc_cache"
CACHE_FILE="$CACHE_DIR/${YEAR}_day${DAY}.txt"

if [ -z "${AOC_SESSION:-}" ]; then
    echo "Error: AOC_SESSION environment variable not set" >&2
    exit 1
fi

mkdir -p "$CACHE_DIR"

if [ -f "$CACHE_FILE" ]; then
    cat "$CACHE_FILE"
else
    curl "https://adventofcode.com/$YEAR/day/$DAY/input" \
        -H "Cookie: session=$AOC_SESSION" \
        -o "$CACHE_FILE"
    cat "$CACHE_FILE"
fi
