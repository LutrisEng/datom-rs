#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
# SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
# SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

set -euxo pipefail

export PATH="$PATH:$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's|host: ||p')/bin"
export CARGO_INCREMENTAL="0"
export RUSTFLAGS="-Cinstrument-coverage"
export RUSTDOCFLAGS="-Cpanic=abort -Cinstrument-coverage -Zunstable-options --persist-doctests target/debug/doctestbins"
export LLVM_PROFILE_FILE="target/coverage/datom-%m.profraw"

rm -rf target

cargo test --verbose -p datom -- --nocapture
binaries="$( \
              for file in \
                $( \
                  RUSTFLAGS="-C instrument-coverage" \
                    cargo test -p datom --tests --no-run --message-format=json \
                      | jq -r "select(.profile.test == true) | .filenames[]" \
                      | grep -v dSYM - \
                ) \
                target/debug/doctestbins/*/rust_out; \
              do \
                [[ -x $file ]] && printf "%s %s " -object $file || true; \
              done \
            )"

llvm-profdata merge -sparse target/coverage/datom-*.profraw -o target/coverage/datom.profdata
llvm-cov report \
    --use-color --ignore-filename-regex='(/rustc|/.cargo/registry)' \
    --instr-profile=target/coverage/datom.profdata \
    $binaries \
    --Xdemangler=rustfilt
llvm-cov show \
    --use-color --ignore-filename-regex='(/rustc|/.cargo/registry)' \
    --instr-profile=target/coverage/datom.profdata \
    $binaries \
    --show-instantiations --show-line-counts-or-regions \
    --Xdemangler=rustfilt
llvm-cov export \
    --format=lcov --instr-profile=target/coverage/datom.profdata \
    --ignore-filename-regex='(/rustc|/.cargo/registry)' \
    --Xdemangler=rustfilt $binaries \
    > datom.lcov
