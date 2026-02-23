#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod utils;
mod weights;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

use ark_ec::{
    pairing::Pairing,
    short_weierstrass::{Affine as SWAffine, Projective as SWProjective, SWCurveConfig},
    twisted_edwards::{Affine as TEAffine, Projective as TEProjective, TECurveConfig},
    AffineRepr,
};
use ark_ff::One;
use ark_scale::{
    hazmat::ArkScaleProjective,
    scale::{Decode, Encode, MaxEncodedLen},
};
use ark_std::{vec, vec::Vec};

pub use sp_crypto_ec_utils::{
    bls12_381 as sub_bls12_381, ed_on_bls12_377 as sub_ed_on_bls12_377,
    ed_on_bls12_381_bandersnatch as sub_ed_on_bls12_381_bandersnatch,
};

pub type ScalarFieldFor<AffineT> = <AffineT as AffineRepr>::ScalarField;

// Compressed and validated. Used to decode data from untrusted domain (e.g. extrinsics).
type ArkScale<T> = ark_scale::ArkScale<T, { ark_scale::WIRE }>;
// Uncompressed and not validated. Used to pass data from/to trusted domain (e.g. host calls).
type ArkScaleHostCall<T> = ark_scale::ArkScale<T, { ark_scale::HOST_CALL }>;

pub use pallet::*;
pub use weights::*;

const DEFAULT_WEIGHT: u64 = 10_000;

fn msm_sw<C: SWCurveConfig>(bases: Vec<u8>, scalars: Vec<u8>) {
    let bases = ArkScale::<Vec<SWAffine<C>>>::decode(&mut bases.as_slice()).unwrap();
    let scalars = ArkScale::<Vec<C::ScalarField>>::decode(&mut scalars.as_slice()).unwrap();
    let _ = C::msm(&bases.0, &scalars.0).unwrap();
}

fn mul_projective_sw<C: SWCurveConfig>(base: Vec<u8>, scalar: Vec<u8>) {
    let base = ArkScaleProjective::<SWProjective<C>>::decode(&mut base.as_slice()).unwrap();
    let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
    let _ = C::mul_projective(&base.0, &scalar.0);
}

fn mul_affine_sw<C: SWCurveConfig>(base: Vec<u8>, scalar: Vec<u8>) {
    let base = ArkScale::<SWAffine<C>>::decode(&mut base.as_slice()).unwrap();
    let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
    let _ = C::mul_affine(&base.0, &scalar.0);
}

fn msm_te<C: TECurveConfig>(bases: Vec<u8>, scalars: Vec<u8>) {
    let bases = ArkScale::<Vec<TEAffine<C>>>::decode(&mut bases.as_slice()).unwrap();
    let scalars = ArkScale::<Vec<C::ScalarField>>::decode(&mut scalars.as_slice()).unwrap();
    let _ = C::msm(&bases.0, &scalars.0).unwrap();
}

fn mul_projective_te<C: TECurveConfig>(base: Vec<u8>, scalar: Vec<u8>) {
    let base = ArkScaleProjective::<TEProjective<C>>::decode(&mut base.as_slice()).unwrap();
    let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
    let _ = C::mul_projective(&base.0, &scalar.0);
}

fn mul_affine_te<C: TECurveConfig>(base: Vec<u8>, scalar: Vec<u8>) {
    let base = ArkScale::<TEAffine<C>>::decode(&mut base.as_slice()).unwrap();
    let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
    let _ = C::mul_affine(&base.0, &scalar.0);
}

fn pairing<P: Pairing>(a: Vec<u8>, b: Vec<u8>) {
    let a = ArkScale::<P::G1Affine>::decode(&mut a.as_slice()).unwrap();
    let b = ArkScale::<P::G2Affine>::decode(&mut b.as_slice()).unwrap();
    let _ = P::multi_pairing([a.0], [b.0]);
}

/// Verify a BLS signature.
///
/// Checks: e(signature, G2_generator) == e(public_key, message_hash)
/// Via the equivalent: e(signature, G2_generator) * e(-public_key, message_hash) == 1
///
/// If `direct` is true, calls into the host functions directly.
/// If `direct` is false, uses the `Pairing` interface (which calls the host functions under the hood).
fn verify_bls_signature(
    public_key: Vec<u8>,
    message_hash: Vec<u8>,
    signature: Vec<u8>,
    direct: bool,
) -> bool {
    let pk = ArkScale::<sub_bls12_381::G1Affine>::decode(&mut public_key.as_slice())
        .unwrap()
        .0;
    let msg = ArkScale::<sub_bls12_381::G2Affine>::decode(&mut message_hash.as_slice())
        .unwrap()
        .0;
    let sig = ArkScale::<sub_bls12_381::G1Affine>::decode(&mut signature.as_slice())
        .unwrap()
        .0;

    let g2_gen = sub_bls12_381::G2Affine::generator();

    if direct {
        type TargetField = <sub_bls12_381::Bls12_381 as Pairing>::TargetField;

        let g1: ArkScaleHostCall<_> = vec![sig, -pk].into();
        let g2: ArkScaleHostCall<_> = vec![g2_gen, msg].into();
        let g1 = g1.encode();
        let g2 = g2.encode();

        let mut buf = vec![0u8; ArkScaleHostCall::<TargetField>::max_encoded_len()];
        sub_bls12_381::host_calls::bls12_381_multi_miller_loop(&g1, &g2, &mut buf).unwrap();
        sub_bls12_381::host_calls::bls12_381_final_exponentiation(&mut buf).unwrap();

        let result = ArkScaleHostCall::<TargetField>::decode(&mut buf.as_slice())
            .unwrap()
            .0;
        result.is_one()
    } else {
        sub_bls12_381::Bls12_381::multi_pairing([sig, -pk], [g2_gen, msg])
            .0
            .is_one()
    }
}

#[frame_support::pallet]
pub mod pallet {

    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Extrinsic weights
        type WeightInfo: WeightInfo;
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // ---------------------------------------------
        // Calls for bls12-381
        // ---------------------------------------------

        #[pallet::call_index(10)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bls12_381_pairing(
            _: OriginFor<T>,
            a: Vec<u8>,
            b: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                pairing::<sub_bls12_381::Bls12_381>(a, b);
            } else {
                pairing::<ark_bls12_381::Bls12_381>(a, b);
            }
            Ok(())
        }

        #[pallet::call_index(11)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bls12_381_msm_g1(
            _: OriginFor<T>,
            bases: Vec<u8>,
            scalars: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                msm_sw::<sub_bls12_381::G1Config>(bases, scalars);
            } else {
                msm_sw::<ark_bls12_381::g1::Config>(bases, scalars);
            }
            Ok(())
        }

        #[pallet::call_index(12)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bls12_381_mul_projective_g1(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                mul_projective_sw::<sub_bls12_381::G1Config>(base, scalar);
            } else {
                mul_projective_sw::<ark_bls12_381::g1::Config>(base, scalar);
            }
            Ok(())
        }

        #[pallet::call_index(13)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bls12_381_mul_affine_g1(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                mul_affine_sw::<sub_bls12_381::G1Config>(base, scalar);
            } else {
                mul_affine_sw::<ark_bls12_381::g1::Config>(base, scalar);
            }
            Ok(())
        }

        #[pallet::call_index(14)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bls12_381_msm_g2(
            _: OriginFor<T>,
            bases: Vec<u8>,
            scalars: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                msm_sw::<sub_bls12_381::G2Config>(bases, scalars);
            } else {
                msm_sw::<ark_bls12_381::g2::Config>(bases, scalars);
            }
            Ok(())
        }

        #[pallet::call_index(15)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bls12_381_mul_projective_g2(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                mul_projective_sw::<sub_bls12_381::G2Config>(base, scalar);
            } else {
                mul_projective_sw::<ark_bls12_381::g2::Config>(base, scalar);
            }
            Ok(())
        }

        #[pallet::call_index(16)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bls12_381_mul_affine_g2(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                mul_affine_sw::<sub_bls12_381::G2Config>(base, scalar);
            } else {
                mul_affine_sw::<ark_bls12_381::g2::Config>(base, scalar);
            }
            Ok(())
        }

        // ---------------------------------------------
        // Calls for ed-on-bls12-377
        // ---------------------------------------------

        #[pallet::call_index(20)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_377_msm_te(
            _: OriginFor<T>,
            bases: Vec<u8>,
            scalars: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                msm_te::<sub_ed_on_bls12_377::EdwardsConfig>(bases, scalars)
            } else {
                msm_te::<ark_ed_on_bls12_377::EdwardsConfig>(bases, scalars)
            }
            Ok(())
        }

        #[pallet::call_index(21)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_377_mul_projective_te(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                mul_projective_te::<sub_ed_on_bls12_377::EdwardsConfig>(base, scalar);
            } else {
                mul_projective_te::<ark_ed_on_bls12_377::EdwardsConfig>(base, scalar);
            }
            Ok(())
        }

        #[pallet::call_index(22)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_377_mul_affine_te(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                mul_affine_te::<sub_ed_on_bls12_377::EdwardsConfig>(base, scalar);
            } else {
                mul_affine_te::<ark_ed_on_bls12_377::EdwardsConfig>(base, scalar);
            }
            Ok(())
        }

        // ---------------------------------------------
        // Calls for ed-on-bls12-381-bandersnatch
        // ---------------------------------------------

        // Short Weierstrass

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_msm_sw(
            _: OriginFor<T>,
            bases: Vec<u8>,
            scalars: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                msm_sw::<sub_ed_on_bls12_381_bandersnatch::SWConfig>(bases, scalars)
            } else {
                msm_sw::<ark_ed_on_bls12_381_bandersnatch::SWConfig>(bases, scalars)
            }
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_mul_projective_sw(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                mul_projective_sw::<sub_ed_on_bls12_381_bandersnatch::SWConfig>(base, scalar);
            } else {
                mul_projective_sw::<ark_ed_on_bls12_381_bandersnatch::SWConfig>(base, scalar);
            }
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_mul_affine_sw(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                mul_affine_sw::<sub_ed_on_bls12_381_bandersnatch::SWConfig>(base, scalar);
            } else {
                mul_affine_sw::<ark_ed_on_bls12_381_bandersnatch::SWConfig>(base, scalar);
            }
            Ok(())
        }

        // Twisted Edwards

        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_msm_te(
            _origin: OriginFor<T>,
            bases: Vec<u8>,
            scalars: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                msm_te::<sub_ed_on_bls12_381_bandersnatch::EdwardsConfig>(bases, scalars)
            } else {
                msm_te::<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig>(bases, scalars)
            }
            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_mul_projective_te(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                mul_projective_te::<sub_ed_on_bls12_381_bandersnatch::EdwardsConfig>(base, scalar);
            } else {
                mul_projective_te::<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig>(base, scalar);
            }
            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_mul_affine_te(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                mul_affine_te::<sub_ed_on_bls12_381_bandersnatch::EdwardsConfig>(base, scalar);
            } else {
                mul_affine_te::<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig>(base, scalar);
            }
            Ok(())
        }

        #[pallet::call_index(7)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn bls12_381_verify_bls_signature(
            _: OriginFor<T>,
            public_key: Vec<u8>,
            message_hash: Vec<u8>,
            signature: Vec<u8>,
            direct: bool,
        ) -> DispatchResult {
            ensure!(
                verify_bls_signature(public_key, message_hash, signature, direct),
                DispatchError::Other("Invalid BLS signature")
            );
            Ok(())
        }
    }
}
