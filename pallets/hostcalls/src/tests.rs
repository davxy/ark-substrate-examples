use crate::{
    mock::{new_test_ext, ArkHostcalls, RuntimeOrigin},
    utils::*,
};
use ark_scale::scale::Encode;
use frame_support::assert_ok;

const MSM_ITEMS: u32 = 500;
const SCALAR_WORDS: u32 = 3;

// ---------------------------------------------
// Tests for bls12-381
// ---------------------------------------------

fn bls12_381_pairing(optimized: bool) {
    let (a, b) = make_pairing_args::<ark_bls12_381::G1Affine, ark_bls12_381::G2Affine>();

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::bls12_381_pairing(
            RuntimeOrigin::none(),
            a.encode(),
            b.encode(),
            optimized
        ));
    });
}

#[test]
fn ark_bls12_381_pairing() {
    bls12_381_pairing(false);
}

#[test]
fn sub_bls12_381_pairing() {
    bls12_381_pairing(true);
}

// ---------------------------------------------
// Tests for ed-on-bls12-377
// ---------------------------------------------

fn ed_on_bls12_377_msm_te(optimized: bool) {
    let (bases, scalars) = make_msm_args::<ark_ed_on_bls12_377::EdwardsProjective>(MSM_ITEMS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_377_msm_te(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode(),
            optimized
        ));
    });
}

#[test]
fn ark_ed_on_bls12_377_msm_te() {
    ed_on_bls12_377_msm_te(false);
}

#[test]
fn sub_ed_on_bls12_377_msm_te() {
    ed_on_bls12_377_msm_te(true);
}

fn ed_on_bls12_377_mul_projective_te(optimized: bool) {
    let (base, scalar) =
        make_scalar_args_projective::<ark_ed_on_bls12_377::EdwardsProjective>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_377_mul_projective_te(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode(),
            optimized,
        ));
    });
}

#[test]
fn ark_ed_on_bls12_377_mul_projective_te() {
    ed_on_bls12_377_mul_projective_te(false)
}

#[test]
fn sub_ed_on_bls12_377_mul_projective_te() {
    ed_on_bls12_377_mul_projective_te(true)
}

fn ed_on_bls12_377_mul_affine_te(optimized: bool) {
    let (base, scalar) = make_scalar_args::<ark_ed_on_bls12_377::EdwardsAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_377_mul_affine_te(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode(),
            optimized
        ));
    });
}

#[test]
fn ark_ed_on_bls12_377_mul_affine_te() {
    ed_on_bls12_377_mul_affine_te(false)
}
#[test]
fn sub_ed_on_bls12_377_mul_affine_te() {
    ed_on_bls12_377_mul_affine_te(true)
}

// ---------------------------------------------
// Tests for ed-on-bls12-381-bandersnatch
// ---------------------------------------------

// Short Weierstrass

fn ed_on_bls12_381_bandersnatch_msm_sw(optimized: bool) {
    let (bases, scalars) =
        make_msm_args::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(MSM_ITEMS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_381_bandersnatch_msm_sw(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode(),
            optimized,
        ));
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_msm_sw() {
    ed_on_bls12_381_bandersnatch_msm_sw(false);
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_msm_sw() {
    ed_on_bls12_381_bandersnatch_msm_sw(true);
}

fn ed_on_bls12_381_bandersnatch_mul_projective_sw(optimized: bool) {
    let (base, scalar) =
        make_scalar_args_projective::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::ed_on_bls12_381_bandersnatch_mul_projective_sw(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode(),
                optimized
            )
        );
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_projective_sw() {
    ed_on_bls12_381_bandersnatch_mul_projective_sw(false)
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_projective_sw() {
    ed_on_bls12_381_bandersnatch_mul_projective_sw(true)
}

fn ed_on_bls12_381_bandersnatch_mul_affine_sw(optimized: bool) {
    let (base, scalar) =
        make_scalar_args::<ark_ed_on_bls12_381_bandersnatch::SWAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_381_bandersnatch_mul_affine_sw(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode(),
            optimized
        ));
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_affine_sw() {
    ed_on_bls12_381_bandersnatch_mul_affine_sw(false)
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_affine_sw() {
    ed_on_bls12_381_bandersnatch_mul_affine_sw(true)
}

// Twisted Edwards

fn ed_on_bls12_381_bandersnatch_msm_te(optimized: bool) {
    let (bases, scalars) =
        make_msm_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>(MSM_ITEMS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_381_bandersnatch_msm_te(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode(),
            optimized
        ));
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_msm_te() {
    ed_on_bls12_381_bandersnatch_msm_te(false);
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_msm_te() {
    ed_on_bls12_381_bandersnatch_msm_te(true);
}

fn ed_on_bls12_381_bandersnatch_mul_projective_te(optimized: bool) {
    let (base, scalar) = make_scalar_args_projective::<
        ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
    >(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::ed_on_bls12_381_bandersnatch_mul_projective_te(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode(),
                optimized,
            )
        );
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_projective_te() {
    ed_on_bls12_381_bandersnatch_mul_projective_te(false)
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_projective_te() {
    ed_on_bls12_381_bandersnatch_mul_projective_te(true)
}

fn ed_on_bls12_381_bandersnatch_mul_affine_te(optimized: bool) {
    let (base, scalar) =
        make_scalar_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_381_bandersnatch_mul_affine_te(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode(),
            optimized
        ));
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_affine_te() {
    ed_on_bls12_381_bandersnatch_mul_affine_te(false)
}
#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_affine_te() {
    ed_on_bls12_381_bandersnatch_mul_affine_te(true)
}
