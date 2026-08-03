#!/bin/bash -p
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd -P)"
exec "$ROOT_DIR/scripts/hepta-ui-release-verifier-self-test.sh" --profile accessibility
