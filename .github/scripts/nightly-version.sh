#!/bin/bash
# Usage: nightly-version.sh <manifest-version> <last-stable-tag-or-empty> [UTC-stamp]
# A fork may have no stable tags (or only custom prerelease tags). Its manifest
# is the floor; otherwise stay above the newest published stable version too.
set -euo pipefail

MANIFEST="${1:?manifest version is required}"
LAST="${2-}"
STAMP="${3:-$(date -u +%Y%m%d%H%M)}"
SEMVER='(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)'
[[ "$MANIFEST" =~ ^${SEMVER}$ ]] || { echo "invalid manifest version: $MANIFEST" >&2; exit 1; }
[[ "$STAMP" =~ ^[0-9]{12}$ ]] || { echo "invalid nightly timestamp: $STAMP" >&2; exit 1; }

NEXT="$MANIFEST"
if [[ -n "$LAST" ]]; then
    [[ "$LAST" =~ ^v${SEMVER}$ ]] || { echo "invalid stable tag: $LAST" >&2; exit 1; }
    BASE="${LAST#v}"
    AFTER_STABLE="${BASE%.*}.$(( ${BASE##*.} + 1 ))"
    NEXT="$(printf '%s\n' "$MANIFEST" "$AFTER_STABLE" | sort -V | tail -n1)"
fi
printf '%s-nightly.%s\n' "$NEXT" "$STAMP"
