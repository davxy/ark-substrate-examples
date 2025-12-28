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
    short_weierstrass::{Affine as SWAffine, Projective as SWProjective, SWCurveConfig},
    twisted_edwards::{Affine as TEAffine, Projective as TEProjective, TECurveConfig},
    AffineRepr,
};
use ark_scale::{hazmat::ArkScaleProjective, scale::Decode};
use ark_std::vec::Vec;

pub use sp_crypto_ec_utils::ed_on_bls12_381_bandersnatch as sub_ed_on_bls12_381_bandersnatch;

pub type ScalarFieldFor<AffineT> = <AffineT as AffineRepr>::ScalarField;

type ArkScale<T> = ark_scale::ArkScale<T>;

pub use pallet::*;

const DEFAULT_WEIGHT: u64 = 10_000;

fn ed_on_bls12_381_bandersnatch_msm_sw<C: SWCurveConfig>(bases: Vec<u8>, scalars: Vec<u8>) {
    let bases = ArkScale::<Vec<SWAffine<C>>>::decode(&mut bases.as_slice()).unwrap();
    let scalars = ArkScale::<Vec<C::ScalarField>>::decode(&mut scalars.as_slice()).unwrap();
    let _ = C::msm(&bases.0, &scalars.0).unwrap();
}

fn ed_on_bls12_381_bandersnatch_mul_projective_sw<C: SWCurveConfig>(
    base: Vec<u8>,
    scalar: Vec<u8>,
) {
    let base = ArkScaleProjective::<SWProjective<C>>::decode(&mut base.as_slice()).unwrap();
    let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
    let _ = C::mul_projective(&base.0, &scalar.0);
}

fn ed_on_bls12_381_bandersnatch_mul_affine_sw<C: SWCurveConfig>(base: Vec<u8>, scalar: Vec<u8>) {
    let base = ArkScale::<SWAffine<C>>::decode(&mut base.as_slice()).unwrap();
    let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
    let _ = C::mul_affine(&base.0, &scalar.0);
}

fn ed_on_bls12_381_bandersnatch_msm_te<C: TECurveConfig>(bases: Vec<u8>, scalars: Vec<u8>) {
    let bases = ArkScale::<Vec<TEAffine<C>>>::decode(&mut bases.as_slice()).unwrap();
    let scalars = ArkScale::<Vec<C::ScalarField>>::decode(&mut scalars.as_slice()).unwrap();
    let _ = C::msm(&bases.0, &scalars.0).unwrap();
}

fn ed_on_bls12_381_bandersnatch_mul_projective_te<C: TECurveConfig>(
    base: Vec<u8>,
    scalar: Vec<u8>,
) {
    let base = ArkScaleProjective::<TEProjective<C>>::decode(&mut base.as_slice()).unwrap();
    let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
    let _ = C::mul_projective(&base.0, &scalar.0);
}

fn ed_on_bls12_381_bandersnatch_mul_affine_te<C: TECurveConfig>(base: Vec<u8>, scalar: Vec<u8>) {
    let base = ArkScale::<TEAffine<C>>::decode(&mut base.as_slice()).unwrap();
    let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
    let _ = C::mul_affine(&base.0, &scalar.0);
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
        // Calls for ed-on-bls12-381-bandersnatch
        // ---------------------------------------------

        // Short Weierstrass

        #[pallet::call_index(44)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_msm_sw(
            _: OriginFor<T>,
            bases: Vec<u8>,
            scalars: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                ed_on_bls12_381_bandersnatch_msm_sw::<sub_ed_on_bls12_381_bandersnatch::SWConfig>(
                    bases, scalars,
                )
            } else {
                ed_on_bls12_381_bandersnatch_msm_sw::<ark_ed_on_bls12_381_bandersnatch::SWConfig>(
                    bases, scalars,
                )
            }
            Ok(())
        }

        #[pallet::call_index(48)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_mul_projective_sw(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                ed_on_bls12_381_bandersnatch_mul_projective_sw::<
                    sub_ed_on_bls12_381_bandersnatch::SWConfig,
                >(base, scalar);
            } else {
                ed_on_bls12_381_bandersnatch_mul_projective_sw::<
                    ark_ed_on_bls12_381_bandersnatch::SWConfig,
                >(base, scalar);
            }
            Ok(())
        }

        #[pallet::call_index(52)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_mul_affine_sw(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                ed_on_bls12_381_bandersnatch_mul_affine_sw::<
                    sub_ed_on_bls12_381_bandersnatch::SWConfig,
                >(base, scalar);
            } else {
                ed_on_bls12_381_bandersnatch_mul_affine_sw::<
                    ark_ed_on_bls12_381_bandersnatch::SWConfig,
                >(base, scalar);
            }
            Ok(())
        }

        // Twisted Edwards

        #[pallet::call_index(46)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_msm_te(
            _origin: OriginFor<T>,
            bases: Vec<u8>,
            scalars: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                ed_on_bls12_381_bandersnatch_msm_te::<sub_ed_on_bls12_381_bandersnatch::EdwardsConfig>(
                    bases, scalars,
                )
            } else {
                ed_on_bls12_381_bandersnatch_msm_te::<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig>(
                    bases, scalars,
                )
            }
            Ok(())
        }

        #[pallet::call_index(51)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_mul_projective_te(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                ed_on_bls12_381_bandersnatch_mul_projective_te::<
                    sub_ed_on_bls12_381_bandersnatch::EdwardsConfig,
                >(base, scalar);
            } else {
                ed_on_bls12_381_bandersnatch_mul_projective_te::<
                    ark_ed_on_bls12_381_bandersnatch::EdwardsConfig,
                >(base, scalar);
            }
            Ok(())
        }

        #[pallet::call_index(54)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ed_on_bls12_381_bandersnatch_mul_affine_te(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                ed_on_bls12_381_bandersnatch_mul_affine_te::<
                    sub_ed_on_bls12_381_bandersnatch::EdwardsConfig,
                >(base, scalar);
            } else {
                ed_on_bls12_381_bandersnatch_mul_affine_te::<
                    ark_ed_on_bls12_381_bandersnatch::EdwardsConfig,
                >(base, scalar);
            }
            Ok(())
        }
    }
}
