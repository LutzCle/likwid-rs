#!/usr/bin/env bash
set -exu

bindgen \
  --dynamic-loading="liblikwid" \
  --whitelist-function="^likwid_marker.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  wrapper.h -- -I"$LIKWID_PATH/include" \
  > src/likwid.rs

