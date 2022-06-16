use crate::*;

#[near_bindgen]
impl TippingContract {
	#[private]
	pub fn send_tip_by_ft(
		&mut self,
		tips_balance_info: TipsBalanceInfo,
		sender: AccountId,
		amount: Balance,
	) -> PromiseOrValue<U128> {
		let ft_identifier = tips_balance_info.get_ft_id().expect("NotAnAccountId");

		self.assert_account_id(&ft_identifier);
		self.assert_account_id(&sender);

		ft_contract::ext(ft_identifier)
			.ft_metadata()
			.then(Self::ext(env::current_account_id()).resolve_send_tip(
				sender,
				tips_balance_info,
				amount,
			))
			.into()
	}

	#[private]
	pub fn resolve_send_tip(
		&mut self,
		sender: AccountId,
		tips_balance_info: TipsBalanceInfo,
		amount: Balance,
	) -> U128 {
		self.internal_resolve_send_tip(sender, tips_balance_info, amount)
	}

	#[private]
	pub fn resolve_claim_tip(&mut self, tips_balance: TipsBalance, amount: Balance) {
		self.internal_resolve_claim_tip(tips_balance, amount);
	}

	#[private]
	pub fn resolve_claim_reference(
		&mut self,
		tips_balance_info: TipsBalanceInfo,
		reference_type: ReferenceType,
		reference_id: ReferenceId,
		account_id: AccountId,
	) {
		self.internal_resolve_claim_reference(
			tips_balance_info,
			reference_type,
			reference_id,
			account_id,
		);
	}
}
