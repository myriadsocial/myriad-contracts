use crate::*;

trait FungibleTokenReceiver {
	fn ft_on_transfer(
		&mut self,
		sender_id: AccountId,
		amount: U128,
		msg: String,
	) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for TippingContract {
	fn ft_on_transfer(
		&mut self,
		sender_id: AccountId,
		amount: U128,
		msg: String,
	) -> PromiseOrValue<U128> {
		let tips_balance_info = from_str::<TipsBalanceInfo>(&msg).expect("InvalidArgument");

		self.send_tip_by_ft(tips_balance_info, sender_id, amount.into())
	}
}
