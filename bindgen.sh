#!/usr/bin/env bash
set -exu

bindgen \
  --dynamic-loading="liblikwid" \
  --whitelist-function="^likwid_marker.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  --raw-line="#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]" \
  wrapper.h -- -I"$LIKWID_PATH/include" \
  > src/likwid.rs

