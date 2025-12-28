use crate::mock::{MaxRingSize, RuntimeOrigin, Test};
use crate::{mock::new_test_ext, utils};
use crate::{Pallet, PublicKeyRaw, RingBuilderPcsParams, RING_BUILDER_DATA, RING_BUILDER_PARAMS};

const TEST_RING_SIZE: u32 = 42;

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

fn ring_verify(optimized: bool) {
    let members = ring_commit(optimized);

    let (input_raw, output_raw, proof_raw) =
        utils::ring_verify_params_gen(MaxRingSize::get(), Some(&members));
    Pallet::<Test>::ring_verify(
        RuntimeOrigin::none(),
        input_raw,
        output_raw,
        proof_raw,
        optimized,
    )
    .unwrap()
}

fn ring_commit(optimized: bool) -> Vec<PublicKeyRaw> {
    let origin = RuntimeOrigin::none();
    let members = utils::ring_members_gen_raw(TEST_RING_SIZE);
    Pallet::<Test>::push_members(origin.clone(), members.clone(), optimized).unwrap();
    Pallet::<Test>::ring_commit(origin, optimized).unwrap();
    members
}

#[test]
fn ark_ietf_verify() {
    new_test_ext().execute_with(|| ietf_verify(false));
}

#[test]
fn sub_ietf_verify() {
    new_test_ext().execute_with(|| ietf_verify(true))
}

#[test]
fn ark_ring_commit() {
    new_test_ext().execute_with(|| ring_commit(false));
}

#[test]
fn sub_ring_commit() {
    new_test_ext().execute_with(|| ring_commit(true));
}

#[test]
fn ark_ring_verify() {
    new_test_ext().execute_with(|| ring_verify(false));
}

#[test]
fn sub_ring_verify() {
    new_test_ext().execute_with(|| ring_verify(true));
}

fn backend_works(pregen_params: bool) {
    use ark_vrf::reexports::ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
    use ark_vrf::ring::{Prover, Verifier};
    use ark_vrf::suites::bandersnatch as ark_bandersnatch;

    let secret = ark_bandersnatch::Secret::from_seed(&[0_u8]);
    let input = ark_bandersnatch::Input::new(b"input").unwrap();
    let output = secret.output(input);

    const _CHECK: u32 = crate::MAX_RING_SIZE - TEST_RING_SIZE; // Static check for MAX_RING_SIZE >= TEST_RING_SIZE

    let params = if pregen_params {
        let pcs_params =
            ark_bandersnatch::PcsParams::deserialize_uncompressed_unchecked(utils::SRS_RAW)
                .unwrap();
        ark_bandersnatch::RingProofParams::from_pcs_params(
            crate::MAX_RING_SIZE as usize,
            pcs_params,
        )
        .unwrap()
    } else {
        ark_bandersnatch::RingProofParams::from_seed(crate::MAX_RING_SIZE as usize, [0_u8; 32])
    };
    assert_eq!(params.max_ring_size(), crate::MAX_RING_SIZE as usize);

    let ring_members = utils::ring_members_gen(TEST_RING_SIZE)
        .into_iter()
        .map(|pk| pk.0)
        .collect::<Vec<_>>();

    // Prove

    let prover_key = params.prover_key(&ring_members);
    let prover = params.prover(prover_key, 0);

    let proof = secret.prove(input, output, &[], &prover);
    let mut proof_raw = Vec::new();
    proof.serialize_compressed(&mut proof_raw).unwrap();

    // Verify

    let verifier_key = params.verifier_key(&ring_members);
    let verifier = params.verifier(verifier_key);
    ark_bandersnatch::Public::verify(input, output, &[], &proof, &verifier).unwrap();

    // Verify with builder

    let (mut builder, builder_pcs_params) = if pregen_params {
        let builder = ark_bandersnatch::RingVerifierKeyBuilder::deserialize_uncompressed_unchecked(
            crate::RING_BUILDER_DATA,
        )
        .unwrap();
        let builder_pcs_params =
            RingBuilderPcsParams::deserialize_uncompressed(crate::RING_BUILDER_PARAMS).unwrap();
        (builder, builder_pcs_params)
    } else {
        params.verifier_key_builder()
    };

    assert_eq!(
        builder_pcs_params.uncompressed_size(),
        RING_BUILDER_PARAMS.len()
    );
    assert_eq!(builder.uncompressed_size(), RING_BUILDER_DATA.len());

    builder.append(&ring_members, &builder_pcs_params).unwrap();
    let verifier_key = builder.finalize();
    let verifier = params.verifier(verifier_key);
    ark_bandersnatch::Public::verify(input, output, &[], &proof, &verifier).unwrap();
}

#[test]
fn backend_works_zcash_params() {
    backend_works(true);
}

#[test]
fn backend_works_rand_params() {
    backend_works(false);
}
