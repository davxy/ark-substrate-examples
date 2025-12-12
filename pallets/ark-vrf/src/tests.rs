use crate::mock::{MaxRingSize, RuntimeOrigin, Test};
use crate::Pallet;
use crate::{mock::new_test_ext, utils};

fn ietf_verify(optimized: bool) {
    let (public_raw, input_raw, output_raw, proof_raw) = utils::ietf_verify_params_gen();
    Pallet::<Test>::ietf_verify(
        RuntimeOrigin::none(),
        public_raw,
        input_raw,
        output_raw,
        proof_raw,
        optimized,
    )
    .unwrap();
}

#[test]
fn ark_ietf_verify() {
    new_test_ext().execute_with(|| ietf_verify(false));
}

#[test]
fn sub_ietf_verify() {
    new_test_ext().execute_with(|| ietf_verify(true))
}

fn ring_commit(optimized: bool) {
    let origin = RuntimeOrigin::none();
    let members = utils::ring_members_gen(MaxRingSize::get());
    Pallet::<Test>::push_members(origin.clone(), members, optimized).unwrap();
    Pallet::<Test>::ring_commit(origin, optimized).unwrap();
}

#[test]
fn ark_ring_commit() {
    new_test_ext().execute_with(|| ring_commit(false));
}

#[test]
fn sub_ring_commit() {
    new_test_ext().execute_with(|| ring_commit(true));
}
