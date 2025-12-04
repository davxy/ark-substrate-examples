use crate::{
    mock::{new_test_ext, ArkHostcalls, RuntimeOrigin},
    sub_ed_on_bls12_381_bandersnatch,
    utils::*,
};
use ark_scale::scale::Encode;
use polkadot_sdk::frame_support::assert_ok;

const MSM_ITEMS: u32 = 500;
const SCALAR_WORDS: u32 = 3;

// ---------------------------------------------
// Tests for ed-on-bls12-381-bandersnatch
// ---------------------------------------------

#[test]
fn ark_ed_on_bls12_381_bandersnatch_msm_sw() {
    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_381_bandersnatch_msm_sw_rand(
            RuntimeOrigin::none(),
            MSM_ITEMS,
            false,
        ));
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_msm_sw() {
    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ed_on_bls12_381_bandersnatch_msm_sw_rand(
            RuntimeOrigin::none(),
            MSM_ITEMS,
            true
        ));
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_msm_te() {
    let (bases, scalars) =
        make_msm_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>(MSM_ITEMS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::ark_ed_on_bls12_381_bandersnatch_msm_te(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode()
        ));
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_msm_te() {
    let (bases, scalars) =
        make_msm_args::<sub_ed_on_bls12_381_bandersnatch::EdwardsProjective>(MSM_ITEMS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::sub_ed_on_bls12_381_bandersnatch_msm_te(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode()
        ));
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_projective_sw() {
    let (base, scalar) =
        make_scalar_args_projective::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::ark_ed_on_bls12_381_bandersnatch_mul_projective_sw(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_projective_sw() {
    new_test_ext().execute_with(|| {
        let (base, scalar) = make_scalar_args_projective::<
            sub_ed_on_bls12_381_bandersnatch::SWProjective,
        >(SCALAR_WORDS);

        assert_ok!(
            ArkHostcalls::sub_ed_on_bls12_381_bandersnatch_mul_projective_sw(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_affine_sw() {
    let (base, scalar) =
        make_scalar_args::<ark_ed_on_bls12_381_bandersnatch::SWAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::ark_ed_on_bls12_381_bandersnatch_mul_affine_sw(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_affine_sw() {
    let (base, scalar) =
        make_scalar_args::<sub_ed_on_bls12_381_bandersnatch::SWAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::sub_ed_on_bls12_381_bandersnatch_mul_affine_sw(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_projective_te() {
    let (base, scalar) = make_scalar_args_projective::<
        ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
    >(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::ark_ed_on_bls12_381_bandersnatch_mul_projective_te(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_projective_te() {
    let (base, scalar) = make_scalar_args_projective::<
        ark_ed_on_bls12_381_bandersnatch::EdwardsProjective,
    >(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::sub_ed_on_bls12_381_bandersnatch_mul_projective_te(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn ark_ed_on_bls12_381_bandersnatch_mul_affine_te() {
    let (base, scalar) =
        make_scalar_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::ark_ed_on_bls12_381_bandersnatch_mul_affine_te(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}

#[test]
fn sub_ed_on_bls12_381_bandersnatch_mul_affine_te() {
    let (base, scalar) =
        make_scalar_args::<sub_ed_on_bls12_381_bandersnatch::EdwardsAffine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(
            ArkHostcalls::sub_ed_on_bls12_381_bandersnatch_mul_affine_te(
                RuntimeOrigin::none(),
                base.encode(),
                scalar.encode()
            )
        );
    });
}
