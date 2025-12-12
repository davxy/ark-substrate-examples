#![allow(dead_code)]

use crate::{ark_bandersnatch, PublicKeyRaw};
use ark_vrf::reexports::ark_serialize::CanonicalSerialize;
use ark_vrf::reexports::ark_std::vec::Vec;

pub fn ring_members_gen(size: u32) -> Vec<PublicKeyRaw> {
    (0..size)
        .map(|i| {
            let pk = ark_bandersnatch::Secret::from_seed(&[i as u8]).public();
            let mut buf = PublicKeyRaw::default();
            pk.0.serialize_compressed(&mut buf.0[..]).unwrap();
            buf
        })
        .collect()
}
