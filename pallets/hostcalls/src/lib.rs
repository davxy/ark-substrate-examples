//! A shell pallet built with [`frame`].
//!
//! To get started with this pallet, try implementing the guide in
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>

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

use ark_ec::{
    pairing::Pairing,
    short_weierstrass::{Affine as SWAffine, Projective as SWProjective, SWCurveConfig},
    twisted_edwards::{Affine as TEAffine, Projective as TEProjective, TECurveConfig},
    AffineRepr,
};
use ark_scale::{hazmat::ArkScaleProjective, scale::Decode};
use ark_std::vec::Vec;

pub use sp_crypto_ec_utils::{
    bls12_381 as sub_bls12_381, ed_on_bls12_377 as sub_ed_on_bls12_377,
    ed_on_bls12_381_bandersnatch as sub_ed_on_bls12_381_bandersnatch,
};

pub type ScalarFieldFor<AffineT> = <AffineT as AffineRepr>::ScalarField;

type ArkScale<T> = ark_scale::ArkScale<T>;

pub use pallet::*;

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
    }
}
