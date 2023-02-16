#!/usr/bin/env bash

CWD=$(pwd)
mkdir programs
mkdir solana_program_library || true
curl -LkSs https://api.github.com/repos/solana-labs/solana-program-library/tarball | tar -xz --strip-components=1 -C ./solana_program_library
tar -zxf -C /solana_program_library solana-program-library.tar.gz
pushd solana_program_library/account-compression/programs/account-compression
  cargo build-bpf --bpf-out-dir ./here
  mv ./here/spl_account_compression.so $CWD/programs/cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK.so
popd

pushd solana_program_library/account-compression/programs/noop
  cargo build-bpf --bpf-out-dir ./here
  mv ./here/spl_noop.so $CWD/programs/noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV.so
popd

pushd solana_program_library
  rm -rf Cargo.toml
popd

rm -rf solana_program_library
