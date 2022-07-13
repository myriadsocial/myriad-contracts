use crate::*;

impl Tipping {
	pub(crate) fn transfer(
		&mut self,
		receiver: &AccountId,
		ft_identifier: &str,
		amount: Balance,
	) -> Promise {
		if ft_identifier == "native" {
			Promise::new(receiver.clone()).transfer(amount)
		} else {
			let ft_identifier = ft_identifier.parse::<AccountId>().unwrap();
			ft_contract::ext(ft_identifier).with_attached_deposit(ONE_YOCTO).ft_transfer(
				receiver.clone(),
				U128(amount),
				None,
			)
		}
	}

	pub(crate) fn create_tips_balance(
		&mut self,
		tips_balance_info: &TipsBalanceInfo,
		amount: &Balance,
	) -> TipsBalance {
		let tips_balance_info = tips_balance_info.clone();
		let tips_balance_key = tips_balance_info.key();

		let tips_balance = match self.tips_balances.get(&tips_balance_key) {
			Some(tips_balance) => tips_balance.add_balance(*amount),
			None => TipsBalance::new(&tips_balance_info).set_balance(*amount),
		};

		self.tips_balances.insert(&tips_balance_key, &tips_balance);

		tips_balance
	}

	pub(crate) fn claim_tips_balance(
		&mut self,
		secondary_key: &TipsBalanceKey,
		main_balance: &TipsBalance,
	) -> TipsBalance {
		let other_tips_balance = self.tips_balances.get(secondary_key);

		if let Some(tips_balance) = other_tips_balance {
			if !tips_balance.get_amount().is_zero() {
				let tips_balance = tips_balance.set_balance(Zero::zero());
				self.tips_balances.insert(secondary_key, &tips_balance);
			}
		}

		let key = main_balance.key();
		let reference_type = main_balance.get_reference_type();
		let reference_id = main_balance.get_reference_id();

		self.tips_balances.insert(&key, main_balance);
		self.claim_reference_log(reference_type, reference_id);

		main_balance.clone()
	}

	pub(crate) fn batch_claim_tips_balance(
		&mut self,
		secondary_keys: &Vec<TipsBalanceKey>,
		main_balances: &Vec<TipsBalance>,
	) {
		for secondary_key in secondary_keys {
			let other_tips_balance = self.tips_balances.get(secondary_key);

			if let Some(tips_balance) = other_tips_balance {
				if !tips_balance.get_amount().is_zero() {
					let tips_balance = tips_balance.set_balance(Zero::zero());
					self.tips_balances.insert(secondary_key, &tips_balance);
				}
			}
		}

		for main_balance in main_balances {
			let key = main_balance.key();
			let reference_type = main_balance.get_reference_type();
			let reference_id = main_balance.get_reference_id();

			self.tips_balances.insert(&key, main_balance);
			self.claim_reference_log(reference_type, reference_id);
		}
	}

	pub(crate) fn calculate_tips(
		&mut self,
		tips_balance_info: &TipsBalanceInfo,
		reference_type: &ReferenceType,
		reference_id: &ReferenceId,
	) -> TipsBalance {
		let mut key = tips_balance_info.key();
		let mut tip = Zero::zero();

		if let Some(tips_balance) = self.tips_balances.get(&key) {
			tip += tips_balance.get_amount();
		}

		key.set_reference(reference_type, reference_id);

		match self.tips_balances.get(&key) {
			Some(res) => res.add_balance(tip),
			None => TipsBalance::new(tips_balance_info)
				.set_balance(tip)
				.set_reference(reference_type, reference_id),
		}
	}

	pub(crate) fn batch_calculate_tips(
		&mut self,
		reference_type: &ReferenceType,
		reference_ids: &Vec<ReferenceId>,
		main_ref_type: &ReferenceType,
		main_ref_id: &ReferenceId,
		account_id: &AccountId,
		tx_fee: Balance,
	) -> (Vec<TipsBalance>, Vec<TipsBalanceKey>) {
		let server_id = env::signer_account_id();
		let values = self.ft_identifiers.values_as_vector();
		let ft_infos: Vec<FtIdentifierInfo> =
			(0..self.ft_identifiers.len()).filter_map(|index| values.get(index)).collect();

		let mut tips_balances = Vec::<TipsBalance>::new();
		let mut keys = Vec::<TipsBalanceKey>::new();

		for ft_info in ft_infos.iter() {
			let mut tip: Balance = Zero::zero();
			let ft_id = ft_info.get_ft_identifier();
			for reference_id in reference_ids {
				let key = TipsBalanceKey::new(&server_id, reference_type, reference_id, ft_id);
				if let Some(tips_balance) = self.tips_balances.get(&key) {
					if !tips_balance.get_amount().is_zero() {
						keys.push(key);
						tip += tips_balance.get_amount();
					}
				}
			}

			let key = TipsBalanceKey::new(&server_id, main_ref_type, main_ref_id, ft_id);
			let main_tips_balance = match self.tips_balances.get(&key) {
				Some(res) => res.add_balance(tip),
				None => {
					let tips_balance_info =
						TipsBalanceInfo::new(&server_id, main_ref_type, main_ref_id, ft_id);

					TipsBalance::new(&tips_balance_info).set_balance(tip)
				},
			};

			let main_tips_balance = if ft_id == "native" {
				let amount = main_tips_balance.get_amount();
				main_tips_balance.set_balance(amount - tx_fee)
			} else {
				main_tips_balance
			};

			tips_balances.push(main_tips_balance.set_account_id(account_id));
		}

		(tips_balances, keys)
	}

	pub(crate) fn internal_resolve_send_tip(
		&mut self,
		sender: AccountId,
		tips_balance_info: TipsBalanceInfo,
		amount: Balance,
	) -> U128 {
		require!(env::promise_results_count() == 1);

		if let PromiseResult::Successful(val) = env::promise_result(0) {
			if let Ok(ft_metadata) = from_slice::<FungibleTokenMetadata>(&val) {
				let ft_identifier = tips_balance_info.get_ft_identifier().to_string();
				let symbol = ft_metadata.get_symbol();
				let decimals = *ft_metadata.get_decimals() as usize;
				let amount_str = amount.to_string();
				let reference = tips_balance_info.reference();

				self.create_ft_identifier(&ft_identifier, symbol, &(decimals as u8));
				self.create_tips_balance(&tips_balance_info, &amount);
				self.tip_log(&sender, &amount_str, symbol, decimals, Some(reference));

				return U128(0)
			}
		}

		U128(amount)
	}

	pub(crate) fn internal_resolve_claim_tip(
		&mut self,
		tips_balance: TipsBalance,
		amount: Balance,
	) {
		require!(env::promise_results_count() == 1);

		if let PromiseResult::Successful(_) = env::promise_result(0) {
			let tips_balance_key = tips_balance.key();
			let ft_identifier = tips_balance.get_ft_identifier().to_string();

			if let Some(ft_identifier_info) = self.ft_identifiers.get(&ft_identifier) {
				let symbol = ft_identifier_info.get_symbol();
				let decimals = ft_identifier_info.get_decimals() as usize;
				let receiver = tips_balance.get_account_id().clone().unwrap();
				let amount_str = amount.to_string();

				self.tip_log(&receiver, &amount_str, symbol, decimals, None);
			}

			self.tips_balances.insert(&tips_balance_key, &tips_balance);
		}
	}

	pub(crate) fn internal_resolve_batch_claim_tip(&mut self, tips_balances: Vec<TipsBalance>) {
		require!(env::promise_results_count() == tips_balances.len() as u64);

		for tips_balance in tips_balances {
			let key = tips_balance.key();
			let amount = tips_balance.get_amount();
			let tips_balance = tips_balance.set_balance(Zero::zero());
			let ft_identifier = tips_balance.get_ft_identifier().to_string();

			if let Some(ft_info) = self.ft_identifiers.get(&ft_identifier) {
				let symbol = ft_info.get_symbol();
				let decimals = ft_info.get_decimals() as usize;
				let receiver = tips_balance.get_account_id().clone().unwrap();
				let amount_str = amount.to_string();

				self.tip_log(&receiver, &amount_str, symbol, decimals, None);
			}

			self.tips_balances.insert(&key, &tips_balance);
		}
	}

	pub(crate) fn internal_resolve_claim_reference(
		&mut self,
		secondary_key: TipsBalanceKey,
		main_balance: TipsBalance,
		native_tips_balance: Option<TipsBalance>,
	) {
		require!(env::promise_results_count() == 1);

		match env::promise_result(0) {
			PromiseResult::Successful(_) => {
				if let Some(native) = native_tips_balance {
					let native_key = native.key();
					self.tips_balances.insert(&native_key, &native);
				}

				self.claim_tips_balance(&secondary_key, &main_balance);
			},
			_ => env::panic_str("TokenNotFound"),
		};
	}

	pub(crate) fn internal_resolve_batch_claim_reference(
		&mut self,
		secondary_keys: Vec<TipsBalanceKey>,
		main_balances: Vec<TipsBalance>,
	) {
		require!(env::promise_results_count() == 1);

		match env::promise_result(0) {
			PromiseResult::Successful(_) => {
				self.batch_claim_tips_balance(&secondary_keys, &main_balances);
			},
			_ => env::panic_str("TokenNotFound"),
		};
	}
}
