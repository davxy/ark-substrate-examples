#!/bin/bash
set -e

cargo build --release -p minimal-template-node --features runtime-benchmarks

run="cargo run --release -p minimal-template-node --features runtime-benchmarks -- benchmark pallet"

pallet=$1
extrinsic=$2

repeat=20
steps=50

results_dir="./results"
mkdir -p "$results_dir"


if [[ $pallet == "" ]]; then
    echo "Usage ./benchmark.sh <pallet> <extrinsic>"
    echo ""
    echo "Available pallets:"
    $run --list
    exit 1
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
  --chain dev \
  --pallet "$pallet" \
  --extrinsic "$extrinsic" \
  --steps $steps \
  --repeat=$repeat \
  --no-storage-info \
  --json-file=$results_dir/results.json \
  --output=$results_dir/weights.rs

