use super::*;

#[allow(unused)]
use crate::Pallet as ArkVrf;

use crate::utils;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

const RING_SIZE_MIN: u32 = 1;
const RING_SIZE_MAX: u32 = 50;

#[benchmarks]
mod benchmarks {
    use super::*;

    /// Ring commitment with buffered keys
    ///
    /// `x` keys are stored in the `RingKeys` buffer before running the benchmark.
    /// We're benchmarking the combination of:
    /// 1. Accumulation of the `x` buffered keys
    /// 2. Final ring commitment
    #[benchmark]
    fn ark_ring_vrf_accumulate_and_commit(x: Linear<RING_SIZE_MIN, RING_SIZE_MAX>) {
        let members = utils::ring_members_gen_raw(x);
        let members: BoundedVec<PublicKeyRaw, T::MaxRingSize> = members.try_into().unwrap();

        RingKeys::<T>::set(Some(members));

        #[extrinsic_call]
        ring_commit(RawOrigin::None, false);
    }

    /// Same as `ark_ring_vrf_accumulate_and_commit` but using the Substrate hostcalls.
    #[benchmark]
    fn sub_ring_vrf_accumulate_and_commit(x: Linear<RING_SIZE_MIN, RING_SIZE_MAX>) {
        let members = utils::ring_members_gen_raw(x);
        let members: BoundedVec<PublicKeyRaw, T::MaxRingSize> = members.try_into().unwrap();

        RingKeys::<T>::set(Some(members));

        #[extrinsic_call]
        ring_commit(RawOrigin::None, true);
    }

    /// Ring accumulation
    ///
    /// `x` keys are accumulated (no commit)
    #[benchmark]
    fn ark_ring_vrf_accumulate(x: Linear<RING_SIZE_MIN, RING_SIZE_MAX>) {
        let members = utils::ring_members_gen_raw(x);

        #[extrinsic_call]
        push_members(RawOrigin::None, members, false);
    }

    /// Same as `ark_ring_vrf_accumulate` but with substrate hostcalls
    #[benchmark]
    fn sub_ring_vrf_accumulate(x: Linear<RING_SIZE_MIN, RING_SIZE_MAX>) {
        let members = utils::ring_members_gen_raw(x);

        #[extrinsic_call]
        push_members(RawOrigin::None, members, true);
    }

    /// Ring commitment
    ///
    /// Keys are assumed to be already accumulated.
    #[benchmark]
    fn ark_ring_vrf_commit() {
        let members = utils::ring_members_gen_raw(RING_SIZE_MAX);

        Pallet::<T>::push_members_impl::<ArkSuite>(members);

        #[extrinsic_call]
        ring_commit(RawOrigin::None, false);
    }

    /// Same as `ark_ring_vrf_commit_accumulated` but using the Substrate hostcalls.
    #[benchmark]
    fn sub_ring_vrf_commit() {
        let members = utils::ring_members_gen_raw(RING_SIZE_MAX);
        Pallet::<T>::push_members_impl::<ArkSuite>(members);

        #[extrinsic_call]
        ring_commit(RawOrigin::None, true);
    }

    #[benchmark]
    fn ark_ring_vrf_verify(x: Linear<RING_SIZE_MIN, RING_SIZE_MAX>) {
        let members = utils::ring_members_gen_raw(x);
        let (input_raw, output_raw, proof_raw) =
            utils::ring_verify_params_gen(T::MaxRingSize::get(), Some(&members));

        Pallet::<T>::push_members_impl::<ArkSuite>(members);
        Pallet::<T>::commit_impl::<ArkSuite>();

        #[extrinsic_call]
        ring_verify(RawOrigin::None, input_raw, output_raw, proof_raw, false);
    }

    #[benchmark]
    fn sub_ring_vrf_verify(x: Linear<RING_SIZE_MIN, RING_SIZE_MAX>) {
        let members = utils::ring_members_gen_raw(x);
        let (input_raw, output_raw, proof_raw) =
            utils::ring_verify_params_gen(T::MaxRingSize::get(), Some(&members));

        Pallet::<T>::push_members_impl::<ArkSuite>(members);
        Pallet::<T>::commit_impl::<ArkSuite>();

        #[extrinsic_call]
        ring_verify(RawOrigin::None, input_raw, output_raw, proof_raw, true);
    }

    #[benchmark]
    fn ark_ietf_vrf_verify() {
        let (public_raw, input_raw, output_raw, proof_raw) = utils::ietf_verify_params_gen();

        #[extrinsic_call]
        ietf_verify(
            RawOrigin::None,
            public_raw,
            input_raw,
            output_raw,
            proof_raw,
            false,
        );
    }

    #[benchmark]
    fn sub_ietf_vrf_verify() {
        let (public_raw, input_raw, output_raw, proof_raw) = utils::ietf_verify_params_gen();

        #[extrinsic_call]
        ietf_verify(
            RawOrigin::None,
            public_raw,
            input_raw,
            output_raw,
            proof_raw,
            true,
        );
    }

    impl_benchmark_test_suite!(ArkVrf, crate::mock::new_test_ext(), crate::mock::Test);
}
