#!/usr/bin/env bash
# One-time bootstrap: creates the OpenFGA store and loads the authorization
# model from scripts/model.fga.yaml.
#
# Requires: fga CLI (available in the nix devShell via `nix develop`)
# Run after `podman compose up -d` and copy the printed IDs into .env.
set -euo pipefail

OPENFGA_URL="${OPENFGA_URL:-http://localhost:8080}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MODEL_FILE="$SCRIPT_DIR/../libraries/authz/fga/model.fga.yaml"

if ! command -v fga &>/dev/null; then
  echo "error: fga CLI not found — run 'nix develop' to enter the dev shell" >&2
  exit 1
fi

echo "Using OpenFGA at $OPENFGA_URL"

# Create store and capture its ID
STORE_JSON=$(fga store create --name library --api-url "$OPENFGA_URL")
STORE_ID=$(echo "$STORE_JSON" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
echo "Created store: $STORE_ID"

# Load the authorization model from the YAML definition
fga store import \
  --api-url "$OPENFGA_URL" \
  --store-id "$STORE_ID" \
  --file "$MODEL_FILE"
echo "Loaded authorization model from $MODEL_FILE"

# Retrieve the active model ID
MODEL_JSON=$(fga model get --api-url "$OPENFGA_URL" --store-id "$STORE_ID")
MODEL_ID=$(echo "$MODEL_JSON" | grep -o '"authorization_model_id":"[^"]*"' | cut -d'"' -f4)
echo "Active model: $MODEL_ID"

echo ""
echo "Add these to your .env:"
echo "  OPENFGA_STORE_ID=$STORE_ID"
echo "  OPENFGA_MODEL_ID=$MODEL_ID"
