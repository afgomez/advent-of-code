#!/usr/bin/env bash

set -euo pipefail

# Usage: ./fetch_input.sh YEAR DAY
# Example: ./fetch_input.sh 2015 1

if [[ $# -ne 2 ]]; then
    echo "Usage: $0 YEAR DAY"
    echo "Example: $0 2015 1"
    exit 1
fi

YEAR="$1"
DAY="$2"

if [[ -z "${ADVENT_OF_CODE_SESSION:-}" ]]; then
    echo "Error: ADVENT_OF_CODE_SESSION environment variable is not set."
    echo "Please set it to your Advent of Code session cookie value."
    exit 1
fi

if ! [[ "$YEAR" =~ ^[0-9]{4}$ ]]; then
    echo "Error: YEAR must be a 4-digit number."
    exit 1
fi

if ! [[ "$DAY" =~ ^[0-9]{1,2}$ ]] || ((DAY < 1 || DAY > 25)); then
    echo "Error: DAY must be an integer between 1 and 25."
    exit 1
fi

PADDED_DAY=$(printf "%02d" "$DAY")
INPUT_DIR="inputs/${YEAR}"
INPUT_FILE="${INPUT_DIR}/${PADDED_DAY}.txt"

mkdir -p "$INPUT_DIR"

URL="https://adventofcode.com/${YEAR}/day/${DAY}/input"

echo "Fetching input for Year $YEAR Day $DAY..."

HTTP_STATUS=$(curl -sS -w "%{http_code}" --fail --cookie "session=${ADVENT_OF_CODE_SESSION}" "$URL" -o "$INPUT_FILE.tmp" || true)

if [[ "$HTTP_STATUS" != "200" ]]; then
    echo "Error: Failed to fetch input (HTTP status $HTTP_STATUS)."
    rm -f "$INPUT_FILE.tmp"
    exit 1
fi

mv "$INPUT_FILE.tmp" "$INPUT_FILE"
echo "Input saved to $INPUT_FILE"
