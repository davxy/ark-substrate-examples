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
use log;

use ark_vrf::reexports::ark_std::vec::Vec;

use ark_vrf::suites::bandersnatch as ark_bandersnatch;
pub(crate) type ArkSuite = ark_bandersnatch::BandersnatchSha512Ell2;

pub(crate) type RingBuilderPcsParams =
    ark_vrf::ring::RingBuilderPcsParams<ark_bandersnatch::BandersnatchSha512Ell2>;

const fn max_ring_size_from_pcs_domain_size(pcs_domain_size: usize) -> usize {
    ark_vrf::ring::max_ring_size_from_pcs_domain_size::<ArkSuite>(pcs_domain_size)
}

#[cfg(feature = "small-ring")]
mod ring_params {
    pub const RING_BUILDER_DATA: &[u8] = include_bytes!("static/ring-builder-small.bin");
    pub const RING_BUILDER_PARAMS: &[u8] = include_bytes!("static/ring-builder-params-small.bin");
    pub const MAX_RING_SIZE: usize = super::max_ring_size_from_pcs_domain_size(1 << 11);
}

#[cfg(not(feature = "small-ring"))]
mod ring_params {
    pub const RING_BUILDER_DATA: &[u8] = include_bytes!("static/ring-builder-full.bin");
    pub const RING_BUILDER_PARAMS: &[u8] = include_bytes!("static/ring-builder-params-full.bin");
    pub const MAX_RING_SIZE: usize = super::max_ring_size_from_pcs_domain_size(1 << 16);
}

pub(crate) use ring_params::*;

mod sub_bandersnatch {
    use ark_vrf::{
        pedersen::PedersenSuite, ring::RingSuite, ring_suite_types, suite_types,
        suites::bandersnatch::BandersnatchSha512Ell2, Suite,
    };

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct BandersnatchSuite;

    impl Suite for BandersnatchSuite {
        const SUITE_ID: &'static [u8] = BandersnatchSha512Ell2::SUITE_ID;
        const CHALLENGE_LEN: usize = BandersnatchSha512Ell2::CHALLENGE_LEN;
        type Affine = sp_crypto_ec_utils::ed_on_bls12_381_bandersnatch::EdwardsAffine;
        type Hasher = <BandersnatchSha512Ell2 as Suite>::Hasher;
        type Codec = <BandersnatchSha512Ell2 as Suite>::Codec;
    }

    impl PedersenSuite for BandersnatchSuite {
        const BLINDING_BASE: AffinePoint = AffinePoint::new_unchecked(
            BandersnatchSha512Ell2::BLINDING_BASE.x,
            BandersnatchSha512Ell2::BLINDING_BASE.y,
        );
    }

    impl RingSuite for BandersnatchSuite {
        type Pairing = sp_crypto_ec_utils::bls12_381::Bls12_381;
        const ACCUMULATOR_BASE: AffinePoint = AffinePoint::new_unchecked(
            BandersnatchSha512Ell2::ACCUMULATOR_BASE.x,
            BandersnatchSha512Ell2::ACCUMULATOR_BASE.y,
        );
        const PADDING: AffinePoint = AffinePoint::new_unchecked(
            BandersnatchSha512Ell2::PADDING.x,
            BandersnatchSha512Ell2::PADDING.y,
        );
    }

    suite_types!(BandersnatchSuite);

    ring_suite_types!(BandersnatchSuite);
}

pub(crate) type SubSuite = sub_bandersnatch::BandersnatchSuite;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

const DEFAULT_WEIGHT: u64 = 10_000;

const SRS_PAGE_SIZE: usize = 1 << 3;

type SrsItem = ark_vrf::ring::G1Affine<ark_bandersnatch::BandersnatchSha512Ell2>;

const COMPRESSED_POINT_SIZE: usize = 32;

const IETF_PROOF_SERIALIZED_SIZE: usize = 64;

const RING_PROOF_SERIALIZED_SIZE: usize = 752;

const RING_VERIFIER_KEY_SERIALIZED_SIZE: usize = 384;

const SRS_ITEM_SERIALIZED_SIZE: usize = 48;
const RING_BUILDER_SERIALIZED_SIZE: usize = 848;

#[derive(
    Clone,
    Eq,
    PartialEq,
    Debug,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
    DecodeWithMemTracking,
    Default,
)]
pub struct CompressedPoint(pub [u8; COMPRESSED_POINT_SIZE]);

pub type PublicKeyRaw = CompressedPoint;
pub type InputRaw = CompressedPoint;
pub type OutputRaw = CompressedPoint;

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Debug,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
    DecodeWithMemTracking,
)]
pub struct SrsItemRaw(pub [u8; SRS_ITEM_SERIALIZED_SIZE]);

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct SrsPage(pub [SrsItemRaw; SRS_PAGE_SIZE]);

impl Default for SrsPage {
    fn default() -> Self {
        Self([SrsItemRaw([0_u8; SRS_ITEM_SERIALIZED_SIZE]); SRS_PAGE_SIZE])
    }
}

#[derive(MaxEncodedLen, Encode, Decode, TypeInfo)]
pub struct RingBuilderRaw(pub [u8; RING_BUILDER_SERIALIZED_SIZE]);

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    MaxEncodedLen,
    Encode,
    Decode,
    TypeInfo,
    DecodeWithMemTracking,
    Debug,
)]
pub struct IetfProofRaw(pub [u8; IETF_PROOF_SERIALIZED_SIZE]);

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    MaxEncodedLen,
    Encode,
    Decode,
    TypeInfo,
    DecodeWithMemTracking,
    Debug,
)]
pub struct RingProofRaw(pub [u8; RING_PROOF_SERIALIZED_SIZE]);

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    MaxEncodedLen,
    Encode,
    Decode,
    TypeInfo,
    DecodeWithMemTracking,
    Debug,
)]
pub struct RingVerifierKeyRaw(pub [u8; RING_VERIFIER_KEY_SERIALIZED_SIZE]);

#[frame_support::pallet]
pub mod pallet {
    use core::ops::Range;

    use ark_vrf::{
        ietf::IetfSuite,
        reexports::ark_serialize::{CanonicalDeserialize, CanonicalSerialize},
        ring::RingSuite,
    };
    use frame_system::pallet_prelude::OriginFor;

    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Maximum number of people included in a ring before a new one is created.
        #[pallet::constant]
        type MaxRingSize: Get<u32>;
    }

    #[pallet::storage]
    pub type Srs<T: Config> = StorageMap<_, Twox64Concat, u32, SrsPage>;

    #[pallet::storage]
    pub type RingSize<T: Config> = StorageValue<_, u32>;

    #[pallet::storage]
    pub type RingBuilder<T> = StorageValue<_, RingBuilderRaw>;

    #[pallet::storage]
    pub type RingKeys<T: Config> = StorageValue<_, BoundedVec<PublicKeyRaw, T::MaxRingSize>>;

    #[pallet::storage]
    pub type RingVerifierKey<T: Config> = StorageValue<_, RingVerifierKeyRaw>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        #[serde(skip)]
        pub _phantom_data: core::marker::PhantomData<T>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                _phantom_data: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            log::info!("Building paged SRS");
            let builder_pcs_params =
                RingBuilderPcsParams::deserialize_uncompressed_unchecked(RING_BUILDER_PARAMS)
                    .unwrap();
            let mut srs_page = SrsPage::default();
            for (i, item) in builder_pcs_params.0.iter().enumerate() {
                let page_off = i % SRS_PAGE_SIZE;
                let raw = &mut srs_page.0[page_off];
                item.serialize_compressed(&mut raw.0[..]).unwrap();
                if page_off == SRS_PAGE_SIZE - 1 {
                    let page_idx = i / SRS_PAGE_SIZE;
                    Srs::<T>::insert(page_idx as u32, srs_page.clone());
                }
            }

            Pallet::<T>::ring_reset_impl();
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // ---------------------------------------------
        // Calls for ring-vrf
        // ---------------------------------------------

        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ring_reset(_: OriginFor<T>) -> DispatchResult {
            Self::ring_reset_impl();
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn push_members(
            _: OriginFor<T>,
            new_members: Vec<PublicKeyRaw>,
            optimized: bool,
        ) -> DispatchResult {
            Self::increment_ring_size(new_members.len() as u32);
            if optimized {
                Self::push_members_impl::<SubSuite>(new_members);
            } else {
                Self::push_members_impl::<ArkSuite>(new_members);
            }
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn push_member_buffered(_: OriginFor<T>, member: PublicKeyRaw) -> DispatchResult {
            Self::increment_ring_size(1);
            let mut members = RingKeys::<T>::get().unwrap_or_default();
            members.try_push(member).expect("Ring is full");
            log::debug!("Pushed new member, current ring size {}", members.len());
            RingKeys::<T>::set(Some(members));
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ring_commit(origin: OriginFor<T>, optimized: bool) -> DispatchResult {
            let buffered_members = RingKeys::<T>::get().unwrap_or_default();
            if !buffered_members.is_empty() {
                Self::push_members(origin, buffered_members.to_vec(), optimized)?;
            }

            if optimized {
                Self::commit_impl::<SubSuite>();
            } else {
                Self::commit_impl::<ArkSuite>();
            }

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ring_verify(
            _: OriginFor<T>,
            input_raw: InputRaw,
            output_raw: OutputRaw,
            proof_raw: RingProofRaw,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                Self::ring_verify_impl::<SubSuite>(input_raw, output_raw, proof_raw);
            } else {
                Self::ring_verify_impl::<ArkSuite>(input_raw, output_raw, proof_raw);
            }
            Ok(())
        }

        // ---------------------------------------------
        // Calls for ietf-vrf
        // ---------------------------------------------

        #[pallet::call_index(10)]
        #[pallet::weight(Weight::from_all(DEFAULT_WEIGHT))]
        pub fn ietf_verify(
            _: OriginFor<T>,
            public_raw: PublicKeyRaw,
            input_raw: InputRaw,
            output_raw: OutputRaw,
            proof_raw: IetfProofRaw,
            optimized: bool,
        ) -> DispatchResult {
            if optimized {
                Self::ietf_verify_impl::<SubSuite>(public_raw, input_raw, output_raw, proof_raw);
            } else {
                Self::ietf_verify_impl::<ArkSuite>(public_raw, input_raw, output_raw, proof_raw);
            }
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub(crate) fn increment_ring_size(new_members_count: u32) {
            let members_count = RingSize::<T>::get().unwrap_or_default() + new_members_count;
            if members_count > T::MaxRingSize::get() {
                panic!("Ring overflow");
            }
            log::debug!("Pushing {new_members_count} new member, total ring size {members_count}");
            RingSize::<T>::set(Some(members_count));
        }

        pub(crate) fn ietf_verify_impl<S: IetfSuite>(
            public_raw: PublicKeyRaw,
            input_raw: InputRaw,
            output_raw: OutputRaw,
            proof_raw: IetfProofRaw,
        ) {
            use ark_vrf::ietf::Verifier;
            let input =
                ark_vrf::Input::<S>::deserialize_compressed_unchecked(&input_raw.0[..]).unwrap();
            let output =
                ark_vrf::Output::<S>::deserialize_compressed_unchecked(&output_raw.0[..]).unwrap();
            let public =
                ark_vrf::Public::<S>::deserialize_compressed_unchecked(&public_raw.0[..]).unwrap();
            let proof =
                ark_vrf::ietf::Proof::<S>::deserialize_compressed_unchecked(&proof_raw.0[..])
                    .unwrap();
            public.verify(input, output, &[], &proof).unwrap();
        }

        pub(crate) fn ring_verify_impl<S: RingSuite>(
            input_raw: InputRaw,
            output_raw: OutputRaw,
            proof_raw: RingProofRaw,
        ) {
            use ark_vrf::ring::Verifier;
            let input =
                ark_vrf::Input::<S>::deserialize_compressed_unchecked(&input_raw.0[..]).unwrap();
            let output =
                ark_vrf::Output::<S>::deserialize_compressed_unchecked(&output_raw.0[..]).unwrap();
            let proof =
                ark_vrf::ring::Proof::<S>::deserialize_compressed_unchecked(&proof_raw.0[..])
                    .unwrap();

            let verifier_key_raw = RingVerifierKey::<T>::get().unwrap();
            let verifier_key =
                ark_vrf::ring::RingVerifierKey::<S>::deserialize_compressed_unchecked(
                    &verifier_key_raw.0[..],
                )
                .unwrap();

            let ring_size = RingSize::<T>::get().unwrap_or_default();
            let verifier = ark_vrf::ring::RingProofParams::<S>::verifier_no_context(
                verifier_key,
                ring_size as usize,
            );

            ark_vrf::Public::<S>::verify(input, output, &[], &proof, &verifier).unwrap();
        }

        pub(crate) fn commit_impl<S: RingSuite>() {
            let builder_raw = RingBuilder::<T>::get().unwrap();
            let builder =
                ark_vrf::ring::RingVerifierKeyBuilder::<S>::deserialize_uncompressed_unchecked(
                    &builder_raw.0[..],
                )
                .unwrap();
            let verifier_key = builder.finalize();
            let mut verifier_key_raw = RingVerifierKeyRaw([0u8; RING_VERIFIER_KEY_SERIALIZED_SIZE]);
            verifier_key
                .serialize_compressed(&mut verifier_key_raw.0[..])
                .unwrap();
            RingVerifierKey::<T>::set(Some(verifier_key_raw));
        }

        pub(crate) fn push_members_impl<S: RingSuite>(new_members: Vec<PublicKeyRaw>) {
            let mut builder_raw = RingBuilder::<T>::get().unwrap();
            let mut builder =
                ark_bandersnatch::RingVerifierKeyBuilder::deserialize_uncompressed_unchecked(
                    &builder_raw.0[..],
                )
                .unwrap();
            let new_members = new_members
                .into_iter()
                .map(|m| {
                    log::trace!("Pushing {:02x?}", m.0);
                    ark_bandersnatch::AffinePoint::deserialize_compressed_unchecked(&m.0[..])
                        .unwrap()
                })
                .collect::<Vec<_>>();
            builder.append(&new_members, Self::fetch_srs_range).unwrap();
            builder
                .serialize_uncompressed(&mut builder_raw.0[..])
                .unwrap();
            RingBuilder::<T>::set(Some(builder_raw));
        }

        pub(crate) fn ring_reset_impl() {
            let mut builder_raw = [0_u8; RING_BUILDER_SERIALIZED_SIZE];
            builder_raw.copy_from_slice(RING_BUILDER_DATA);
            log::debug!("Reset ring verifier key builder");
            RingBuilder::<T>::set(Some(RingBuilderRaw(builder_raw)));
        }

        // Given a range, returns the list of chunks that maps to the keys at those indices.
        pub(crate) fn fetch_srs_range(range: Range<usize>) -> Option<Vec<SrsItem>> {
            log::debug!("SRS lookup {range:?}");

            let start_page = range.start / SRS_PAGE_SIZE;
            let end_page = (range.end - 1) / SRS_PAGE_SIZE;

            Some(
                (start_page..=end_page)
                    .flat_map(|page_idx| {
                        log::trace!("  Reading page {page_idx}");
                        Srs::<T>::get(page_idx as u32).unwrap().0.into_iter()
                    })
                    .skip(range.start % SRS_PAGE_SIZE)
                    .take(range.end - range.start)
                    .map(|data| SrsItem::deserialize_compressed(&data.0[..]).unwrap())
                    .collect(),
            )
        }
    }
}
