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
	pub fn resolve_batch_claim_tip(&mut self, tips_balances: Vec<TipsBalance>) {
		self.internal_resolve_batch_claim_tip(tips_balances);
	}

	#[private]
	pub fn resolve_claim_reference(
		&mut self,
		secondary_key: TipsBalanceKey,
		main_balance: TipsBalance,
		native_tips_balance: Option<TipsBalance>,
	) {
		self.internal_resolve_claim_reference(secondary_key, main_balance, native_tips_balance);
	}

	#[private]
	pub fn resolve_batch_claim_reference(
		&mut self,
		secondary_keys: Vec<TipsBalanceKey>,
		main_balances: Vec<TipsBalance>,
	) {
		self.internal_resolve_batch_claim_reference(secondary_keys, main_balances);
	}
}
