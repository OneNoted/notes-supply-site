#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mode="${1:-all}"
found=0

pattern_args=(
  -e '\bgh[pousr]_[A-Za-z0-9_]{20,}\b'
  -e '\bgithub_pat_[A-Za-z0-9_]{20,}\b'
  -e '-----BEGIN [A-Z0-9 ]*PRIVATE KEY-----'
  -e 'x-access-token:[^[:space:]@]+@github\.com'
)

tree_args=(
  -n
  -I
  -H
  -P
  --color=never
  --hidden
  --glob=!**/.git/**
  --glob=!**/.jj/**
  --glob=!**/node_modules/**
  --glob=!**/target/**
  --glob=!**/dist/**
  --glob=!**/output/**
  --glob=!**/local/**
)

scan_tree() {
  echo "Scanning working tree for GitHub secret material..."
  if rg "${tree_args[@]}" "${pattern_args[@]}" "$repo_root"; then
    echo "GitHub secret-like material detected in the working tree." >&2
    found=1
  fi
}

scan_history() {
  echo "Scanning git history for GitHub secret material..."
  if git -C "$repo_root" log -p --all -- . \
    ':(exclude).githooks/**' \
    ':(exclude)local/**' \
    ':(exclude)node_modules/**' \
    ':(exclude)target/**' \
    ':(exclude)dist/**' \
    ':(exclude)output/**' \
    | rg -n -P --color=never "${pattern_args[@]}"; then
    echo "GitHub secret-like material detected in git history." >&2
    found=1
  fi
}

case "$mode" in
  tree)
    scan_tree
    ;;
  history)
    scan_history
    ;;
  all)
    scan_tree
    scan_history
    ;;
  *)
    echo "usage: $0 [tree|history|all]" >&2
    exit 64
    ;;
esac

if [[ "$found" -ne 0 ]]; then
  exit 1
fi

echo "No GitHub secret-like material detected."
