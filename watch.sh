#!/bin/bash
# set -euxo pipefail
set -euo pipefail
cd `/usr/bin/dirname $0`

trap "final; exit 1" 2

function final {
  echo "Ctrl+C pushed."
}

dir=$(greadlink -f .)

while true; do
  set +e
  source ./build.sh
  set -e

  echo "build succeeded! $(date -Iseconds)"

  out=$(fswatch \
    --one-event \
    --recursive \
    --exclude='.*' \
    --include=$dir'/.*/src/.*\.rs$' \
    .)

  if [ -z "$out" ]; then
    break  
  fi
done

