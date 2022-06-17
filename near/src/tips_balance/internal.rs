use crate::*;

impl TippingContract {
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
		tips_balance_info: &TipsBalanceInfo,
		reference_type: &ReferenceType,
		reference_id: &ReferenceId,
		account_id: &AccountId,
	) -> TipsBalance {
		let mut key = tips_balance_info.key();

		let tips_balance = if tips_balance_info.get_reference_type() == reference_type {
			assert!(tips_balance_info.get_reference_id() == reference_id, "WrongFormat");

			self.tips_balances
				.get(&key)
				.unwrap_or_else(|| TipsBalance::new(tips_balance_info))
		} else {
			let mut tip = Zero::zero();

			if let Some(tips_balance) = self.tips_balances.get(&key) {
				tip += tips_balance.get_amount();
				self.tips_balances.remove(&key);
			}

			key.set_reference(reference_type, reference_id);

			match self.tips_balances.get(&key) {
				Some(res) => res.add_balance(tip),
				None => TipsBalance::new(tips_balance_info)
					.set_balance(tip)
					.set_reference(reference_type, reference_id),
			}
		};

		let tips_balance = tips_balance.set_account_id(account_id);

		self.tips_balances.insert(&key, &tips_balance);
		self.claim_reference_log(reference_type, reference_id);

		tips_balance
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

	pub(crate) fn internal_resolve_claim_reference(
		&mut self,
		tips_balance_info: TipsBalanceInfo,
		reference_type: ReferenceType,
		reference_id: ReferenceId,
		account_id: AccountId,
	) {
		require!(env::promise_results_count() == 1);

		match env::promise_result(0) {
			PromiseResult::Successful(val) => {
				let ft_metadata = from_slice::<FungibleTokenMetadata>(&val).expect("TokenNotFound");
				let symbol = ft_metadata.get_symbol();
				let decimals = ft_metadata.get_decimals();
				let ft_identifier = tips_balance_info.get_ft_identifier().to_string();

				self.create_ft_identifier(&ft_identifier, symbol, decimals);
				self.claim_tips_balance(
					&tips_balance_info,
					&reference_type,
					&reference_id,
					&account_id,
				);
			},
			_ => env::panic_str("TokenNotFound"),
		};
	}
}
