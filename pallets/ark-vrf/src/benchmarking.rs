use super::*;

use crate::utils;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

const RING_SIZE_MIN: u32 = 1;
const RING_SIZE_MAX: u32 = 10;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn ark_ring_vrf_commit_buffered(x: Linear<RING_SIZE_MIN, RING_SIZE_MAX>) {
        let members = utils::ring_members_gen(x);
        let members: BoundedVec<PublicKeyRaw, T::MaxRingSize> = members.try_into().unwrap();

        RingKeys::<T>::set(Some(members));

        #[extrinsic_call]
        ring_commit(RawOrigin::None, false);
    }

    #[benchmark]
    fn sub_ring_vrf_commit_buffered(x: Linear<RING_SIZE_MIN, RING_SIZE_MAX>) {
        let members = utils::ring_members_gen(x);
        let members: BoundedVec<PublicKeyRaw, T::MaxRingSize> = members.try_into().unwrap();

        RingKeys::<T>::set(Some(members));

        #[extrinsic_call]
        ring_commit(RawOrigin::None, true);
    }

    #[benchmark]
    fn ark_ring_vrf_commit_unbuffered(x: Linear<RING_SIZE_MIN, RING_SIZE_MAX>) {
        let members = utils::ring_members_gen(x);

        Pallet::<T>::push_members_impl::<ArkSuite>(members);

        #[extrinsic_call]
        ring_commit(RawOrigin::None, false);
    }

    #[benchmark]
    fn sub_ring_vrf_commit_unbuffered(x: Linear<RING_SIZE_MIN, RING_SIZE_MAX>) {
        let members = utils::ring_members_gen(x);

        Pallet::<T>::push_members_impl::<ArkSuite>(members);

        #[extrinsic_call]
        ring_commit(RawOrigin::None, true);
    }

    impl_benchmark_test_suite!(ArkVrf, crate::mock::new_test_ext(), crate::mock::Test);
}
