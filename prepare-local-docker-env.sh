#!/usr/bin/env bash

# Add Gum Program Library

CWD=$(pwd)
mkdir programs

mkdir gum_program_library || true
curl -LkSs https://api.github.com/repos/gumhq/gpl/tarball | tar -xz --strip-components=1 -C ./gum_program_library

pushd gum_program_library/programs/gpl_core
  cargo build-bpf --bpf-out-dir ./here
  mv ./here/gpl_core.so $CWD/programs/CDDMdCAWB5AXgvEy7XJRggAu37QPG1b9aJXndZoPUkkm.so
popd

pushd gum_program_library/programs/gpl_compression
  cargo build-bpf --bpf-out-dir ./here
  mv ./here/gpl_compression.so $CWD/programs/41kNwkQ9jESNYZJyAA1ENscQfx7vfkEf6uetVSFmfyaW.so
popd

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
rm -rf gum_program_library
