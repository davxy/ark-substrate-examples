use super::*;

#[allow(unused)]
use crate::Pallet as ArkGroth16;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    // ---------------------------------------------
    // Calls for bls12-381
    // ---------------------------------------------

    #[benchmark]
    fn ark_bls12_381_groth16_verify() {
        let (vk, c, proof) = utils::groth16_verify_params_gen();

        #[extrinsic_call]
        bls12_381_groth16_verify(RawOrigin::None, vk.0, c.0, proof.0, false);
    }

    impl_benchmark_test_suite!(ArkGroth16, crate::mock::new_test_ext(), crate::mock::Test);
}
