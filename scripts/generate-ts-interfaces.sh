#!/usr/bin/env bash
set -euo pipefail

# Script to generate TypeScript interfaces from the Rust `shared_types` crate using ts-rs.
# Run this from the repo root.

echo "[staxmedos] Generating TypeScript interfaces from Rust (ts-rs)..."

# Ensure Rust toolchain is available
if ! command -v cargo > /dev/null; then
  echo "cargo not found in PATH"
  exit 1
fi

# Run the test target that triggers ts-rs generation (project uses cargo test -p shared_types per README)
cargo test -p shared_types

# Show git status so we can see generated file changes in CI logs
if [ -n "$(git status --porcelain)" ]; then
  echo "[staxmedos] Generated files changed:"
  git --no-pager status --porcelain
  git --no-pager diff --name-only
else
  echo "[staxmedos] No generated changes detected"
fi
