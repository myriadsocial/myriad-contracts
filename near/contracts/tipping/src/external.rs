use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FungibleTokenMetadata {
	pub spec: String,
	pub name: String,
	pub symbol: String,
	pub icon: Option<String>,
	pub reference: Option<String>,
	pub reference_hash: Option<Base64VecU8>,
	pub decimals: u8,
}
impl FungibleTokenMetadata {
	pub fn get_symbol(&self) -> &str {
		&self.symbol
	}

	pub fn get_decimals(&self) -> &u8 {
		&self.decimals
	}
}

#[ext_contract(ft_contract)]
trait FtContract {
	fn ft_metadata() -> FungibleTokenMetadata;
	fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}
