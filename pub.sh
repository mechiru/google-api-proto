#!/usr/bin/env bash
set -eu

cd $(dirname ${BASH_SOURCE:-$0})

echo "Update git submodule"
git submodule update --remote --checkout xtask/proto

echo "Run xtask clean generate"
cargo xtask clean generate

if [ $(git diff HEAD --name-only | wc -l) -eq 0 ]; then
  echo "No changes!"
  exit 0
fi

echo "Run xtask test"
cargo xtask test

if [ ${CI:-false} ]; then
  git config --local user.name "mechiru"
  git config --local user.email "$(git config user.name)@users.noreply.github.com"
fi

if [ $(git diff HEAD --name-only xtask/proto | wc -l) -gt 0 ]; then
  git add xtask/proto
  git commit -m "xtask: update submodule googleapis/googleapis"
fi

if [ $(git diff HEAD --name-only google-api-proto | wc -l) -gt 0 ]; then
  git add google-api-proto
  git commit -m "google-api-proto: regenerate code"
fi

echo "Sync with the origin repository"
git push origin master

echo "Publish to crates.io"
cargo release \
      --execute \
      --no-confirm \
      --package google-api-proto \
      --token "$CARGO_REGISTRY_TOKEN" \
      --verbose \
      alpha
