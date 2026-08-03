#!/usr/bin/env bash
# Hash original test files for Port Mortem kickoff verification.
# Portable (Linux/macOS) counterpart to scripts/hash-tests.ps1.
set -euo pipefail

cd "$(dirname "$0")/.."

sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

combined=""
for f in $(find tests/original -maxdepth 1 -name '*.js' | sort); do
  name="$(basename "$f")"
  hash="$(sha256_of "$f")"
  echo "$name $hash"
  combined="${combined}${name} ${hash}"$'\n'
done

manifest_hash="$(printf '%s' "$combined" | { command -v sha256sum >/dev/null 2>&1 && sha256sum | awk '{print $1}' || shasum -a 256 | awk '{print $1}'; })"

echo ""
echo "MANIFEST_SHA256 $manifest_hash"
