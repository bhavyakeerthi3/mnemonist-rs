#!/usr/bin/env bash
# Hash committed upstream fixtures. Delegating to Node keeps this command
# byte-stable across checkout line-ending settings.
set -euo pipefail

cd "$(dirname "$0")"
node hash-tests.js
