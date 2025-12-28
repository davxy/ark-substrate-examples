#![allow(dead_code)]

use crate::{
    ark_bandersnatch, CompressedPoint, IetfProofRaw, InputRaw, OutputRaw, PublicKeyRaw,
    RingProofRaw,
};
use ark_vrf::reexports::ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_vrf::reexports::ark_std::vec::Vec;

pub trait GetRaw<const N: usize>: CanonicalSerialize {
    fn get_raw(&self) -> [u8; N] {
        let mut buf = [0_u8; N];
        self.serialize_compressed(&mut buf[..]).unwrap();
        buf
    }
}

impl<T, const N: usize> GetRaw<N> for T where T: CanonicalSerialize {}

pub fn ietf_verify_params_gen() -> (PublicKeyRaw, InputRaw, OutputRaw, IetfProofRaw) {
    use ark_vrf::ietf::Prover;

    let secret = ark_bandersnatch::Secret::from_seed(b"secret");
    let public = secret.public();
    let input = ark_bandersnatch::Input::new(b"input").unwrap();
    let output = secret.output(input);
    let proof = secret.prove(input, output, &[]);

    (
        CompressedPoint(public.get_raw()),
        CompressedPoint(input.get_raw()),
        CompressedPoint(output.get_raw()),
        IetfProofRaw(proof.get_raw()),
    )
}

pub fn ring_members_gen_raw(ring_size: u32) -> Vec<PublicKeyRaw> {
    log::debug!("Generate {ring_size} ring items");
    ring_members_gen(ring_size)
        .into_iter()
        .map(|pk| {
            let mut buf = PublicKeyRaw::default();
            pk.0.serialize_compressed(&mut buf.0[..]).unwrap();
            buf
        })
        .collect()
}

pub fn ring_members_gen(ring_size: u32) -> Vec<ark_bandersnatch::Public> {
    (0..ring_size)
        .map(|i| ark_bandersnatch::Secret::from_seed(&[i as u8]).public())
        .collect()
}

pub(crate) const SRS_RAW: &[u8] = include_bytes!("static/srs-uncompressed.bin");

pub fn ring_verify_params_gen(
    max_ring_size: u32,
    members: Option<&[PublicKeyRaw]>,
) -> (InputRaw, OutputRaw, RingProofRaw) {
    use ark_vrf::ring::Prover;

    let secret = ark_bandersnatch::Secret::from_seed(&[0_u8]);
    let input = ark_bandersnatch::Input::new(b"input").unwrap();
    let output = secret.output(input);

    let pcs_params =
        ark_bandersnatch::PcsParams::deserialize_uncompressed_unchecked(SRS_RAW).unwrap();
    let params =
        ark_bandersnatch::RingProofParams::from_pcs_params(max_ring_size as usize, pcs_params)
            .unwrap();

    let ring_members = match members {
        Some(members) => members
            .iter()
            .map(|k| ark_bandersnatch::Public::deserialize_compressed_unchecked(&k.0[..]).unwrap())
            .collect::<Vec<_>>(),
        None => ring_members_gen(max_ring_size),
    };
    let ring_members = ring_members.into_iter().map(|pk| pk.0).collect::<Vec<_>>();

    let prover_key = params.prover_key(&ring_members);
    let prover = params.prover(prover_key, 0);

    let proof = secret.prove(input, output, &[], &prover);

    (
        CompressedPoint(input.get_raw()),
        CompressedPoint(output.get_raw()),
        RingProofRaw(proof.get_raw()),
    )
}
