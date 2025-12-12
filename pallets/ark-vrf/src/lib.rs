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

use ark_vrf::ring::RingSuite as RingSuiteT;

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

const PUBLIC_KEY_SERIALIZED_SIZE: usize = 32;

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
pub struct PublicKeyRaw(pub [u8; PUBLIC_KEY_SERIALIZED_SIZE]);

const SRS_ITEM_SERIALIZED_SIZE: usize = 48;

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

const RING_BUILDER_SERIALIZED_SIZE: usize = 848;

#[derive(MaxEncodedLen, Encode, Decode, TypeInfo)]
pub struct RingBuilderRaw(pub [u8; RING_BUILDER_SERIALIZED_SIZE]);

#[frame_support::pallet]
pub mod pallet {
    use core::ops::Range;

    use ark_vrf::reexports::ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
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
    pub type RingBuilder<T> = StorageValue<_, RingBuilderRaw>;

    #[pallet::storage]
    pub type RingKeys<T: Config> = StorageValue<_, BoundedVec<PublicKeyRaw, T::MaxRingSize>>;

    #[pallet::storage]
    pub type Srs<T: Config> = StorageMap<_, Twox64Concat, u32, SrsPage>;

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
            type SrsItem = ark_vrf::ring::G1Affine<ark_bandersnatch::BandersnatchSha512Ell2>;
            pub const RING_BUILDER_PARAMS: &[u8] =
                include_bytes!("static/ring-builder-params-full.bin");
            let srs =
                <Vec<SrsItem>>::deserialize_uncompressed_unchecked(RING_BUILDER_PARAMS).unwrap();
            assert_eq!(srs.len(), 1 << 14);
            let mut srs_page = SrsPage::default();
            for (i, item) in srs.iter().enumerate() {
                let page_off = i % SRS_PAGE_SIZE;
                let raw = &mut srs_page.0[page_off];
                item.serialize_compressed(&mut raw.0[..]).unwrap();
                if page_off == SRS_PAGE_SIZE - 1 {
                    let page_index = i / SRS_PAGE_SIZE;
                    Srs::<T>::insert(page_index as u32, srs_page.clone());
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

            // log::debug!("Commit ring with {} members", members.len());
            // TODO: intermediate function returning the builder

            if optimized {
                Self::commit_impl::<SubSuite>();
            } else {
                Self::commit_impl::<ArkSuite>();
            }

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub(crate) fn commit_impl<S: RingSuiteT>() {
            let builder_raw = RingBuilder::<T>::get().unwrap();
            let builder =
                ark_vrf::ring::RingVerifierKeyBuilder::<S>::deserialize_uncompressed_unchecked(
                    &builder_raw.0[..],
                )
                .unwrap();
            let _verifier_key = builder.finalize();
        }

        pub(crate) fn push_members_impl<S: RingSuiteT>(new_members: Vec<PublicKeyRaw>) {
            let mut builder_raw = RingBuilder::<T>::get().unwrap();
            let lookup = |range: Range<usize>| Self::fetch_srs_chunks(range).ok();
            let mut builder =
                ark_bandersnatch::RingVerifierKeyBuilder::deserialize_uncompressed_unchecked(
                    &builder_raw.0[..],
                )
                .unwrap();
            let new_members = new_members
                .into_iter()
                .map(|m| {
                    ark_bandersnatch::AffinePoint::deserialize_compressed_unchecked(&m.0[..])
                        .expect("TODO")
                })
                .collect::<Vec<_>>();
            builder.append(&new_members, lookup).unwrap();
            builder
                .serialize_uncompressed(&mut builder_raw.0[..])
                .unwrap();
            RingBuilder::<T>::set(Some(builder_raw));
        }

        pub(crate) fn ring_reset_impl() {
            const RING_BUILDER_DATA: &[u8] = include_bytes!("static/ring-builder-full.bin");
            let mut builder_raw = [0_u8; RING_BUILDER_SERIALIZED_SIZE];
            builder_raw.copy_from_slice(RING_BUILDER_DATA);
            RingBuilder::<T>::set(Some(RingBuilderRaw(builder_raw)));
        }

        // Given a range, returns the list of chunks that maps to the keys at those indices.
        pub(crate) fn fetch_srs_chunks(range: Range<usize>) -> Result<Vec<SrsItem>, ()> {
            let expected_len = range.end.saturating_sub(range.start);
            let mut page_idx = range.start.checked_div(SRS_PAGE_SIZE).ok_or(())?;

            let mut chunks = Srs::<T>::get(page_idx as u32)
                .ok_or(())?
                .0
                .into_iter()
                .skip(range.start % SRS_PAGE_SIZE)
                .take(expected_len)
                .map(|data| SrsItem::deserialize_compressed(&data.0[..]).unwrap())
                .collect::<Vec<_>>();

            while chunks.len() < expected_len {
                page_idx = page_idx.checked_add(1).ok_or(())?;
                let page = Srs::<T>::get(page_idx as u32).ok_or(())?;
                chunks.extend(
                    page.0
                        .into_iter()
                        .map(|data| SrsItem::deserialize_compressed(&data.0[..]).unwrap())
                        .take(expected_len.saturating_sub(chunks.len())),
                );
            }

            Ok(chunks)
        }
    }
}
