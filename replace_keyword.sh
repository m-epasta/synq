#!/bin/bash
set -euo pipefail

if [ $# -ne 2 ]; then
  echo "Usage: $0 <keyword> <replacement>" >&2
  exit 1
fi

keyword="$1"
replacement="$2"

# Verify ripgrep is installed
if ! command -v rg &>/dev/null; then
  echo "Error: ripgrep (rg) is not installed." >&2
  exit 1
fi

rg -l -0 -F -- "$keyword" | while IFS= read -r -d '' file; do
  echo "Processing: $file"
  tmpfile=$(mktemp)
  if rg --replace "$replacement" --passthru -F -- "$keyword" "$file" >"$tmpfile"; then
    mv "$tmpfile" "$file"
  else
    rm -f "$tmpfile"
    echo "Error processing $file" >&2
    exit 1
  fi
done

echo "Replacement complete."
