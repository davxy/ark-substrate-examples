use crate::{
    mock::{new_test_ext, ArkHostcalls, RuntimeOrigin},
    utils::*,
    ArkScale,
};
use ark_ec::{AffineRepr, CurveGroup};
use ark_scale::scale::Encode;
use ark_std::{test_rng, UniformRand};
use frame_support::{assert_err, assert_ok, pallet_prelude::DispatchError};

const MSM_ITEMS: u32 = 256;
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

fn bls12_381_msm_g1(optimized: bool) {
    let (bases, scalars) = make_msm_args::<ark_bls12_381::G1Projective>(MSM_ITEMS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::bls12_381_msm_g1(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode(),
            optimized,
        ));
    });
}

#[test]
fn ark_bls12_381_msm_g1() {
    bls12_381_msm_g1(false);
}

#[test]
fn sub_bls12_381_msm_g1() {
    bls12_381_msm_g1(true);
}

fn bls12_381_mul_projective_g1(optimized: bool) {
    let (base, scalar) = make_scalar_args_projective::<ark_bls12_381::G1Projective>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::bls12_381_mul_projective_g1(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode(),
            optimized
        ));
    });
}

#[test]
fn ark_bls12_381_mul_projective_g1() {
    bls12_381_mul_projective_g1(false)
}

#[test]
fn sub_bls12_381_mul_projective_g1() {
    bls12_381_mul_projective_g1(true)
}

fn bls12_381_mul_affine_g1(optimized: bool) {
    let (base, scalar) = make_scalar_args::<ark_bls12_381::G1Affine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::bls12_381_mul_affine_g1(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode(),
            optimized
        ));
    });
}

#[test]
fn ark_bls12_381_mul_affine_g1() {
    bls12_381_mul_affine_g1(false)
}

#[test]
fn sub_bls12_381_mul_affine_g1() {
    bls12_381_mul_affine_g1(true)
}

fn bls12_381_msm_g2(optimized: bool) {
    let (bases, scalars) = make_msm_args::<ark_bls12_381::G2Projective>(MSM_ITEMS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::bls12_381_msm_g2(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode(),
            optimized,
        ));
    });
}

#[test]
fn ark_bls12_381_msm_g2() {
    bls12_381_msm_g2(false);
}

#[test]
fn sub_bls12_381_msm_g2() {
    bls12_381_msm_g2(true);
}

fn bls12_381_mul_projective_g2(optimized: bool) {
    let (base, scalar) = make_scalar_args_projective::<ark_bls12_381::G2Projective>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::bls12_381_mul_projective_g2(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode(),
            optimized
        ));
    });
}

#[test]
fn ark_bls12_381_mul_projective_g2() {
    bls12_381_mul_projective_g2(false)
}

#[test]
fn sub_bls12_381_mul_projective_g2() {
    bls12_381_mul_projective_g2(true)
}

fn bls12_381_mul_affine_g2(optimized: bool) {
    let (base, scalar) = make_scalar_args::<ark_bls12_381::G2Affine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::bls12_381_mul_affine_g2(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode(),
            optimized
        ));
    });
}

#[test]
fn ark_bls12_381_mul_affine_g2() {
    bls12_381_mul_affine_g2(false)
}

#[test]
fn sub_bls12_381_mul_affine_g2() {
    bls12_381_mul_affine_g2(true)
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

// ---------------------------------------------
// BLS12-381 BLS signature verification
// ---------------------------------------------

// Construct a valid BLS signature tuple (pk, msg_hash, sig) satisfying:
//   e(sig, G2_gen) == e(pk, msg_hash)
// by picking random scalars s, t and using bilinearity:
//   pk = s * G1_gen, msg_hash = t * G2_gen, sig = (s * t) * G1_gen
fn make_bls_signature_args() -> (ArkScale<ark_bls12_381::G1Affine>, ArkScale<ark_bls12_381::G2Affine>, ArkScale<ark_bls12_381::G1Affine>) {
    let rng = &mut test_rng();
    let s = ark_bls12_381::Fr::rand(rng);
    let t = ark_bls12_381::Fr::rand(rng);

    let pk = (ark_bls12_381::G1Affine::generator() * s).into_affine();
    let msg_hash = (ark_bls12_381::G2Affine::generator() * t).into_affine();
    let sig = (ark_bls12_381::G1Affine::generator() * (s * t)).into_affine();

    (pk.into(), msg_hash.into(), sig.into())
}

fn bls12_381_verify_bls_signature_valid(direct: bool) {
    let (pk, msg_hash, sig) = make_bls_signature_args();

    new_test_ext().execute_with(|| {
        assert_ok!(ArkHostcalls::bls12_381_verify_bls_signature(
            RuntimeOrigin::none(),
            pk.encode(),
            msg_hash.encode(),
            sig.encode(),
            direct,
        ));
    });
}

#[test]
fn direct_bls12_381_verify_bls_signature_valid() {
    bls12_381_verify_bls_signature_valid(true);
}

#[test]
fn pairing_bls12_381_verify_bls_signature_valid() {
    bls12_381_verify_bls_signature_valid(false);
}

fn bls12_381_verify_bls_signature_invalid(direct: bool) {
    let (pk, msg_hash, _sig) = make_bls_signature_args();
    // Use a random G1 point as a bogus signature
    let bad_sig: ArkScale<ark_bls12_381::G1Affine> =
        ark_bls12_381::G1Affine::rand(&mut test_rng()).into();

    new_test_ext().execute_with(|| {
        assert_err!(
            ArkHostcalls::bls12_381_verify_bls_signature(
                RuntimeOrigin::none(),
                pk.encode(),
                msg_hash.encode(),
                bad_sig.encode(),
                direct,
            ),
            DispatchError::Other("Invalid BLS signature")
        );
    });
}

#[test]
fn direct_bls12_381_verify_bls_signature_invalid() {
    bls12_381_verify_bls_signature_invalid(true);
}

#[test]
fn pairing_bls12_381_verify_bls_signature_invalid() {
    bls12_381_verify_bls_signature_invalid(false);
}
