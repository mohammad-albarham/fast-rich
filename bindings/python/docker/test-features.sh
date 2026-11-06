#!/usr/bin/env bash
# Run feature_test.py inside one Docker image per Linux wheel target, in both
# piped and pseudo-terminal mode, with real `rich` installed as a reference.
# Results: <out-dir>/<image>-<arch>-{pipe,tty}.json, then summarize.py merges them.
#
# Usage: bindings/python/docker/test-features.sh [fast-rich-version] [out-dir]
# Set WHEEL_DIR=/path/to/wheels to test locally built wheels instead of PyPI.
set -uo pipefail

VERSION="${1:-0.3.2}"
DIR="$(cd "$(dirname "$0")" && pwd)"
OUT="$(mkdir -p "${2:-$DIR/results}" && cd "${2:-$DIR/results}" && pwd)"
RICH_VERSION="${RICH_VERSION:-15.0.0}"
WHEEL_DIR="${WHEEL_DIR:-}"

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
  if [ -n "$WHEEL_DIR" ]; then
    # Local wheels: plain Python base image (no Rust), wheel installed offline.
    run_image="$image"
    install="pip install -q --no-cache-dir --no-index --find-links /wheels fast-rich >/dev/null 2>&1 || { echo 'wheel install failed'; exit 1; }"
    mounts=(-v "$(cd "$WHEEL_DIR" && pwd):/wheels:ro")
  else
    # Same no-Rust image as test-pypi-wheels.sh (cached after the first build).
    docker build --quiet --platform "$plat" --build-arg BASE_IMAGE="$image" \
      --build-arg FAST_RICH_VERSION="$VERSION" -t "$tag" "$DIR" >/dev/null || { status=1; continue; }
    run_image="$tag"
    install="true"
    mounts=()
  fi
  docker run --rm --platform "$plat" -e TEST_IMAGE="${image} ${plat}" ${mounts[@]+"${mounts[@]}"} \
    -v "$DIR/feature_test.py:/t/feature_test.py:ro" -v "$OUT:/out" "$run_image" sh -c "
      $install
      pip install -q --no-cache-dir rich==${RICH_VERSION} >/dev/null 2>&1 || { echo 'rich install failed'; exit 1; }
      ! command -v cargo && ! command -v rustc || { echo 'Rust found'; exit 1; }
      python /t/feature_test.py --timeout 60 --json /out/${name}-pipe.json | tail -1 &&
      python /t/feature_test.py --timeout 60 --tty --json /out/${name}-tty.json | tail -1
    " || status=1
done
exit "$status"
