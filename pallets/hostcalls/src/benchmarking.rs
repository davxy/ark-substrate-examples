//! Benchmarking setup for pallet-ark-hostcalls

use super::*;

#[allow(unused)]
use crate::Pallet as ArkHostcalls;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

// Min number of elements for multi scalar multiplication
const MSM_LEN_MIN: u32 = 10;
// Max number of elements for multi scalar multiplication
const MSM_LEN_MAX: u32 = 100;

// // Scalar min words for single scalar multiplication (1 = 64 bit)
// const SCALAR_WORDS_MIN: u32 = 1;
// // Scalar max words for single scalar multiplication (16 = 1024 bit)
// const SCALAR_WORDS_MAX: u32 = 16;

#[benchmarks]
mod benchmarks {
    use super::*;

    // ---------------------------------------------
    // Calls for bls12-381
    // ---------------------------------------------

    #[benchmark]
    fn ark_bls12_381_pairing() {
        let (a, b) = utils::make_pairing_args::<ark_bls12_381::G1Affine, ark_bls12_381::G2Affine>();

        #[extrinsic_call]
        bls12_381_pairing(RawOrigin::None, a.encode(), b.encode(), false);
    }

    // ---------------------------------------------
    // Calls for ed-on-bls12-377
    // ---------------------------------------------

    // Twisted Edwards

    #[benchmark]
    fn ark_ed_on_bls12_377_msm_te(x: Linear<MSM_LEN_MIN, MSM_LEN_MAX>) {
        let (bases, scalars) = utils::make_msm_args::<ark_ed_on_bls12_377::EdwardsProjective>(x);

        #[extrinsic_call]
        ed_on_bls12_377_msm_te(
            RawOrigin::Signed(whitelisted_caller()),
            bases.encode(),
            scalars.encode(),
            false,
        );
    }

    #[benchmark]
    fn sub_ed_on_bls12_377_msm_te(x: Linear<MSM_LEN_MIN, MSM_LEN_MAX>) {
        let (bases, scalars) = utils::make_msm_args::<ark_ed_on_bls12_377::EdwardsProjective>(x);

        #[extrinsic_call]
        ed_on_bls12_377_msm_te(
            RawOrigin::Signed(whitelisted_caller()),
            bases.encode(),
            scalars.encode(),
            true,
        );
    }

    #[benchmark]
    fn ark_ed_on_bls12_377_mul_projective_te() {
        let (base, scalar) =
            utils::make_mul_projective_args::<ark_ed_on_bls12_377::EdwardsProjective>();

        #[extrinsic_call]
        ed_on_bls12_377_mul_projective_te(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            false,
        );
    }

    #[benchmark]
    fn sub_ed_on_bls12_377_mul_projective_te() {
        let (base, scalar) =
            utils::make_mul_projective_args::<ark_ed_on_bls12_377::EdwardsProjective>();

        #[extrinsic_call]
        ed_on_bls12_377_mul_projective_te(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            true,
        );
    }

    #[benchmark]
    fn ark_ed_on_bls12_377_mul_affine_te() {
        let (base, scalar) = utils::make_mul_affine_args::<ark_ed_on_bls12_377::EdwardsAffine>();

        #[extrinsic_call]
        ed_on_bls12_377_mul_affine_te(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            false,
        );
    }

    #[benchmark]
    fn sub_ed_on_bls12_377_mul_affine_te() {
        let (base, scalar) = utils::make_mul_affine_args::<ark_ed_on_bls12_377::EdwardsAffine>();

        #[extrinsic_call]
        ed_on_bls12_377_mul_affine_te(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            true,
        );
    }

    // ---------------------------------------------
    // Calls for ed-on-bls12-381-bandersnatch
    // ---------------------------------------------

    // Short Weierstrass

    #[benchmark]
    fn ark_ed_on_bls12_381_bandersnatch_msm_sw(x: Linear<MSM_LEN_MIN, MSM_LEN_MAX>) {
        let (bases, scalars) =
            utils::make_msm_args::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(x);

        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_msm_sw(
            RawOrigin::Signed(whitelisted_caller()),
            bases.encode(),
            scalars.encode(),
            false,
        );
    }

    #[benchmark]
    fn sub_ed_on_bls12_381_bandersnatch_msm_sw(x: Linear<MSM_LEN_MIN, MSM_LEN_MAX>) {
        let (bases, scalars) =
            utils::make_msm_args::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(x);
        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_msm_sw(
            RawOrigin::Signed(whitelisted_caller()),
            bases.encode(),
            scalars.encode(),
            true,
        );
    }

    #[benchmark]
    fn ark_ed_on_bls12_381_bandersnatch_mul_projective_sw() {
        let (base, scalar) =
            utils::make_mul_projective_args::<ark_ed_on_bls12_381_bandersnatch::SWProjective>();

        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_mul_projective_sw(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            false,
        );
    }

    #[benchmark]
    fn sub_ed_on_bls12_381_bandersnatch_mul_projective_sw() {
        let (base, scalar) =
            utils::make_mul_projective_args::<ark_ed_on_bls12_381_bandersnatch::SWProjective>();

        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_mul_projective_sw(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            true,
        );
    }

    #[benchmark]
    fn ark_ed_on_bls12_381_bandersnatch_mul_affine_sw() {
        let (base, scalar) =
            utils::make_mul_affine_args::<ark_ed_on_bls12_381_bandersnatch::SWAffine>();

        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_mul_affine_sw(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            false,
        );
    }

    #[benchmark]
    fn sub_ed_on_bls12_381_bandersnatch_mul_affine_sw() {
        let (base, scalar) =
            utils::make_mul_affine_args::<ark_ed_on_bls12_381_bandersnatch::SWAffine>();

        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_mul_affine_sw(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            true,
        );
    }

    // Twisted Edwards

    #[benchmark]
    fn ark_ed_on_bls12_381_bandersnatch_msm_te(x: Linear<MSM_LEN_MIN, MSM_LEN_MAX>) {
        let (bases, scalars) =
            utils::make_msm_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>(x);
        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_msm_te(
            RawOrigin::Signed(whitelisted_caller()),
            bases.encode(),
            scalars.encode(),
            false,
        );
    }

    #[benchmark]
    fn sub_ed_on_bls12_381_bandersnatch_msm_te(x: Linear<MSM_LEN_MIN, MSM_LEN_MAX>) {
        let (bases, scalars) =
            utils::make_msm_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>(x);
        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_msm_te(
            RawOrigin::Signed(whitelisted_caller()),
            bases.encode(),
            scalars.encode(),
            true,
        );
    }

    #[benchmark]
    fn ark_ed_on_bls12_381_bandersnatch_mul_projective_te() {
        let (base, scalar) = utils::make_mul_projective_args::<
            ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
        >();

        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_mul_projective_te(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            false,
        );
    }

    #[benchmark]
    fn sub_ed_on_bls12_381_bandersnatch_mul_projective_te() {
        let (base, scalar) = utils::make_mul_projective_args::<
            ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
        >();

        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_mul_projective_te(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            true,
        );
    }

    #[benchmark]
    fn ark_ed_on_bls12_381_bandersnatch_mul_affine_te() {
        let (base, scalar) =
            utils::make_mul_affine_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>();

        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_mul_affine_te(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            false,
        );
    }

    #[benchmark]
    fn sub_ed_on_bls12_381_bandersnatch_mul_affine_te() {
        let (base, scalar) =
            utils::make_mul_affine_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>();

        #[extrinsic_call]
        ed_on_bls12_381_bandersnatch_mul_affine_te(
            RawOrigin::Signed(whitelisted_caller()),
            base.encode(),
            scalar.encode(),
            true,
        );
    }

    impl_benchmark_test_suite!(ArkHostcalls, crate::mock::new_test_ext(), crate::mock::Test);
}
