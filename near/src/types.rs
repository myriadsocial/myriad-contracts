use crate::*;

pub const ONE_YOCTO: Balance = 1;

#[derive(
	BorshDeserialize, BorshSerialize, Serialize, Deserialize, Default, Clone, PartialEq, Eq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
	pub total_item_count: u64,
	pub total_page_count: u64,
	pub items_per_page: u64,
	pub current_page: Option<u64>,
	pub next_page: Option<u64>,
	pub previous_page: Option<u64>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
	FtIdentifierInfo,
	TipsBalance,
}
