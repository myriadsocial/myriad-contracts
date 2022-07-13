mod external;
mod ft_callback;
mod internal;
mod tips_balance;
mod types;

use external::*;
use near_sdk::{
	assert_one_yocto,
	borsh::{self, BorshDeserialize, BorshSerialize},
	collections::UnorderedMap,
	env, ext_contract,
	json_types::{Base64VecU8, U128},
	log, near_bindgen, require,
	serde::{Deserialize, Serialize},
	serde_json::{from_slice, from_str},
	AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};
use num_traits::Zero;
use std::cmp::min;
use tips_balance::types::*;
use types::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Tipping {
	pub owner: AccountId,
	pub tips_balances: UnorderedMap<TipsBalanceKey, TipsBalance>,
	pub ft_identifiers: UnorderedMap<FtIdentifier, FtIdentifierInfo>,
}

#[near_bindgen]
impl Tipping {
	#[init]
	pub fn new(ft_identifiers: Option<Vec<FtIdentifier>>) -> Self {
		assert!(!env::state_exists(), "Already initialized");

		let mut this = Self {
			owner: env::signer_account_id(),
			tips_balances: UnorderedMap::new(StorageKeys::TipsBalance),
			ft_identifiers: UnorderedMap::new(StorageKeys::FtIdentifierInfo),
		};

		let near = FtIdentifierInfo::new("native", 24, "NEAR");

		this.ft_identifiers.insert(&"native".to_string(), &near);

		if let Some(ft_identifiers) = ft_identifiers {
			for ft_identifier in ft_identifiers {
				let ft_identifier = ft_identifier.parse::<AccountId>();

				if let Ok(ft_identifier) = ft_identifier {
					if !env::is_valid_account_id(ft_identifier.as_bytes()) {
						continue
					}

					ft_contract::ext(ft_identifier.clone()).ft_metadata().then(
						Self::ext(env::current_account_id())
							.resolve_ft_identifier(ft_identifier.to_string()),
					);
				}
			}
		}

		this
	}

	#[private]
	pub fn resolve_ft_identifier(&mut self, ft_identifier: FtIdentifier) {
		self.internal_resolve_ft_identifier(ft_identifier);
	}

	// call
	pub fn transfer_owner_key(&mut self, new_owner: AccountId) -> &'static str {
		assert!(env::signer_account_id() == self.owner, "UnauthorizedAdmin");

		self.owner = new_owner;

		"OwnerTransferred"
	}

	// view
	pub fn get_owner(&self) -> AccountId {
		self.owner.clone()
	}

	pub fn get_ft_identifiers(
		&self,
		page_number: Option<u64>,
		page_limit: Option<u64>,
	) -> FtIdentifierWithPagination {
		if self.ft_identifiers.is_empty() {
			return FtIdentifierWithPagination::default()
		}

		let total_item = self.ft_identifiers.len();
		let meta = self.metadata(total_item, page_number, page_limit);

		if meta.current_page.is_none() {
			return FtIdentifierWithPagination::default()
		}

		let page_number = meta.current_page.unwrap();
		let page_limit = meta.items_per_page;
		let from_index: u64 = (page_number - 1) * page_limit;
		let values = self.ft_identifiers.values_as_vector();
		let data = (from_index..min(from_index + page_limit, total_item))
			.filter_map(|index| values.get(index))
			.collect();

		FtIdentifierWithPagination { data, meta }
	}
}
