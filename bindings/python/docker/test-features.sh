#!/usr/bin/env bash
# Run feature_test.py inside one Docker image per Linux wheel target, in both
# piped and pseudo-terminal mode, with real `rich` installed as a reference.
# Results: <out-dir>/<image>-<arch>-{pipe,tty}.json, then summarize.py merges them.
#
# Usage: bindings/python/docker/test-features.sh [fast-rich-version] [out-dir]
set -uo pipefail

VERSION="${1:-0.3.2}"
DIR="$(cd "$(dirname "$0")" && pwd)"
OUT="$(mkdir -p "${2:-$DIR/results}" && cd "${2:-$DIR/results}" && pwd)"
RICH_VERSION="${RICH_VERSION:-15.0.0}"

MATRIX=(
  "python:3.10-slim|linux/amd64"
  "python:3.14-slim|linux/amd64"
  "python:3.10-slim|linux/arm64"
  "python:3.14-slim|linux/arm64"
  "python:3.10-alpine|linux/amd64"
  "python:3.14-alpine|linux/amd64"
  "python:3.10-alpine|linux/arm64"
  "python:3.14-alpine|linux/arm64"
)

status=0
for entry in "${MATRIX[@]}"; do
  image="${entry%%|*}"
  plat="${entry##*|}"
  name="$(echo "${image}-${plat#linux/}" | tr ':/.' '---')"
  tag="fast-rich-test:${VERSION}-$(echo "${image}-${plat}" | tr ':/.' '---')"
  echo "################ ${image} (${plat}) ################"
  # Same no-Rust image as test-pypi-wheels.sh (cached after the first build).
  docker build --quiet --platform "$plat" --build-arg BASE_IMAGE="$image" \
    --build-arg FAST_RICH_VERSION="$VERSION" -t "$tag" "$DIR" >/dev/null || { status=1; continue; }
  docker run --rm --platform "$plat" -e TEST_IMAGE="${image} ${plat}" \
    -v "$DIR/feature_test.py:/t/feature_test.py:ro" -v "$OUT:/out" "$tag" sh -c "
      pip install -q --no-cache-dir rich==${RICH_VERSION} >/dev/null 2>&1 || { echo 'rich install failed'; exit 1; }
      ! command -v cargo && ! command -v rustc || { echo 'Rust found'; exit 1; }
      python /t/feature_test.py --timeout 60 --json /out/${name}-pipe.json | tail -1 &&
      python /t/feature_test.py --timeout 60 --tty --json /out/${name}-tty.json | tail -1
    " || status=1
done
exit "$status"
