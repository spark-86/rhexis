#!/usr/bin/env bash

# make sure the dir is there
mkdir -p ./rhexis-startup/src/lattice_transforms

echo "📦 Packing RHP: transform_lattice_scope_cache_lookup"
./target/debug/rhp pack \
  --plugin-type transform \
  --descriptor-path ./packages/transforms/transform_lattice_scope_cache_lookup/src/rhp.json \
  --code-path ./target/debug/libtransform_lattice_scope_cache_lookup.dylib \
  --output-path ./rhexis-startup/src/lattice_transforms/transform_lattice_scope_cache_lookup-v0.rhp

echo "📦 Packing RHP: transform_net_send_flux_batch"
./target/debug/rhp pack \
  --plugin-type transform \
  --descriptor-path ./packages/transforms/transform_net_send_flux_batch/src/rhp.json \
  --code-path ./target/debug/libtransform_net_send_flux_batch.dylib \
  --output-path ./rhexis-startup/src/transform_net_send_flux_batch-v0.rhp
echo

echo "📦 Packing RHP: transform_net_send_flux_fire"
./target/debug/rhp pack \
  --plugin-type transform \
  --descriptor-path ./packages/transforms/transform_net_send_flux_fire/src/rhp.json \
  --code-path ./target/debug/libtransform_net_send_flux_fire.dylib \
  --output-path ./rhexis-startup/src/transform_net_send_flux_fire-v0.rhp

echo "📦 Packing RHP: transform_crypto_ed25519_generate"
./target/debug/rhp pack \
  --plugin-type transform \
  --descriptor-path ./packages/transforms/transform_crypto_ed25519_generate/src/rhp.json \
  --code-path ./target/debug/libtransform_crypto_ed25519_generate.dylib \
  --output-path ./rhexis-startup/src/transform_crypto_ed25519_generate-v0.rhp

echo "🧬 Packing HPC: hpc_net_send_flux"
./target/debug/rhp pack \
  --plugin-type hpc \
  --descriptor-path ./packages/hpcs/hpc_net_send_flux/src/rhp.json \
  --code-path ./target/debug/libhpc_net_send_flux.dylib \
  --output-path ./rhexis-startup/src/hpc_net_send_flux-v0.rhp

echo "🧬 Packing HPC: hpc_crypto_ed25519_generate"
./target/debug/rhp pack \
  --plugin-type hpc \
  --descriptor-path ./packages/hpcs/hpc_crypto_ed25519_generate/src/rhp.json \
  --code-path ./target/debug/libhpc_crypto_ed25519_generate.dylib \
  --output-path ./rhexis-startup/src/hpc_crypto_ed25519_generate-v0.rhp

echo "🧩 Building sample flux"
./target/debug/flux pack \
  --input ./tools/flux/src/net.json \
  --output ./rhexis-startup/src/net.flux

read -n1 -r -p "Press any key to fire up the CRE..."

./target/debug/rhexis-startup \
    --startup-script ./rhexis-startup/src/net-test.json
