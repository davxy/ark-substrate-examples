//! A shell pallet built with [`frame`].
//!
//! To get started with this pallet, try implementing the guide in
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod utils;

use frame::prelude::*;
use polkadot_sdk::polkadot_sdk_frame as frame;

use ark_ec::{short_weierstrass::SWCurveConfig, twisted_edwards::TECurveConfig, AffineRepr};
use ark_scale::{hazmat::ArkScaleProjective, scale::Decode};
use ark_std::vec::Vec;

pub use sp_crypto_ec_utils::ed_on_bls12_381_bandersnatch as sub_ed_on_bls12_381_bandersnatch;

pub type ScalarFieldFor<AffineT> = <AffineT as AffineRepr>::ScalarField;

type ArkScale<T> = ark_scale::ArkScale<T>;

pub use pallet::*;

const MSM_LEN: u32 = 10;
// const SCALAR_WORDS: u32 = 3;

const DEFAULT_WEIGHT: u64 = 10_000;

pub fn ed_on_bls12_381_bandersnatch_msm_sw<C: SWCurveConfig>(
    bases: Vec<u8>,
    scalars: Vec<u8>,
) -> DispatchResult {
    use ark_ec::short_weierstrass::Affine;
    let bases = ArkScale::<Vec<Affine<C>>>::decode(&mut bases.as_slice()).unwrap();
    let scalars = ArkScale::<Vec<C::ScalarField>>::decode(&mut scalars.as_slice()).unwrap();
    let _ = C::msm(&bases.0, &scalars.0);
    Ok(())
}

#[frame::pallet]
pub mod pallet {

    use super::*;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // ---------------------------------------------
        // Calls for ed-on-bls12-381-bandersnatch
        // ---------------------------------------------

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
        }

        #[pallet::call_index(144)]
        #[pallet::weight(Weight::from_all(10_000))]
        pub fn ed_on_bls12_381_bandersnatch_msm_sw_rand(
            origin: OriginFor<T>,
            items: u32,
            optimized: bool,
        ) -> DispatchResult {
            let (bases, scalars) =
                utils::make_msm_args::<ark_ed_on_bls12_381_bandersnatch::SWProjective>(items);
            Self::ed_on_bls12_381_bandersnatch_msm_sw(
                origin,
                bases.encode(),
                scalars.encode(),
                optimized,
            )
        }

        #[pallet::call_index(46)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(0).ref_time())]
        pub fn ark_ed_on_bls12_381_bandersnatch_msm_te(
            _origin: OriginFor<T>,
            bases: Vec<u8>,
            scalars: Vec<u8>,
        ) -> DispatchResult {
            let bases = ArkScale::<Vec<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>>::decode(
                &mut bases.as_slice(),
            )
            .unwrap();
            let scalars = ArkScale::<
                Vec<ScalarFieldFor<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>>,
            >::decode(&mut scalars.as_slice())
            .unwrap();
            let _ = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as TECurveConfig>::msm(
                &bases.0, &scalars.0,
            );
            Ok(())
        }

        #[pallet::call_index(47)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(0).ref_time())]
        pub fn sub_ed_on_bls12_381_bandersnatch_msm_te(
            _origin: OriginFor<T>,
            bases: Vec<u8>,
            scalars: Vec<u8>,
        ) -> DispatchResult {
            let bases = ArkScale::<Vec<sub_ed_on_bls12_381_bandersnatch::EdwardsAffine>>::decode(
                &mut bases.as_slice(),
            )
            .unwrap();
            let scalars = ArkScale::<
                Vec<ScalarFieldFor<sub_ed_on_bls12_381_bandersnatch::EdwardsAffine>>,
            >::decode(&mut scalars.as_slice())
            .unwrap();
            let _ = <sub_ed_on_bls12_381_bandersnatch::EdwardsConfig as TECurveConfig>::msm(
                &bases.0, &scalars.0,
            );
            Ok(())
        }

        #[pallet::call_index(48)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(0).ref_time())]
        pub fn ark_ed_on_bls12_381_bandersnatch_mul_projective_sw(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base =
                ArkScaleProjective::<ark_ed_on_bls12_381_bandersnatch::SWProjective>::decode(
                    &mut base.as_slice(),
                )
                .unwrap();
            let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
            let _ = <ark_ed_on_bls12_381_bandersnatch::SWConfig as SWCurveConfig>::mul_projective(
                &base.0, &scalar.0,
            );
            Ok(())
        }

        #[pallet::call_index(49)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(0).ref_time())]
        pub fn sub_ed_on_bls12_381_bandersnatch_mul_projective_sw(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base =
                ArkScaleProjective::<sub_ed_on_bls12_381_bandersnatch::SWProjective>::decode(
                    &mut base.as_slice(),
                )
                .unwrap();
            let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
            let _ = <sub_ed_on_bls12_381_bandersnatch::SWConfig as SWCurveConfig>::mul_projective(
                &base.0, &scalar.0,
            );
            Ok(())
        }

        #[pallet::call_index(50)]
        #[pallet::weight(Weight::from_all(10_000))]
        pub fn ark_ed_on_bls12_381_bandersnatch_mul_projective_te(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base =
                ArkScaleProjective::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>::decode(
                    &mut base.as_slice(),
                )
                .unwrap();
            let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
            let _ =
                <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as TECurveConfig>::mul_projective(
                    &base.0, &scalar.0,
                );
            Ok(())
        }

        #[pallet::call_index(51)]
        #[pallet::weight(Weight::from_all(10_000))]
        pub fn sub_ed_on_bls12_381_bandersnatch_mul_projective_te(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base =
                ArkScaleProjective::<sub_ed_on_bls12_381_bandersnatch::EdwardsProjective>::decode(
                    &mut base.as_slice(),
                )
                .unwrap();
            let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
            let _ =
                <sub_ed_on_bls12_381_bandersnatch::EdwardsConfig as TECurveConfig>::mul_projective(
                    &base.0, &scalar.0,
                );
            Ok(())
        }

        #[pallet::call_index(52)]
        #[pallet::weight(Weight::from_all(10_000))]
        pub fn ark_ed_on_bls12_381_bandersnatch_mul_affine_sw(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base = ArkScale::<ark_ed_on_bls12_381_bandersnatch::SWAffine>::decode(
                &mut base.as_slice(),
            )
            .unwrap();
            let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
            let _ = <ark_ed_on_bls12_381_bandersnatch::SWConfig as SWCurveConfig>::mul_affine(
                &base.0, &scalar.0,
            );
            Ok(())
        }

        #[pallet::call_index(53)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn sub_ed_on_bls12_381_bandersnatch_mul_affine_sw(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base = ArkScale::<sub_ed_on_bls12_381_bandersnatch::SWAffine>::decode(
                &mut base.as_slice(),
            )
            .unwrap();
            let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
            let _ = <sub_ed_on_bls12_381_bandersnatch::SWConfig as SWCurveConfig>::mul_affine(
                &base.0, &scalar.0,
            );
            Ok(())
        }

        #[pallet::call_index(54)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn ark_ed_on_bls12_381_bandersnatch_mul_affine_te(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base = ArkScale::<ark_ed_on_bls12_381_bandersnatch::EdwardsAffine>::decode(
                &mut base.as_slice(),
            )
            .unwrap();
            let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
            let _ = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as TECurveConfig>::mul_affine(
                &base.0, &scalar.0,
            );
            Ok(())
        }

        #[pallet::call_index(55)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn sub_ed_on_bls12_381_bandersnatch_mul_affine_te(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base = ArkScale::<sub_ed_on_bls12_381_bandersnatch::EdwardsAffine>::decode(
                &mut base.as_slice(),
            )
            .unwrap();
            let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();
            let _ = <sub_ed_on_bls12_381_bandersnatch::EdwardsConfig as TECurveConfig>::mul_affine(
                &base.0, &scalar.0,
            );
            Ok(())
        }
    }
}
