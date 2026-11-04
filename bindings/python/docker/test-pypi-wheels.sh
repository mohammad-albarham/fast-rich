#!/usr/bin/env bash
# Build one Docker image per Linux wheel target and run the smoke test in each.
# Usage: bindings/python/docker/test-pypi-wheels.sh [version]   (default 0.3.2)
set -uo pipefail

VERSION="${1:-0.3.2}"
DIR="$(cd "$(dirname "$0")" && pwd)"

# image | platform  -> wheel exercised
MATRIX=(
  "python:3.10-slim|linux/amd64"    # manylinux x86_64, oldest supported Python
  "python:3.14-slim|linux/amd64"    # manylinux x86_64, newest Python
  "python:3.10-slim|linux/arm64"    # manylinux aarch64
  "python:3.14-slim|linux/arm64"    # manylinux aarch64
  "python:3.10-alpine|linux/amd64"  # musllinux x86_64
  "python:3.14-alpine|linux/amd64"  # musllinux x86_64
  "python:3.10-alpine|linux/arm64"  # musllinux aarch64
  "python:3.14-alpine|linux/arm64"  # musllinux aarch64
)

results=()
status=0
for entry in "${MATRIX[@]}"; do
  image="${entry%%|*}"
  plat="${entry##*|}"
  tag="fast-rich-test:${VERSION}-$(echo "${image}-${plat}" | tr ':/.' '---')"
  echo
  echo "################ ${image} (${plat}) ################"
  if docker build --quiet --platform "$plat" \
       --build-arg BASE_IMAGE="$image" \
       --build-arg FAST_RICH_VERSION="$VERSION" \
       -t "$tag" "$DIR" >/dev/null \
     && docker run --rm --platform "$plat" "$tag"; then
    results+=("PASS  ${image}  ${plat}")
  else
    results+=("FAIL  ${image}  ${plat}")
    status=1
  fi
done

echo
echo "================ summary (fast-rich ${VERSION}) ================"
printf '%s\n' "${results[@]}"
exit "$status"
