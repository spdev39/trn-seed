// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_sft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-102-147`, CPU: `Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/seed
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_sft
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// ./pallet/sft/src/weights.rs
// --template
// ./scripts/pallet_template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_sft.
pub trait WeightInfo {
	fn create_collection() -> Weight;
	fn toggle_public_mint() -> Weight;
	fn set_mint_fee() -> Weight;
	fn create_token() -> Weight;
	fn mint() -> Weight;
	fn transfer() -> Weight;
	fn burn() -> Weight;
	fn set_owner() -> Weight;
	fn set_max_issuance() -> Weight;
	fn set_base_uri() -> Weight;
	fn set_name() -> Weight;
	fn set_royalties_schedule() -> Weight;
}

/// Weights for pallet_sft using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Nft NextCollectionId (r:1 w:1)
	// Storage: EVM AccountCodes (r:1 w:1)
	// Storage: Futurepass DefaultProxy (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Sft SftCollectionInfo (r:0 w:1)
	fn create_collection() -> Weight {
		Weight::from_ref_time(96_589_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:1)
	// Storage: Sft TokenInfo (r:0 w:1)
	fn create_token() -> Weight {
		Weight::from_ref_time(66_702_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:0)
	// Storage: Sft TokenInfo (r:1 w:1)
	fn mint() -> Weight {
		Weight::from_ref_time(67_408_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Sft TokenInfo (r:1 w:1)
	fn transfer() -> Weight {
		Weight::from_ref_time(62_718_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Sft TokenInfo (r:1 w:1)
	fn burn() -> Weight {
		Weight::from_ref_time(66_107_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:1)
	fn set_owner() -> Weight {
		Weight::from_ref_time(59_368_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:0)
	// Storage: Sft TokenInfo (r:1 w:1)
	fn set_max_issuance() -> Weight {
		Weight::from_ref_time(61_021_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:1)
	fn set_base_uri() -> Weight {
		Weight::from_ref_time(56_065_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:1)
	fn set_name() -> Weight {
		Weight::from_ref_time(55_452_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Sft CollectionInfo (r:1 w:1)
	fn set_royalties_schedule() -> Weight {
		Weight::from_ref_time(68_177_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Sft CollectionInfo (r:1 w:0)
	// Storage: Sft PublicMintInfo (r:1 w:1)
	fn toggle_public_mint() -> Weight {
		Weight::from_ref_time(30_057_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Sft CollectionInfo (r:1 w:0)
	// Storage: Sft PublicMintInfo (r:1 w:1)
	fn set_mint_fee() -> Weight {
		Weight::from_ref_time(30_177_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Nft NextCollectionId (r:1 w:1)
	// Storage: EVM AccountCodes (r:1 w:1)
	// Storage: Futurepass DefaultProxy (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Sft SftCollectionInfo (r:0 w:1)
	fn create_collection() -> Weight {
		Weight::from_ref_time(96_589_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:1)
	// Storage: Sft TokenInfo (r:0 w:1)
	fn create_token() -> Weight {
		Weight::from_ref_time(66_702_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:0)
	// Storage: Sft TokenInfo (r:1 w:1)
	fn mint() -> Weight {
		Weight::from_ref_time(67_408_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Sft TokenInfo (r:1 w:1)
	fn transfer() -> Weight {
		Weight::from_ref_time(62_718_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Sft TokenInfo (r:1 w:1)
	fn burn() -> Weight {
		Weight::from_ref_time(66_107_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:1)
	fn set_owner() -> Weight {
		Weight::from_ref_time(59_368_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:0)
	// Storage: Sft TokenInfo (r:1 w:1)
	fn set_max_issuance() -> Weight {
		Weight::from_ref_time(61_021_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:1)
	fn set_base_uri() -> Weight {
		Weight::from_ref_time(56_065_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Sft SftCollectionInfo (r:1 w:1)
	fn set_name() -> Weight {
		Weight::from_ref_time(55_452_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Sft CollectionInfo (r:1 w:1)
	fn set_royalties_schedule() -> Weight {
		Weight::from_ref_time(68_177_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}

	// Storage: Sft CollectionInfo (r:1 w:0)
	// Storage: Sft PublicMintInfo (r:1 w:1)
	fn toggle_public_mint() -> Weight {
		Weight::from_ref_time(30_057_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Sft CollectionInfo (r:1 w:0)
	// Storage: Sft PublicMintInfo (r:1 w:1)
	fn set_mint_fee() -> Weight {
		Weight::from_ref_time(30_177_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}

