#!/bin/bash

cd "$(dirname "$0")/../libdeflate-sys/" || exit 1
if ! [ -r "libdeflate/libdeflate.h" ]; then
    git submodule update --init || exit $?
fi

# Convert comments to doc comments, which bindgen will pick up and include in the rust bindings
sed 's_/\*$_/**_' libdeflate/libdeflate.h > libdeflate/binding_libdeflate.h
trap 'rm -f libdeflate/binding_libdeflate.h' EXIT

bindgen \
    --allowlist-item '(?i-u:libdeflate).*' \
    --allowlist-var '(?i-u:libdeflate).*' \
    --no-layout-tests  \
    --use-core \
    --rust-target 1.64 \
    --generate-inline-functions \
    --merge-extern-blocks \
    --no-derive-debug \
    -o src/bindings.rs \
    libdeflate/binding_libdeflate.h