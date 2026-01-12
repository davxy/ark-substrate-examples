use crate::{
    mock::{new_test_ext, ArkGroth16, RuntimeOrigin},
    utils,
};
use frame_support::assert_ok;
// ---------------------------------------------
// Tests for bls12-381
// ---------------------------------------------

fn bls12_381_groth16_verify(optimized: bool) {
    let (vk, c, proof) = utils::groth16_verify_params_gen();

    new_test_ext().execute_with(|| {
        assert_ok!(ArkGroth16::bls12_381_groth16_verify(
            RuntimeOrigin::none(),
            vk.0,
            c.0,
            proof.0,
            optimized
        ));
    });
}

#[test]
fn ark_bls12_381_groth16_verify() {
    bls12_381_groth16_verify(false);
}

#[test]
fn sub_bls12_381_groth16_verify() {
    bls12_381_groth16_verify(true);
}
