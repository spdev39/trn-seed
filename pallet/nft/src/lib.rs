// Copyright 2022-2023 Futureverse Corporation Limited
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// You may obtain a copy of the License at the root of this project source code

#![cfg_attr(not(feature = "std"), no_std)]
//! # NFT Module
//!
//! Provides the basic creation and management of dynamic NFTs (created at runtime).
//!
//! Intended to be used "as is" by dapps and provide basic NFT feature set for smart contracts
//! to extend.
//!
//! *Collection*:
//! Collection are a grouping of tokens- equivalent to an ERC721 contract
//!
//! *Tokens*:
//!  Individual tokens within a collection. Globally identifiable by a tuple of (collection, serial
//! number)

use frame_support::{
	ensure,
	traits::{fungibles::Transfer, Get},
	transactional, PalletId,
};
use seed_pallet_common::{OnNewAssetSubscriber, OnTransferSubscriber, Xls20MintRequest};
use seed_primitives::{
	AssetId, Balance, CollectionUuid, MetadataScheme, OriginChain, ParachainId, RoyaltiesSchedule,
	SerialNumber, TokenCount, TokenId, TokenLockReason, MAX_COLLECTION_ENTITLEMENTS,
};
use sp_runtime::{
	traits::{AccountIdConversion, One, Zero},
	DispatchResult,
};
use sp_std::prelude::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod tests;
pub mod weights;

pub use weights::WeightInfo;

mod impls;
pub mod traits;
mod types;

pub use impls::*;
pub use pallet::*;
pub use types::*;

/// The maximum amount of owned tokens to be returned by the RPC
pub const MAX_OWNED_TOKENS_LIMIT: u16 = 1000;
/// The logging target for this module
pub(crate) const LOG_TARGET: &str = "nft";

#[frame_support::pallet]
pub mod pallet {
	use super::{DispatchResult, *};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use seed_pallet_common::utils::PublicMintInformation;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(5);

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		_phantom: sp_std::marker::PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig { _phantom: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			NextCollectionId::<T>::put(1_u32);
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The system event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Max tokens that a collection can contain
		type MaxTokensPerCollection: Get<u32>;
		/// Max quantity of NFTs that can be minted in one transaction
		type MintLimit: Get<u32>;
		/// Handler for when an NFT has been transferred
		type OnTransferSubscription: OnTransferSubscriber;
		/// Handler for when an NFT collection has been created
		type OnNewAssetSubscription: OnNewAssetSubscriber<CollectionUuid>;
		/// Handles a multi-currency fungible asset system
		type MultiCurrency: Transfer<Self::AccountId, Balance = Balance, AssetId = AssetId>;
		/// This pallet's Id, used for deriving a sovereign account ID
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		/// The parachain_id being used by this parachain
		type ParachainId: Get<ParachainId>;
		/// The maximum length of a collection name, stored on-chain
		#[pallet::constant]
		type StringLimit: Get<u32>;
		/// Provides the public call to weight mapping
		type WeightInfo: WeightInfo;
		/// Interface for sending XLS20 mint requests
		type Xls20MintRequest: Xls20MintRequest<AccountId = Self::AccountId>;
	}

	/// Map from collection to its information
	#[pallet::storage]
	pub type CollectionInfo<T: Config> = StorageMap<
		_,
		Twox64Concat,
		CollectionUuid,
		CollectionInformation<T::AccountId, T::MaxTokensPerCollection, T::StringLimit>,
	>;

	/// Map from collection to its public minting information
	#[pallet::storage]
	pub type PublicMintInfo<T: Config> =
		StorageMap<_, Twox64Concat, CollectionUuid, PublicMintInformation>;

	/// The next available incrementing collection id
	#[pallet::storage]
	pub type NextCollectionId<T> = StorageValue<_, u32, ValueQuery>;

	/// Map from a token to lock status if any
	#[pallet::storage]
	pub type TokenLocks<T> = StorageMap<_, Twox64Concat, TokenId, TokenLockReason>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new collection of tokens was created
		CollectionCreate {
			collection_uuid: CollectionUuid,
			initial_issuance: TokenCount,
			max_issuance: Option<TokenCount>,
			collection_owner: T::AccountId,
			metadata_scheme: MetadataScheme,
			name: Vec<u8>,
			royalties_schedule: Option<RoyaltiesSchedule<T::AccountId>>,
			origin_chain: OriginChain,
			compatibility: CrossChainCompatibility,
		},
		/// Public minting was enabled/disabled for a collection
		PublicMintToggle { collection_id: CollectionUuid, enabled: bool },
		/// Token(s) were minted
		Mint {
			collection_id: CollectionUuid,
			start: SerialNumber,
			end: SerialNumber,
			owner: T::AccountId,
		},
		/// Payment was made to cover a public mint
		MintFeePaid {
			who: T::AccountId,
			collection_id: CollectionUuid,
			payment_asset: AssetId,
			payment_amount: Balance,
			token_count: TokenCount,
		},
		/// A mint price was set for a collection
		MintPriceSet {
			collection_id: CollectionUuid,
			payment_asset: Option<AssetId>,
			mint_price: Option<Balance>,
		},
		/// Token(s) were bridged
		BridgedMint {
			collection_id: CollectionUuid,
			serial_numbers: BoundedVec<SerialNumber, T::MaxTokensPerCollection>,
			owner: T::AccountId,
		},
		/// A new owner was set
		OwnerSet { collection_id: CollectionUuid, new_owner: T::AccountId },
		/// Max issuance was set
		MaxIssuanceSet { collection_id: CollectionUuid, max_issuance: TokenCount },
		/// Base URI was set
		BaseUriSet { collection_id: CollectionUuid, base_uri: Vec<u8> },
		/// Name was set
		NameSet { collection_id: CollectionUuid, name: BoundedVec<u8, T::StringLimit> },
		/// Royalties schedule was set
		RoyaltiesScheduleSet {
			collection_id: CollectionUuid,
			royalties_schedule: RoyaltiesSchedule<T::AccountId>,
		},
		/// A token was transferred
		Transfer {
			previous_owner: T::AccountId,
			collection_id: CollectionUuid,
			serial_numbers: Vec<SerialNumber>,
			new_owner: T::AccountId,
		},
		/// A token was burned
		Burn { collection_id: CollectionUuid, serial_number: SerialNumber },
		/// Collection has been claimed
		CollectionClaimed { account: T::AccountId, collection_id: CollectionUuid },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Given collection name is invalid (invalid utf-8, too long, empty)
		CollectionNameInvalid,
		/// No more Ids are available, they've been exhausted
		NoAvailableIds,
		/// Origin does not own the NFT
		NotTokenOwner,
		/// The token does not exist
		NoToken,
		/// Origin is not the collection owner and is not permitted to perform the operation
		NotCollectionOwner,
		/// This collection has not allowed public minting
		PublicMintDisabled,
		/// Cannot operate on a listed NFT
		TokenLocked,
		/// Total royalties would exceed 100% of sale or an empty vec is supplied
		RoyaltiesInvalid,
		/// The collection does not exist
		NoCollectionFound,
		/// The metadata path is invalid (non-utf8 or empty)
		InvalidMetadataPath,
		/// The caller can not be the new owner
		InvalidNewOwner,
		/// The number of tokens have exceeded the max tokens allowed
		TokenLimitExceeded,
		/// The quantity exceeds the max tokens per mint limit
		MintLimitExceeded,
		/// Max issuance needs to be greater than 0 and initial_issuance
		/// Cannot exceed MaxTokensPerCollection
		InvalidMaxIssuance,
		/// The max issuance has already been set and can't be changed
		MaxIssuanceAlreadySet,
		/// The collection max issuance has been reached and no more tokens can be minted
		MaxIssuanceReached,
		/// Attemped to mint a token that was bridged from a different chain
		AttemptedMintOnBridgedToken,
		/// Cannot claim already claimed collections
		CannotClaimNonClaimableCollections,
		/// Initial issuance on XLS-20 compatible collections must be zero
		InitialIssuanceNotZero,
		/// Total issuance of collection must be zero to add xls20 compatibility
		CollectionIssuanceNotZero,
		/// Token(s) blocked from minting during the bridging process
		BlockedMint,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::claim_unowned_collection())]
		/// Bridged collections from Ethereum will initially lack an owner. These collections will
		/// be assigned to the pallet. This allows for claiming those collections assuming they were
		/// assigned to the pallet
		pub fn claim_unowned_collection(
			origin: OriginFor<T>,
			collection_id: CollectionUuid,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let _who = ensure_root(origin)?;

			CollectionInfo::<T>::try_mutate(collection_id, |maybe_collection| -> DispatchResult {
				let collection = maybe_collection.as_mut().ok_or(Error::<T>::NoCollectionFound)?;
				ensure!(
					collection.owner == Self::account_id(),
					Error::<T>::CannotClaimNonClaimableCollections
				);

				collection.owner = new_owner.clone();
				Ok(())
			})?;
			let event = Event::<T>::CollectionClaimed { account: new_owner, collection_id };
			Self::deposit_event(event);

			Ok(())
		}

		/// Set the owner of a collection
		/// Caller must be the current collection owner
		#[pallet::weight(T::WeightInfo::set_owner())]
		pub fn set_owner(
			origin: OriginFor<T>,
			collection_id: CollectionUuid,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut collection_info =
				<CollectionInfo<T>>::get(collection_id).ok_or(Error::<T>::NoCollectionFound)?;
			ensure!(collection_info.is_collection_owner(&who), Error::<T>::NotCollectionOwner);
			collection_info.owner = new_owner.clone();
			<CollectionInfo<T>>::insert(collection_id, collection_info);
			Self::deposit_event(Event::<T>::OwnerSet { collection_id, new_owner });
			Ok(())
		}

		/// Set the max issuance of a collection
		/// Caller must be the current collection owner
		#[pallet::weight(T::WeightInfo::set_max_issuance())]
		pub fn set_max_issuance(
			origin: OriginFor<T>,
			collection_id: CollectionUuid,
			max_issuance: TokenCount,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut collection_info =
				<CollectionInfo<T>>::get(collection_id).ok_or(Error::<T>::NoCollectionFound)?;
			ensure!(!max_issuance.is_zero(), Error::<T>::InvalidMaxIssuance);
			ensure!(collection_info.is_collection_owner(&who), Error::<T>::NotCollectionOwner);
			ensure!(collection_info.max_issuance.is_none(), Error::<T>::MaxIssuanceAlreadySet);
			ensure!(
				collection_info.collection_issuance <= max_issuance,
				Error::<T>::InvalidMaxIssuance
			);

			collection_info.max_issuance = Some(max_issuance);
			<CollectionInfo<T>>::insert(collection_id, collection_info);
			Self::deposit_event(Event::<T>::MaxIssuanceSet { collection_id, max_issuance });
			Ok(())
		}

		/// Set the base URI of a collection
		/// Caller must be the current collection owner
		#[pallet::weight(T::WeightInfo::set_base_uri())]
		pub fn set_base_uri(
			origin: OriginFor<T>,
			collection_id: CollectionUuid,
			base_uri: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut collection_info =
				<CollectionInfo<T>>::get(collection_id).ok_or(Error::<T>::NoCollectionFound)?;
			ensure!(collection_info.is_collection_owner(&who), Error::<T>::NotCollectionOwner);

			collection_info.metadata_scheme = base_uri
				.clone()
				.as_slice()
				.try_into()
				.map_err(|_| Error::<T>::InvalidMetadataPath)?;

			<CollectionInfo<T>>::insert(collection_id, collection_info);
			Self::deposit_event(Event::<T>::BaseUriSet { collection_id, base_uri });
			Ok(())
		}

		/// Create a new collection
		/// Additional tokens can be minted via `mint_additional`
		///
		/// `name` - the name of the collection
		/// `initial_issuance` - number of tokens to mint now
		/// `max_issuance` - maximum number of tokens allowed in collection
		/// `token_owner` - the token owner, defaults to the caller
		/// `metadata_scheme` - The off-chain metadata referencing scheme for tokens in this
		/// `royalties_schedule` - defacto royalties plan for secondary sales, this will
		/// apply to all tokens in the collection by default.
		#[pallet::weight(T::WeightInfo::create_collection())]
		#[transactional]
		pub fn create_collection(
			origin: OriginFor<T>,
			name: BoundedVec<u8, T::StringLimit>,
			initial_issuance: TokenCount,
			max_issuance: Option<TokenCount>,
			token_owner: Option<T::AccountId>,
			metadata_scheme: MetadataScheme,
			royalties_schedule: Option<RoyaltiesSchedule<T::AccountId>>,
			cross_chain_compatibility: CrossChainCompatibility,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_create_collection(
				who,
				name,
				initial_issuance,
				max_issuance,
				token_owner,
				metadata_scheme,
				royalties_schedule,
				OriginChain::Root,
				cross_chain_compatibility,
			)?;
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::toggle_public_mint())]
		pub fn toggle_public_mint(
			origin: OriginFor<T>,
			collection_id: CollectionUuid,
			enabled: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection_info =
				<CollectionInfo<T>>::get(collection_id).ok_or(Error::<T>::NoCollectionFound)?;
			// Only the owner can make this call
			ensure!(collection_info.is_collection_owner(&who), Error::<T>::NotCollectionOwner);

			// Get public mint info and set enabled flag
			let mut public_mint_info = <PublicMintInfo<T>>::get(collection_id).unwrap_or_default();
			public_mint_info.enabled = enabled;

			if public_mint_info == PublicMintInformation::default() {
				// If the pricing details are None, and enabled is false
				// Remove the storage entry
				<PublicMintInfo<T>>::remove(collection_id);
			} else {
				// Otherwise, update the storage
				<PublicMintInfo<T>>::insert(collection_id, public_mint_info);
			}

			Self::deposit_event(Event::<T>::PublicMintToggle { collection_id, enabled });
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::set_mint_fee())]
		pub fn set_mint_fee(
			origin: OriginFor<T>,
			collection_id: CollectionUuid,
			pricing_details: Option<(AssetId, Balance)>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection_info =
				<CollectionInfo<T>>::get(collection_id).ok_or(Error::<T>::NoCollectionFound)?;
			// Only the owner can make this call
			ensure!(collection_info.is_collection_owner(&who), Error::<T>::NotCollectionOwner);

			// Get the existing public mint info if it exists
			let mut public_mint_info = <PublicMintInfo<T>>::get(collection_id).unwrap_or_default();
			public_mint_info.pricing_details = pricing_details;

			if public_mint_info == PublicMintInformation::default() {
				// If the pricing details are None, and enabled is false
				// Remove the storage entry
				<PublicMintInfo<T>>::remove(collection_id);
			} else {
				// Otherwise, update the storage
				<PublicMintInfo<T>>::insert(collection_id, public_mint_info);
			}

			// Extract payment asset and mint price for clearer event logging
			let (payment_asset, mint_price) = match pricing_details {
				Some((asset, price)) => (Some(asset), Some(price)),
				None => (None, None),
			};

			Self::deposit_event(Event::<T>::MintPriceSet {
				collection_id,
				payment_asset,
				mint_price,
			});
			Ok(())
		}

		/// Mint tokens for an existing collection
		///
		/// `collection_id` - the collection to mint tokens in
		/// `quantity` - how many tokens to mint
		/// `token_owner` - the token owner, defaults to the caller if unspecified
		/// Caller must be the collection owner
		/// -----------
		/// Weight is O(N) where N is `quantity`
		#[pallet::weight(T::WeightInfo::mint())]
		#[transactional]
		pub fn mint(
			origin: OriginFor<T>,
			collection_id: CollectionUuid,
			quantity: TokenCount,
			token_owner: Option<T::AccountId>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(quantity <= T::MintLimit::get(), Error::<T>::MintLimitExceeded);

			let mut collection_info =
				<CollectionInfo<T>>::get(collection_id).ok_or(Error::<T>::NoCollectionFound)?;
			let public_mint_info = <PublicMintInfo<T>>::get(collection_id).unwrap_or_default();

			// Perform pre mint checks
			let serial_numbers =
				Self::pre_mint(&who, quantity, &collection_info, public_mint_info.enabled)?;
			let owner = token_owner.unwrap_or(who.clone());
			let xls20_compatible = collection_info.cross_chain_compatibility.xrpl;
			let metadata_scheme = collection_info.metadata_scheme.clone();

			// Increment next serial number
			let next_serial_number = collection_info.next_serial_number;
			collection_info.next_serial_number =
				next_serial_number.checked_add(quantity).ok_or(Error::<T>::NoAvailableIds)?;

			// Only charge mint fee if public mint enabled and caller is not collection owner
			if public_mint_info.enabled && !collection_info.is_collection_owner(&who) {
				// Charge the mint fee for the mint
				Self::charge_mint_fee(
					&who,
					collection_id,
					&collection_info.owner,
					public_mint_info,
					quantity,
				)?;
			}

			// Perform the mint and update storage
			Self::do_mint(collection_id, collection_info, &owner, &serial_numbers)?;

			// Check if this collection is XLS-20 compatible
			if xls20_compatible {
				// Pay XLS20 mint fee and send requests
				let _ = T::Xls20MintRequest::request_xls20_mint(
					&who,
					collection_id,
					serial_numbers.clone().into_inner(),
					metadata_scheme,
				)?;
			}

			// throw event, listing starting and endpoint token ids (sequential mint)
			Self::deposit_event(Event::<T>::Mint {
				collection_id,
				start: *serial_numbers.first().ok_or(Error::<T>::NoToken)?,
				end: *serial_numbers.last().ok_or(Error::<T>::NoToken)?,
				owner,
			});
			Ok(())
		}

		/// Transfer ownership of an NFT
		/// Caller must be the token owner
		#[pallet::weight(T::WeightInfo::transfer())]
		#[transactional]
		pub fn transfer(
			origin: OriginFor<T>,
			collection_id: CollectionUuid,
			serial_numbers: BoundedVec<SerialNumber, T::MaxTokensPerCollection>,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_transfer(collection_id, serial_numbers, &who, &new_owner)
		}

		/// Burn a token 🔥
		///
		/// Caller must be the token owner
		#[pallet::weight(T::WeightInfo::burn())]
		#[transactional]
		pub fn burn(origin: OriginFor<T>, token_id: TokenId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let (collection_id, serial_number) = token_id;

			Self::do_burn(&who, collection_id, serial_number)?;
			Self::deposit_event(Event::<T>::Burn { collection_id, serial_number });
			Ok(())
		}

		/// Set the name of a collection
		/// Caller must be the current collection owner
		#[pallet::weight(T::WeightInfo::set_name())]
		pub fn set_name(
			origin: OriginFor<T>,
			collection_id: CollectionUuid,
			name: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut collection_info =
				<CollectionInfo<T>>::get(collection_id).ok_or(Error::<T>::NoCollectionFound)?;
			ensure!(collection_info.is_collection_owner(&who), Error::<T>::NotCollectionOwner);

			ensure!(!name.is_empty(), Error::<T>::CollectionNameInvalid);
			ensure!(core::str::from_utf8(&name).is_ok(), Error::<T>::CollectionNameInvalid);
			collection_info.name = name.clone();

			<CollectionInfo<T>>::insert(collection_id, collection_info);
			Self::deposit_event(Event::<T>::NameSet { collection_id, name });
			Ok(())
		}

		/// Set the royalties schedule of a collection
		/// Caller must be the current collection owner
		#[pallet::weight(T::WeightInfo::set_royalties_schedule())]
		pub fn set_royalties_schedule(
			origin: OriginFor<T>,
			collection_id: CollectionUuid,
			royalties_schedule: RoyaltiesSchedule<T::AccountId>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut collection_info =
				<CollectionInfo<T>>::get(collection_id).ok_or(Error::<T>::NoCollectionFound)?;
			ensure!(collection_info.is_collection_owner(&who), Error::<T>::NotCollectionOwner);

			// Check that the entitlements are less than MAX_ENTITLEMENTS - 2
			// This is because when the token is listed, two more entitlements will be added
			// for the network fee and marketplace fee
			ensure!(
				royalties_schedule.entitlements.len() <= MAX_COLLECTION_ENTITLEMENTS as usize,
				Error::<T>::RoyaltiesInvalid
			);
			ensure!(royalties_schedule.validate(), Error::<T>::RoyaltiesInvalid);

			collection_info.royalties_schedule = Some(royalties_schedule.clone());

			<CollectionInfo<T>>::insert(collection_id, collection_info);
			Self::deposit_event(Event::<T>::RoyaltiesScheduleSet {
				collection_id,
				royalties_schedule,
			});
			Ok(())
		}
	}
}

impl<T: Config> From<TokenOwnershipError> for Error<T> {
	fn from(val: TokenOwnershipError) -> Error<T> {
		match val {
			TokenOwnershipError::TokenLimitExceeded => Error::<T>::TokenLimitExceeded,
		}
	}
}
