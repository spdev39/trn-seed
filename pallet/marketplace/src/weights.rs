//! Autogenerated weights for pallet_marketplace
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Justins-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ../rust_builds/release/seed
// benchmark
// pallet
// --chain
// dev
// --steps
// 50
// --repeat
// 20
// --pallet
// pallet_nft
// --extrinsic=*
// --execution
// wasm
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --output
// ./output
// --template
// ./scripts/pallet_template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_marketplace.
pub trait WeightInfo {
    fn register_marketplace() -> Weight;
    fn sell() -> Weight;
    fn buy() -> Weight;
    fn auction() -> Weight;
    fn bid() -> Weight;
    fn cancel_sale() -> Weight;
    fn update_fixed_price() -> Weight;
    fn make_simple_offer() -> Weight;
    fn cancel_offer() -> Weight;
    fn accept_offer() -> Weight;
    fn set_fee_to() -> Weight;
}

/// Weights for pallet_assets_ext using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: Marketplace NextMarketplaceId (r:1 w:1)
    // Storage: Marketplace RegisteredMarketplaces (r:0 w:1)
    fn register_marketplace() -> Weight {
        Weight::from_ref_time(48_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Nft CollectionInfo (r:1 w:0)
    // Storage: Marketplace NextListingId (r:1 w:1)
    // Storage: Nft TokenLocks (r:1 w:1)
    // Storage: Marketplace Listings (r:0 w:1)
    // Storage: Marketplace ListingEndSchedule (r:0 w:1)
    // Storage: Marketplace OpenCollectionListings (r:0 w:1)
    fn sell() -> Weight {
        Weight::from_ref_time(85_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    // Storage: Marketplace Listings (r:1 w:1)
    // Storage: Marketplace FeeTo (r:1 w:0)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: Nft CollectionInfo (r:1 w:1)
    // Storage: TokenApprovals ERC721Approvals (r:0 w:1)
    // Storage: Nft TokenLocks (r:0 w:1)
    // Storage: Marketplace ListingEndSchedule (r:0 w:1)
    // Storage: Marketplace OpenCollectionListings (r:0 w:1)
    fn buy() -> Weight {
        Weight::from_ref_time(148_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(9 as u64))
    }
    // Storage: Nft CollectionInfo (r:1 w:0)
    // Storage: Marketplace NextListingId (r:1 w:1)
    // Storage: Nft TokenLocks (r:1 w:1)
    // Storage: Marketplace Listings (r:0 w:1)
    // Storage: Marketplace ListingEndSchedule (r:0 w:1)
    // Storage: Marketplace OpenCollectionListings (r:0 w:1)
    fn auction() -> Weight {
        Weight::from_ref_time(93_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    // Storage: Marketplace Listings (r:1 w:1)
    // Storage: Marketplace ListingWinningBid (r:1 w:1)
    // Storage: AssetsExt Holds (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: System Account (r:2 w:2)
    // Storage: Marketplace ListingEndSchedule (r:0 w:2)
    fn bid() -> Weight {
        Weight::from_ref_time(183_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(8 as u64))
            .saturating_add(T::DbWeight::get().writes(10 as u64))
    }
    // Storage: Marketplace Listings (r:1 w:1)
    // Storage: Nft TokenLocks (r:0 w:1)
    // Storage: Marketplace ListingEndSchedule (r:0 w:1)
    // Storage: Marketplace OpenCollectionListings (r:0 w:1)
    fn cancel_sale() -> Weight {
        Weight::from_ref_time(57_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    // Storage: Marketplace Listings (r:1 w:1)
    fn update_fixed_price() -> Weight {
        Weight::from_ref_time(48_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Nft CollectionInfo (r:1 w:0)
    // Storage: Marketplace NextOfferId (r:1 w:1)
    // Storage: Nft TokenLocks (r:1 w:0)
    // Storage: AssetsExt Holds (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: System Account (r:1 w:1)
    // Storage: Marketplace TokenOffers (r:1 w:1)
    // Storage: Marketplace Offers (r:0 w:1)
    fn make_simple_offer() -> Weight {
        Weight::from_ref_time(172_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(9 as u64))
            .saturating_add(T::DbWeight::get().writes(8 as u64))
    }
    // Storage: Marketplace Offers (r:1 w:1)
    // Storage: AssetsExt Holds (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: System Account (r:1 w:1)
    // Storage: Marketplace TokenOffers (r:1 w:1)
    fn cancel_offer() -> Weight {
        Weight::from_ref_time(132_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().writes(7 as u64))
    }
    // Storage: Marketplace Offers (r:1 w:1)
    // Storage: Nft TokenLocks (r:1 w:0)
    // Storage: Nft CollectionInfo (r:1 w:1)
    // Storage: AssetsExt Holds (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: System Account (r:1 w:1)
    // Storage: Marketplace TokenOffers (r:1 w:1)
    // Storage: TokenApprovals ERC721Approvals (r:0 w:1)
    fn accept_offer() -> Weight {
        Weight::from_ref_time(185_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(9 as u64))
            .saturating_add(T::DbWeight::get().writes(9 as u64))
    }
    // Storage: Marketplace FeeTo (r:0 w:1)
    fn set_fee_to() -> Weight {
        Weight::from_ref_time(32_000_000 as u64)
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: Marketplace NextMarketplaceId (r:1 w:1)
    // Storage: Marketplace RegisteredMarketplaces (r:0 w:1)
    fn register_marketplace() -> Weight {
        Weight::from_ref_time(48_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: Nft CollectionInfo (r:1 w:0)
    // Storage: Marketplace NextListingId (r:1 w:1)
    // Storage: Nft TokenLocks (r:1 w:1)
    // Storage: Marketplace Listings (r:0 w:1)
    // Storage: Marketplace ListingEndSchedule (r:0 w:1)
    // Storage: Marketplace OpenCollectionListings (r:0 w:1)
    fn sell() -> Weight {
        Weight::from_ref_time(85_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(3 as u64))
            .saturating_add(RocksDbWeight::get().writes(5 as u64))
    }
    // Storage: Marketplace Listings (r:1 w:1)
    // Storage: Marketplace FeeTo (r:1 w:0)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: Nft CollectionInfo (r:1 w:1)
    // Storage: TokenApprovals ERC721Approvals (r:0 w:1)
    // Storage: Nft TokenLocks (r:0 w:1)
    // Storage: Marketplace ListingEndSchedule (r:0 w:1)
    // Storage: Marketplace OpenCollectionListings (r:0 w:1)
    fn buy() -> Weight {
        Weight::from_ref_time(148_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(6 as u64))
            .saturating_add(RocksDbWeight::get().writes(9 as u64))
    }
    // Storage: Nft CollectionInfo (r:1 w:0)
    // Storage: Marketplace NextListingId (r:1 w:1)
    // Storage: Nft TokenLocks (r:1 w:1)
    // Storage: Marketplace Listings (r:0 w:1)
    // Storage: Marketplace ListingEndSchedule (r:0 w:1)
    // Storage: Marketplace OpenCollectionListings (r:0 w:1)
    fn auction() -> Weight {
        Weight::from_ref_time(93_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(3 as u64))
            .saturating_add(RocksDbWeight::get().writes(5 as u64))
    }
    // Storage: Marketplace Listings (r:1 w:1)
    // Storage: Marketplace ListingWinningBid (r:1 w:1)
    // Storage: AssetsExt Holds (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: System Account (r:2 w:2)
    // Storage: Marketplace ListingEndSchedule (r:0 w:2)
    fn bid() -> Weight {
        Weight::from_ref_time(183_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(8 as u64))
            .saturating_add(RocksDbWeight::get().writes(10 as u64))
    }
    // Storage: Marketplace Listings (r:1 w:1)
    // Storage: Nft TokenLocks (r:0 w:1)
    // Storage: Marketplace ListingEndSchedule (r:0 w:1)
    // Storage: Marketplace OpenCollectionListings (r:0 w:1)
    fn cancel_sale() -> Weight {
        Weight::from_ref_time(57_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(4 as u64))
    }
    // Storage: Marketplace Listings (r:1 w:1)
    fn update_fixed_price() -> Weight {
        Weight::from_ref_time(48_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Nft CollectionInfo (r:1 w:0)
    // Storage: Marketplace NextOfferId (r:1 w:1)
    // Storage: Nft TokenLocks (r:1 w:0)
    // Storage: AssetsExt Holds (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: System Account (r:1 w:1)
    // Storage: Marketplace TokenOffers (r:1 w:1)
    // Storage: Marketplace Offers (r:0 w:1)
    fn make_simple_offer() -> Weight {
        Weight::from_ref_time(172_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(9 as u64))
            .saturating_add(RocksDbWeight::get().writes(8 as u64))
    }
    // Storage: Marketplace Offers (r:1 w:1)
    // Storage: AssetsExt Holds (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: System Account (r:1 w:1)
    // Storage: Marketplace TokenOffers (r:1 w:1)
    fn cancel_offer() -> Weight {
        Weight::from_ref_time(132_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(7 as u64))
            .saturating_add(RocksDbWeight::get().writes(7 as u64))
    }
    // Storage: Marketplace Offers (r:1 w:1)
    // Storage: Nft TokenLocks (r:1 w:0)
    // Storage: Nft CollectionInfo (r:1 w:1)
    // Storage: AssetsExt Holds (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: System Account (r:1 w:1)
    // Storage: Marketplace TokenOffers (r:1 w:1)
    // Storage: TokenApprovals ERC721Approvals (r:0 w:1)
    fn accept_offer() -> Weight {
        Weight::from_ref_time(185_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(9 as u64))
            .saturating_add(RocksDbWeight::get().writes(9 as u64))
    }
    // Storage: Marketplace FeeTo (r:0 w:1)
    fn set_fee_to() -> Weight {
        Weight::from_ref_time(32_000_000 as u64)
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
}