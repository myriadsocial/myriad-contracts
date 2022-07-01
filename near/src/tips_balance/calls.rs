use crate::*;

#[near_bindgen]
impl TippingContract {
	#[payable]
	pub fn send_tip(&mut self, tips_balance_info: TipsBalanceInfo) {
		let tip_amount = env::attached_deposit();
		let account_balance = env::account_balance();
		let ft_identifier = tips_balance_info.get_ft_identifier();

		assert!(ft_identifier == "native", "OnlyForNear");
		assert!(account_balance > tip_amount, "InsufficientBalance");
		assert!(tip_amount > 0, "TipsMustMoreThanMinimum");

		let sender = env::predecessor_account_id();
		let reference = tips_balance_info.reference();
		let amount_str = tip_amount.to_string();

		self.create_tips_balance(&tips_balance_info, &tip_amount);
		self.tip_log(&sender, &amount_str, "NEAR", 24, Some(reference));
	}

	#[payable]
	pub fn claim_tip(&mut self, tips_balance_info: TipsBalanceInfo) {
		assert_one_yocto();

		let tips_balance_key = tips_balance_info.key();
		let tips_balance = self.tips_balances.get(&tips_balance_key).expect("TipsBalanceNotExists");
		let amount = tips_balance.get_amount();
		let receiver = env::signer_account_id();
		let account_id = tips_balance.get_account_id().as_ref().expect("ReceiverNotExists");

		assert!(amount > 0, "NothingToClaimed");
		assert!(account_id == &receiver, "Unauthorized");

		let ft_id = tips_balance_info.get_ft_identifier();
		let tips_balance = tips_balance.set_balance(Zero::zero());
		let claim_tips_promise = self.transfer(&receiver, ft_id, amount);

		claim_tips_promise
			.then(Self::ext(env::current_account_id()).resolve_claim_tip(tips_balance, amount));
	}

	#[payable]
	pub fn batch_claim_tips(
		&mut self,
		server_id: ServerId,
		reference_type: ReferenceType,
		reference_id: ReferenceId,
	) {
		assert_one_yocto();

		let receiver = env::signer_account_id();
		let total_ft = self.ft_identifiers.len();
		let values = self.ft_identifiers.values_as_vector();
		let tips_balances: Vec<TipsBalance> = (0..total_ft)
			.filter_map(|index| {
				let ft_info = values.get(index).unwrap();
				let ft_identifier = ft_info.get_ft_identifier();
				let key =
					TipsBalanceKey::new(&server_id, &reference_type, &reference_id, ft_identifier);
				let tips_balance = self.tips_balances.get(&key);

				if let Some(tips_balance) = tips_balance {
					if tips_balance.get_amount().is_zero() {
						return None
					}

					if tips_balance.get_account_id().is_none() {
						return None
					}

					if tips_balance.get_account_id().as_ref().unwrap() != &receiver {
						return None
					}

					return Some(tips_balance)
				}

				None
			})
			.collect();

		assert!(!tips_balances.is_empty(), "NothingToClaimed");

		let init_tips_balance = &tips_balances[0];
		let init_ft = init_tips_balance.get_ft_identifier();
		let init_amount = init_tips_balance.get_amount();
		let receiver = env::signer_account_id();
		let mut tips_promise = self.transfer(&receiver, init_ft, init_amount);

		for tips_balance in tips_balances.iter().skip(1) {
			let ft = tips_balance.get_ft_identifier();
			let tips_amount = tips_balance.get_amount();
			let receiver = tips_balance.get_account_id().as_ref().unwrap();

			tips_promise = tips_promise.and(self.transfer(receiver, ft, tips_amount));
		}

		tips_promise
			.then(Self::ext(env::current_account_id()).resolve_batch_claim_tip(tips_balances));
	}

	pub fn claim_reference(
		&mut self,
		tips_balance_info: TipsBalanceInfo,
		reference_type: ReferenceType,
		reference_id: ReferenceId,
		account_id: AccountId,
		tx_fee: String,
	) {
		let receiver = env::signer_account_id();
		let tips_balance_info = tips_balance_info.set_server_id(&receiver);

		assert!(receiver != account_id, "Unauthorized");

		// Check near balance for tx fee
		let native_key = TipsBalanceKey::new(&receiver, &reference_type, &reference_id, "native");
		let native_tips_balance = self.tips_balances.get(&native_key).unwrap_or_else(|| {
			let tips_balance_info =
				TipsBalanceInfo::new(&receiver, &reference_type, &reference_id, "native");
			TipsBalance::new(&tips_balance_info)
		});
		let total_tip = native_tips_balance.get_amount();
		let tx_fee = tx_fee.parse::<Balance>().expect("FailedParseTxFee");
		let tx_fee = if total_tip >= tx_fee { tx_fee } else { 0 };

		// Calculate tips
		let main_balance = self.calculate_tips(&tips_balance_info, &reference_type, &reference_id);
		let main_balance_key = main_balance.key();

		let mut balance = main_balance.get_amount();

		let native_tips_balance: Option<TipsBalance> = if main_balance_key != native_key {
			Some(native_tips_balance.set_balance(total_tip - tx_fee).set_account_id(&account_id))
		} else {
			balance -= tx_fee;
			None
		};

		Promise::new(receiver).transfer(tx_fee).then(
			Self::ext(env::current_account_id()).resolve_claim_reference(
				tips_balance_info.key(),
				main_balance.set_account_id(&account_id).set_balance(balance),
				native_tips_balance,
			),
		);
	}

	#[payable]
	pub fn batch_claim_references(
		&mut self,
		reference_type: ReferenceType,
		reference_ids: Vec<ReferenceId>,
		main_ref_type: ReferenceType,
		main_ref_id: ReferenceId,
		account_id: AccountId,
		tx_fee: String,
	) {
		let receiver = env::signer_account_id();

		assert_one_yocto();
		assert!(receiver != account_id, "Unauthorized");

		// Check near balance
		let native_key = TipsBalanceKey::new(&receiver, &main_ref_type, &main_ref_id, "native");
		let native_tips = self.tips_balances.get(&native_key).unwrap_or_else(|| {
			let tips_balance_info =
				TipsBalanceInfo::new(&receiver, &main_ref_type, &main_ref_id, "native");
			TipsBalance::new(&tips_balance_info)
		});
		let total_tip = native_tips.get_amount();
		let tx_fee = tx_fee.parse::<Balance>().expect("FailedParseTxFee");
		let tx_fee = if total_tip >= tx_fee { tx_fee } else { 0 };

		let (main_tip_balances, keys) = self.batch_calculate_tips(
			&reference_type,
			&reference_ids,
			&main_ref_type,
			&main_ref_id,
			&account_id,
			tx_fee,
		);

		Promise::new(receiver).transfer(tx_fee).then(
			Self::ext(env::current_account_id())
				.resolve_batch_claim_reference(keys, main_tip_balances),
		);
	}
}
