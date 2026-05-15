#!/usr/bin/env bash
# Build OCI images for all services via Nix and load them into podman.
# Run from the repository root (or any subdirectory — the script resolves the root itself).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

SERVICES=(user-service book-service review-service)

for svc in "${SERVICES[@]}"; do
  echo "==> building $svc image"
  nix build --no-write-lock-file ".#${svc}-image" --out-link "result-${svc}"
  echo "==> loading $svc into podman"
  podman load < "result-${svc}"
  rm -f "result-${svc}"
done

echo "==> done"
