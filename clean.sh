#!/bin/sh

cargo clean -p \
    $(cargo tree --no-dedupe --prefix none \
      | cut -d' ' -f1 \
      | sort -u \
      | grep -v "^\
             $(toml get Cargo.toml package.name \
              | sed s/\"//g)\$" \
      | tr '\n' ' ' \
      | sed 's/ / -p /g' \
      | rev \
      | cut -c4- \
      | rev \
    )