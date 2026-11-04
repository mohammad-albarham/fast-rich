# Installs fast-rich from PyPI into a stock Python image that has no Rust
# toolchain, then runs the smoke test. If PyPI had no matching wheel, pip
# would fall back to the sdist and the build would fail here (no cargo).
#
#   docker build --platform linux/arm64 \
#     --build-arg BASE_IMAGE=python:3.12-alpine \
#     --build-arg FAST_RICH_VERSION=0.3.2 \
#     -t fast-rich-test bindings/python/docker
#   docker run --rm fast-rich-test

ARG BASE_IMAGE=python:3.12-slim
FROM ${BASE_IMAGE}

ARG FAST_RICH_VERSION=0.3.2
ENV PIP_DISABLE_PIP_VERSION_CHECK=1 PIP_NO_CACHE_DIR=1 PYTHONUNBUFFERED=1

RUN if command -v cargo || command -v rustc; then echo "Rust found in base image" >&2; exit 1; fi \
 && pip install "fast-rich==${FAST_RICH_VERSION}"

COPY smoke_test.py /smoke_test.py
WORKDIR /tmp
CMD ["python", "/smoke_test.py"]
