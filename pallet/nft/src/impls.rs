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

use crate::{traits::NFTExt, *};
use frame_support::{ensure, traits::Get, weights::Weight};
use frame_system::RawOrigin;
use precompile_utils::constants::ERC721_PRECOMPILE_ADDRESS_PREFIX;
use seed_pallet_common::{
	log,
	utils::{next_asset_uuid, PublicMintInformation},
	OnNewAssetSubscriber, OnTransferSubscriber,
};
use seed_primitives::{
	CollectionUuid, MetadataScheme, OriginChain, RoyaltiesSchedule, SerialNumber, TokenCount,
	TokenId, MAX_COLLECTION_ENTITLEMENTS,
};
use sp_runtime::{
	traits::Zero, ArithmeticError, BoundedVec, DispatchError, DispatchResult, SaturatedConversion,
};

impl<T: Config> Pallet<T> {
	/// Returns the CollectionUuid unique across parachains
	pub fn next_collection_uuid() -> Result<CollectionUuid, DispatchError> {
		let collection_id = <NextCollectionId<T>>::get();
		match next_asset_uuid(collection_id, T::ParachainId::get().into()) {
			Some(next_collection_uuid) => Ok(next_collection_uuid),
			None => Err(Error::<T>::NoAvailableIds.into()),
		}
	}

	/// Return whether the collection exists or not
	pub fn collection_exists(collection_id: CollectionUuid) -> bool {
		<CollectionInfo<T>>::contains_key(collection_id)
	}

	/// Returns number of tokens owned by an account in a collection
	/// Used by the ERC721 precompile for balance_of
	pub fn token_balance_of(who: &T::AccountId, collection_id: CollectionUuid) -> TokenCount {
		match <CollectionInfo<T>>::get(collection_id) {
			Some(collection_info) => {
				let serial_numbers: Vec<SerialNumber> = collection_info
					.owned_tokens
					.into_iter()
					.find(|token_ownership| &token_ownership.owner == who)
					.map(|token_ownership| token_ownership.owned_serials.clone().into_inner())
					.unwrap_or_default();
				serial_numbers.len() as TokenCount
			},
			None => TokenCount::zero(),
		}
	}

	/// Construct & return the full metadata URI for a given `token_id` (analogous to ERC721
	/// metadata token_uri)
	pub fn token_uri(token_id: TokenId) -> Vec<u8> {
		let collection_info = <CollectionInfo<T>>::get(token_id.0);
		if collection_info.is_none() {
			// should not happen
			log!(warn, "🃏 Unexpected empty metadata scheme: {:?}", token_id);
			return Default::default()
		}

		let collection_info = collection_info.unwrap();
		collection_info.metadata_scheme.construct_token_uri(token_id.1)
	}

	/// Transfer the given token from `current_owner` to `new_owner`
	/// Does no verification
	pub fn do_transfer(
		collection_id: CollectionUuid,
		serial_numbers: BoundedVec<SerialNumber, T::MaxTokensPerCollection>,
		current_owner: &T::AccountId,
		new_owner: &T::AccountId,
	) -> DispatchResult {
		ensure!(current_owner != new_owner, Error::<T>::InvalidNewOwner);

		CollectionInfo::<T>::try_mutate(collection_id, |maybe_collection_info| -> DispatchResult {
			let collection_info =
				maybe_collection_info.as_mut().ok_or(Error::<T>::NoCollectionFound)?;

			// Check ownership and locks
			for serial_number in serial_numbers.iter() {
				ensure!(
					collection_info.is_token_owner(current_owner, *serial_number),
					Error::<T>::NotTokenOwner
				);
				ensure!(
					!<TokenLocks<T>>::contains_key((collection_id, serial_number)),
					Error::<T>::TokenLocked
				);
			}

			collection_info.remove_user_tokens(current_owner, serial_numbers.clone());
			collection_info
				.add_user_tokens(new_owner, serial_numbers.clone())
				.map_err(|e| Error::<T>::from(e))?;

			for serial_number in serial_numbers.clone().iter() {
				T::OnTransferSubscription::on_nft_transfer(&(collection_id, *serial_number));
			}
			Self::deposit_event(Event::<T>::Transfer {
				previous_owner: current_owner.clone(),
				collection_id,
				serial_numbers: serial_numbers.into_inner(),
				new_owner: new_owner.clone(),
			});
			Ok(())
		})
	}

	/// Mint additional tokens in a collection
	/// This is called by the nft-peg pallet and mints tokens based on the token ids bridged
	/// An extra check is made to ensure tokens have not already been minted, if this happens
	/// execution won't fail, however those tokens will not be minted twice.
	pub fn mint_bridged_token(
		owner: &T::AccountId,
		collection_id: CollectionUuid,
		serial_numbers: Vec<SerialNumber>,
	) -> Result<Weight, (Weight, DispatchError)> {
		if serial_numbers.is_empty() {
			return Ok(Weight::zero())
		};

		let collection_info = match <CollectionInfo<T>>::get(collection_id) {
			Some(info) => info,
			None => return Ok(T::DbWeight::get().reads(1)),
		};

		// remove duplicates from serial_numbers
		let mut serial_numbers_trimmed = serial_numbers;
		serial_numbers_trimmed.sort_unstable();
		serial_numbers_trimmed.dedup();

		// Trim the new serial_numbers and remove any that have already been minted
		serial_numbers_trimmed = serial_numbers_trimmed
			.into_iter()
			.filter(|serial_number| {
				if collection_info.token_exists(*serial_number) {
					// Since we don't want to error, throw a warning instead.
					// If we error, then some tokens may be lost
					log!(
						warn,
						"🃏 Token Couldn't be minted as this token_id already exists: ({:?},{:?})",
						collection_id,
						serial_number
					);
					false
				} else {
					true
				}
			})
			.collect::<Vec<SerialNumber>>();

		let serial_numbers: Result<BoundedVec<SerialNumber, T::MaxTokensPerCollection>, ()> =
			BoundedVec::try_from(serial_numbers_trimmed);
		match serial_numbers {
			Ok(serial_numbers) => {
				let mint = Self::do_mint(collection_id, collection_info, owner, &serial_numbers);

				if mint.is_ok() {
					// throw event, listing all serial numbers minted from bridging
					// SerialNumbers will never exceed the limit denoted by
					// nft_peg::MaxTokensPerMint Which is set to 50 in the runtime, so this event is
					// safe to list all bridged serial_numbers
					Self::deposit_event(Event::<T>::BridgedMint {
						collection_id,
						serial_numbers,
						owner: owner.clone(),
					});

					Ok(T::DbWeight::get().reads_writes(1, 1))
				} else {
					Err((T::DbWeight::get().reads(1), (Error::<T>::BlockedMint).into()))
				}
			},
			_ => Ok(T::DbWeight::get().reads(1)),
		}
	}

	/// Perform validity checks on collection_info
	/// Return bounded vec of serial numbers to mint
	pub fn pre_mint(
		who: &T::AccountId,
		quantity: TokenCount,
		collection_info: &CollectionInformation<
			T::AccountId,
			T::MaxTokensPerCollection,
			T::StringLimit,
		>,
		public_mint_enabled: bool,
	) -> Result<BoundedVec<SerialNumber, T::MaxTokensPerCollection>, DispatchError> {
		// Quantity must be some
		ensure!(quantity > Zero::zero(), Error::<T>::NoToken);
		// Caller must be collection_owner if public mint is disabled
		ensure!(
			collection_info.is_collection_owner(&who) || public_mint_enabled,
			Error::<T>::PublicMintDisabled
		);
		// Check we don't exceed the token limit
		ensure!(
			collection_info.collection_issuance.saturating_add(quantity) <
				T::MaxTokensPerCollection::get(),
			Error::<T>::TokenLimitExceeded
		);
		// Cannot mint for a token that was bridged from Ethereum
		ensure!(
			collection_info.origin_chain == OriginChain::Root,
			Error::<T>::AttemptedMintOnBridgedToken
		);

		let previous_serial_number = collection_info.next_serial_number;
		let next_serial_number =
			previous_serial_number.checked_add(quantity).ok_or(Error::<T>::NoAvailableIds)?;

		// Check early that we won't exceed the BoundedVec limit
		ensure!(
			next_serial_number <= T::MaxTokensPerCollection::get(),
			Error::<T>::TokenLimitExceeded
		);

		// Can't mint more than specified max_issuance
		if let Some(max_issuance) = collection_info.max_issuance {
			ensure!(max_issuance >= next_serial_number, Error::<T>::MaxIssuanceReached);
		}

		let serial_numbers_unbounded: Vec<SerialNumber> =
			(previous_serial_number..next_serial_number).collect();
		let serial_numbers: BoundedVec<SerialNumber, T::MaxTokensPerCollection> =
			BoundedVec::try_from(serial_numbers_unbounded)
				.map_err(|_| Error::<T>::TokenLimitExceeded)?;
		Ok(serial_numbers)
	}

	pub(crate) fn charge_mint_fee(
		who: &T::AccountId,
		collection_id: CollectionUuid,
		collection_owner: &T::AccountId,
		public_mint_info: PublicMintInformation,
		token_count: TokenCount,
	) -> DispatchResult {
		// Calculate the total fee
		let total_fee = match public_mint_info.pricing_details {
			Some((asset, price)) => Some((asset, price.saturating_mul(token_count as Balance))),
			None => None,
		};
		// Charge the fee if there is a fee set
		if let Some((asset, total_fee)) = total_fee {
			T::MultiCurrency::transfer(asset, who, &collection_owner, total_fee, false)?;
			// Deposit event
			Self::deposit_event(Event::<T>::MintFeePaid {
				who: who.clone(),
				collection_id,
				payment_asset: asset,
				payment_amount: total_fee,
				token_count,
			});
		}

		Ok(())
	}

	/// Perform the mint operation and update storage accordingly.
	pub(crate) fn do_mint(
		collection_id: CollectionUuid,
		collection_info: CollectionInformation<
			T::AccountId,
			T::MaxTokensPerCollection,
			T::StringLimit,
		>,
		token_owner: &T::AccountId,
		serial_numbers: &BoundedVec<SerialNumber, T::MaxTokensPerCollection>,
	) -> DispatchResult {
		let mut new_collection_info = collection_info;
		// Update collection issuance
		new_collection_info.collection_issuance = new_collection_info
			.collection_issuance
			.checked_add(serial_numbers.len().saturated_into())
			.ok_or(ArithmeticError::Overflow)?;

		ensure!(
			new_collection_info.collection_issuance <= T::MaxTokensPerCollection::get(),
			Error::<T>::TokenLimitExceeded
		);

		new_collection_info
			.add_user_tokens(&token_owner, serial_numbers.clone())
			.map_err(|e| Error::<T>::from(e))?;

		// Update CollectionInfo storage
		<CollectionInfo<T>>::insert(collection_id, new_collection_info);
		Ok(())
	}

	/// Find the tokens owned by an `address` in the given collection
	/// limit return tokens that are larger than the cursor
	/// Returns list of tokens, the sum of all tokens owned by the user
	/// and the new cursor for the next owned SerialNumber not included in the returned list
	pub fn owned_tokens(
		collection_id: CollectionUuid,
		who: &T::AccountId,
		cursor: SerialNumber,
		limit: u16,
	) -> (SerialNumber, TokenCount, Vec<SerialNumber>) {
		let collection_info = match <CollectionInfo<T>>::get(collection_id) {
			Some(info) => info,
			None => return (Default::default(), Default::default(), Default::default()),
		};

		// Collect all tokens owned by address
		let mut owned_tokens: Vec<SerialNumber> = match collection_info
			.owned_tokens
			.into_inner()
			.iter()
			.find(|token_ownership| &token_ownership.owner == who)
		{
			Some(token_ownership) => token_ownership.owned_serials.clone().into_inner(),
			None => vec![],
		};

		// Sort the vec to ensure no tokens are missed
		owned_tokens.sort();
		// Store the last owned token by this account
		let last_id: SerialNumber = owned_tokens.last().copied().unwrap_or_default();
		// Get the sum of all tokens owned by this account
		let total_owned: TokenCount = owned_tokens.len().saturated_into();

		// Shorten list to any tokens above the cursor and return the limit
		// Note max limit is restricted by MAX_OWNED_TOKENS_LIMIT const
		let response: Vec<SerialNumber> = owned_tokens
			.into_iter()
			.filter(|serial_number| serial_number >= &cursor)
			.take(sp_std::cmp::min(limit, MAX_OWNED_TOKENS_LIMIT).into())
			.collect();

		let new_cursor: SerialNumber = match response.last().copied() {
			Some(highest) => {
				if highest != last_id {
					// There are still tokens remaining that aren't being returned in this call,
					// return the next cursor
					highest.saturating_add(1)
				} else {
					// 0 indicates that this is the end of the owned tokens
					0
				}
			},
			None => 0,
		};

		(new_cursor, total_owned, response)
	}

	/// Create the collection
	pub fn do_create_collection(
		owner: T::AccountId,
		name: BoundedVec<u8, T::StringLimit>,
		initial_issuance: TokenCount,
		max_issuance: Option<TokenCount>,
		token_owner: Option<T::AccountId>,
		metadata_scheme: MetadataScheme,
		royalties_schedule: Option<RoyaltiesSchedule<T::AccountId>>,
		origin_chain: OriginChain,
		cross_chain_compatibility: CrossChainCompatibility,
	) -> Result<u32, DispatchError> {
		// Check we can issue the new tokens
		let collection_uuid = Self::next_collection_uuid()?;

		// Check max issuance is valid
		if let Some(max_issuance) = max_issuance {
			ensure!(max_issuance > Zero::zero(), Error::<T>::InvalidMaxIssuance);
			ensure!(initial_issuance <= max_issuance, Error::<T>::InvalidMaxIssuance);
			ensure!(
				max_issuance <= T::MaxTokensPerCollection::get(),
				Error::<T>::InvalidMaxIssuance
			);
		}

		// Validate collection attributes
		ensure!(!name.is_empty(), Error::<T>::CollectionNameInvalid);
		ensure!(core::str::from_utf8(&name).is_ok(), Error::<T>::CollectionNameInvalid);
		if let Some(royalties_schedule) = royalties_schedule.clone() {
			// Check that the entitlements are less than MAX_ENTITLEMENTS - 2
			// This is because when the token is listed, two more entitlements will be added
			// for the network fee and marketplace fee
			ensure!(
				royalties_schedule.entitlements.len() <= MAX_COLLECTION_ENTITLEMENTS as usize,
				Error::<T>::RoyaltiesInvalid
			);
			ensure!(royalties_schedule.validate(), Error::<T>::RoyaltiesInvalid);
		}

		// Now mint the collection tokens
		let mut owned_tokens = BoundedVec::default();
		if initial_issuance > Zero::zero() {
			ensure!(initial_issuance <= T::MintLimit::get(), Error::<T>::MintLimitExceeded);
			// XLS-20 compatible collections cannot have an initial issuance
			// This is to prevent the fees from being bypassed in the mint function.
			// Instead the user should specify 0 initial_issuance and use the mint function to
			// mint tokens
			ensure!(!cross_chain_compatibility.xrpl, Error::<T>::InitialIssuanceNotZero);
			// mint initial tokens to token_owner or owner
			let token_owner = token_owner.unwrap_or(owner.clone());
			let serial_numbers_unbounded: Vec<SerialNumber> = (0..initial_issuance).collect();
			let serial_numbers: BoundedVec<SerialNumber, T::MaxTokensPerCollection> =
				BoundedVec::try_from(serial_numbers_unbounded)
					.map_err(|_| Error::<T>::TokenLimitExceeded)?;
			// Create token_ownership object with token_owner and initial serial_numbers
			let token_ownership = TokenOwnership::new(token_owner, serial_numbers);
			owned_tokens = BoundedVec::try_from(vec![token_ownership])
				.map_err(|_| Error::<T>::TokenLimitExceeded)?;
		}

		let collection_info = CollectionInformation {
			owner: owner.clone(),
			name: name.clone(),
			metadata_scheme: metadata_scheme.clone(),
			royalties_schedule: royalties_schedule.clone(),
			max_issuance,
			origin_chain: origin_chain.clone(),
			next_serial_number: initial_issuance,
			collection_issuance: initial_issuance,
			cross_chain_compatibility,
			owned_tokens,
		};
		<CollectionInfo<T>>::insert(collection_uuid, collection_info);

		// will not overflow, asserted prior qed.
		<NextCollectionId<T>>::mutate(|i| *i += u32::one());

		// Add some code to the EVM
		T::OnNewAssetSubscription::on_asset_create(
			collection_uuid,
			ERC721_PRECOMPILE_ADDRESS_PREFIX,
		);

		Self::deposit_event(Event::<T>::CollectionCreate {
			collection_uuid,
			initial_issuance,
			max_issuance,
			collection_owner: owner,
			metadata_scheme,
			name: name.into_inner(),
			royalties_schedule,
			origin_chain,
			compatibility: cross_chain_compatibility,
		});
		Ok(collection_uuid)
	}

	/// Check token locks and burn tokens
	pub fn do_burn(
		who: &T::AccountId,
		collection_id: CollectionUuid,
		serial_number: SerialNumber,
	) -> DispatchResult {
		ensure!(
			!<TokenLocks<T>>::contains_key((collection_id, serial_number)),
			Error::<T>::TokenLocked
		);

		CollectionInfo::<T>::try_mutate(collection_id, |maybe_collection_info| -> DispatchResult {
			let collection_info =
				maybe_collection_info.as_mut().ok_or(Error::<T>::NoCollectionFound)?;

			ensure!(collection_info.is_token_owner(who, serial_number), Error::<T>::NotTokenOwner);
			collection_info.collection_issuance =
				collection_info.collection_issuance.saturating_sub(1);
			collection_info.owned_tokens.iter_mut().for_each(|token_ownership| {
				if token_ownership.owner == *who {
					token_ownership.owned_serials.retain(|&serial| serial != serial_number)
				}
			});
			// Remove approvals for this token
			T::OnTransferSubscription::on_nft_transfer(&(collection_id, serial_number));
			Ok(())
		})
	}

	/// Enables XLS-20 compatibility for a collection with 0 issuance
	pub fn enable_xls20_compatibility(
		who: T::AccountId,
		collection_id: CollectionUuid,
	) -> DispatchResult {
		let mut collection_info =
			CollectionInfo::<T>::get(collection_id).ok_or(Error::<T>::NoCollectionFound)?;

		// Caller must be collection owner
		ensure!(collection_info.is_collection_owner(&who), Error::<T>::NotCollectionOwner);
		// Collection issuance must be 0 (i.e. no tokens minted)
		ensure!(
			collection_info.collection_issuance.is_zero(),
			Error::<T>::CollectionIssuanceNotZero
		);

		collection_info.cross_chain_compatibility.xrpl = true;
		CollectionInfo::<T>::insert(collection_id, collection_info);
		Ok(())
	}

	/// The account ID of the NFT pallet.
	pub fn account_id() -> T::AccountId {
		T::PalletId::get().into_account_truncating()
	}
}

impl<T: Config> NFTExt for Pallet<T> {
	type AccountId = T::AccountId;
	type MaxTokensPerCollection = T::MaxTokensPerCollection;
	type StringLimit = T::StringLimit;

	fn do_mint(
		origin: Self::AccountId,
		collection_id: CollectionUuid,
		quantity: TokenCount,
		token_owner: Option<Self::AccountId>,
	) -> DispatchResult {
		Self::mint(RawOrigin::Signed(origin).into(), collection_id, quantity, token_owner)
	}

	fn do_transfer(
		origin: Self::AccountId,
		collection_id: CollectionUuid,
		serial_numbers: Vec<SerialNumber>,
		new_owner: Self::AccountId,
	) -> DispatchResult {
		let bounded_serials =
			BoundedVec::try_from(serial_numbers).map_err(|_| Error::<T>::TokenLimitExceeded)?;
		Self::transfer(RawOrigin::Signed(origin).into(), collection_id, bounded_serials, new_owner)
	}

	fn do_create_collection(
		owner: Self::AccountId,
		name: BoundedVec<u8, Self::StringLimit>,
		initial_issuance: TokenCount,
		max_issuance: Option<TokenCount>,
		token_owner: Option<Self::AccountId>,
		metadata_scheme: MetadataScheme,
		royalties_schedule: Option<RoyaltiesSchedule<Self::AccountId>>,
		origin_chain: OriginChain,
	) -> Result<CollectionUuid, DispatchError> {
		Self::do_create_collection(
			owner,
			name,
			initial_issuance,
			max_issuance,
			token_owner,
			metadata_scheme,
			royalties_schedule,
			origin_chain,
			CrossChainCompatibility::default(),
		)
	}

	fn get_token_owner(token_id: &TokenId) -> Option<Self::AccountId> {
		let Some(collection) = CollectionInfo::<T>::get(token_id.0) else {
            return None;
        };
		collection.get_token_owner(token_id.1)
	}

	fn get_collection_info(
		collection_id: CollectionUuid,
	) -> Result<
		CollectionInformation<Self::AccountId, Self::MaxTokensPerCollection, Self::StringLimit>,
		DispatchError,
	> {
		CollectionInfo::<T>::get(collection_id).ok_or(Error::<T>::NoCollectionFound.into())
	}

	fn enable_xls20_compatibility(
		who: Self::AccountId,
		collection_id: CollectionUuid,
	) -> DispatchResult {
		Self::enable_xls20_compatibility(who, collection_id)
	}

	fn next_collection_uuid() -> Result<CollectionUuid, DispatchError> {
		Self::next_collection_uuid()
	}

	fn increment_collection_id() -> DispatchResult {
		ensure!(<NextCollectionId<T>>::get().checked_add(1).is_some(), Error::<T>::NoAvailableIds);
		<NextCollectionId<T>>::mutate(|i| *i += u32::one());
		Ok(())
	}

	fn get_token_lock(token_id: TokenId) -> Option<TokenLockReason> {
		<TokenLocks<T>>::get(token_id)
	}

	fn set_token_lock(token_id: TokenId, lock_reason: Option<TokenLockReason>) -> DispatchResult {
		match lock_reason {
			Some(reason) => {
				ensure!(!<TokenLocks<T>>::contains_key(token_id), Error::<T>::TokenLocked);
				<TokenLocks<T>>::insert(token_id, reason);
			},
			None => {
				<TokenLocks<T>>::remove(token_id);
			},
		}
		Ok(())
	}
}
