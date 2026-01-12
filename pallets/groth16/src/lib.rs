#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod utils;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

use ark_ec::pairing::Pairing;
use ark_groth16::Groth16;
use ark_snark::SNARK;
use ark_std::vec::Vec;

pub use sp_crypto_ec_utils::bls12_381 as sub_bls12_381;

pub type ArkScaleWire<T> = ark_scale::ArkScale<T, { ark_scale::WIRE }>;
pub type ArkScaleHost<T> = ark_scale::ArkScale<T, { ark_scale::HOST_CALL }>;

pub use pallet::*;

use crate::utils::deserialize_uncompressed_host;

const DEFAULT_WEIGHT: u64 = 10_000;

pub type ScalarFieldFor<PairingT> = <PairingT as Pairing>::ScalarField;

pub type VerifierKeyFor<PairingT> =
    <Groth16<PairingT> as SNARK<<PairingT as Pairing>::ScalarField>>::VerifyingKey;

pub type ProofFor<PairingT> =
    <Groth16<PairingT> as SNARK<<PairingT as Pairing>::ScalarField>>::Proof;

pub fn groth16_verify<P: Pairing>(vk: Vec<u8>, c: Vec<u8>, proof: Vec<u8>) {
    let vk = deserialize_uncompressed_host::<VerifierKeyFor<P>>(vk);
    let c = deserialize_uncompressed_host::<ScalarFieldFor<P>>(c);
    let proof = deserialize_uncompressed_host::<ProofFor<P>>(proof);
    let result = Groth16::<P>::verify(&vk, &[c], &proof).unwrap();
    assert!(result);
}

#[frame_support::pallet]
pub mod pallet {

    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // ---------------------------------------------
        // Calls for bls12-381
        // ---------------------------------------------

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bls12_381_groth16_verify(
            _: OriginFor<T>,
            vk: Vec<u8>,
            c: Vec<u8>,
            proof: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                groth16_verify::<sub_bls12_381::Bls12_381>(vk, c, proof);
            } else {
                groth16_verify::<ark_bls12_381::Bls12_381>(vk, c, proof);
            }
            Ok(())
        }
    }
}
