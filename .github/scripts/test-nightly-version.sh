#!/bin/bash
set -euo pipefail
SCRIPT="$(dirname "$0")/nightly-version.sh"
STAMP=202609051800

check() {
    local actual
    actual="$(bash "$SCRIPT" "$1" "$2" "$STAMP")"
    [[ "$actual" == "$3-nightly.$STAMP" ]] || {
        echo "expected $3-nightly.$STAMP, got $actual" >&2
        exit 1
    }
}
check 26.9.0 '' 26.9.0
check 26.9.0 v26.8.9 26.9.0
check 26.9.0 v26.9.0 26.9.1
check 26.9.0 v26.9.9 26.9.10
check 26.9.0 v26.10.0 26.10.1
for invalid in '' 26.9 26.09.0 nightly; do
    if bash "$SCRIPT" "$invalid" '' "$STAMP" >/dev/null 2>&1; then
        echo "accepted invalid manifest version: $invalid" >&2
        exit 1
    fi
done
if bash "$SCRIPT" 26.9.0 v26.8.1-merged "$STAMP" >/dev/null 2>&1; then
    echo "accepted a custom tag as a stable release" >&2
    exit 1
fi
echo "nightly version tests passed"
