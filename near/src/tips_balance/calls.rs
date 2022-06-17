use crate::*;

#[near_bindgen]
impl TippingContract {
	#[payable]
	pub fn send_tip(&mut self, tips_balance_info: TipsBalanceInfo) {
		let tip_amount = env::attached_deposit();
		let account_balance = env::account_balance();
		let minimum_tip: Balance = 100000000000000000000000; // 0.1 NEAR
		let ft_identifier = tips_balance_info.get_ft_identifier();

		assert!(ft_identifier == "native", "OnlyForNear");
		assert!(account_balance > tip_amount, "InsufficientBalance");
		assert!(tip_amount >= minimum_tip, "TipsMustMoreThanMinimum");

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

		let tips_balance = tips_balance.set_balance(Zero::zero());

		if tips_balance_info.get_ft_identifier() == "native" {
			Promise::new(receiver.clone()).transfer(amount);

			self.tips_balances.insert(&tips_balance_key, &tips_balance);
			self.tip_log(&receiver, &amount.to_string(), "NEAR", 24, None);
		} else {
			let ft_identifier = tips_balance_info.get_ft_id().expect("NotAnAccountId");
			self.assert_account_id(&ft_identifier);

			ft_contract::ext(ft_identifier)
				.with_attached_deposit(ONE_YOCTO)
				.ft_transfer(receiver, U128(amount), None)
				.then(Self::ext(env::current_account_id()).resolve_claim_tip(tips_balance, amount));
		}
	}

	#[payable]
	pub fn claim_reference(
		&mut self,
		tips_balance_info: TipsBalanceInfo,
		reference_type: ReferenceType,
		reference_id: ReferenceId,
		account_id: AccountId,
	) {
		assert_one_yocto();
		assert!(env::signer_account_id() == self.owner, "Unauthorized");
		self.assert_account_id(&account_id);

		let ft_identifier = tips_balance_info.get_ft_identifier();
		let ft_identifier_info = self.ft_identifiers.get(&ft_identifier.to_string());

		if ft_identifier_info.is_some() {
			self.claim_tips_balance(
				&tips_balance_info,
				&reference_type,
				&reference_id,
				&account_id,
			);
		} else {
			let ft_identifier = tips_balance_info.get_ft_id().expect("NotAnAccountId");
			self.assert_account_id(&ft_identifier);

			ft_contract::ext(ft_identifier).ft_metadata().then(
				Self::ext(env::current_account_id()).resolve_claim_reference(
					tips_balance_info,
					reference_type,
					reference_id,
					account_id,
				),
			);
		}
	}
}
