default: benchmark

# Run all benchmarks locall
benchmark:
	./scripts/benchmark.sh pallet_ark_hostcalls all
	./scripts/benchmark.sh pallet_ark_vrf all --no-compile
	./scripts/benchmark.sh pallet_ark_groth16 all --no-compile

# Benchmark on SSH server and copy the weight files back
ssh-benchmark server:
	#!/usr/bin/env bash
	set -ex
	rsync -avz --progress --exclude 'target' --exclude '.git' . {{server}}:polkadot-arkworks-extensions/
	ssh {{server}} "source ~/.cargo/env && cd polkadot-arkworks-extensions && just benchmark"
	scp {{server}}:polkadot-arkworks-extensions/pallets/vrf/src/weights.rs ./pallets/vrf/src/weights.rs
	scp {{server}}:polkadot-arkworks-extensions/pallets/groth16/src/weights.rs ./pallets/groth16/src/weights.rs
	scp {{server}}:polkadot-arkworks-extensions/pallets/hostcalls/src/weights.rs ./pallets/hostcalls/src/weights.rs
