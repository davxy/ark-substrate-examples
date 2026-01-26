#!/bin/bash
set -ex

pallet=$1
extrinsic=$2
extra_arg=$3

binary="./target/release/ark-node"
run="$binary benchmark pallet"

# Skip compilation if --no-compile is passed unless the binary does not exist
if [[ $extra_arg != "--no-compile" ]] || [[ ! -f $binary ]]; then
    RUSTFLAGS="-C target-cpu=native" cargo build --release -p ark-node --features runtime-benchmarks,small-ring
fi

# How many repetitions of each benchmark should be run
REPEAT=${REPEAT:-3}
# How many samples we should take across the variable components
STEPS=${STEPS:-30}

results_dir="./pallets/${pallet#pallet_ark_}/src"

if [[ $pallet == "all" ]]; then
    pallet="*"
fi

if [[ $pallet == "" ]]; then
    echo "Usage ./benchmark.sh <pallet> <extrinsic>"
    echo ""
    echo "Available pallets:"
    $run --list
    exit 1
fi

if [[ $extrinsic == "all" ]]; then
    extrinsic="*"
fi

if [[ $extrinsic == "" ]]; then
    echo "Usage ./benchmark.sh <pallet> <extrinsic>"
    echo ""
    echo "Available extrinsics for '$pallet':"
    $run --pallet "$pallet" --list | grep "$pallet" | awk '{ print "- " $2 }'
    echo ""
    echo "Use 'all' to run all benchmarks"
    exit 1
fi

$run \
  --runtime=target/release/wbuild/ark-runtime/ark_runtime.compact.wasm \
  --template=pallets/frame-weight-template.hbs \
  --pallet="$pallet" \
  --extrinsic="$extrinsic" \
  --steps="$steps" \
  --repeat="$repeat" \
  --no-storage-info \
  --disable-proof-recording \
  --no-median-slopes \
  --json-file="$results_dir/results.json" \
  --output="$results_dir/weights.rs"
