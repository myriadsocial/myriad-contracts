use crate::*;

pub type ServerId = String;
pub type ReferenceType = String;
pub type ReferenceId = String;
pub type FtIdentifier = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct TipsBalanceKey(ServerId, ReferenceType, ReferenceId, FtIdentifier);
impl TipsBalanceKey {
	pub fn new(
		server_id: &str,
		reference_type: &str,
		reference_id: &str,
		ft_identifier: &str,
	) -> Self {
		Self(
			server_id.to_string(),
			reference_type.to_string(),
			reference_id.to_string(),
			ft_identifier.to_string(),
		)
	}

	pub fn set_reference(&mut self, reference_type: &str, reference_id: &str) {
		self.1 = reference_type.to_string();
		self.2 = reference_id.to_string();
	}
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct TipsBalance {
	tips_balance_info: TipsBalanceInfo,
	account_id: Option<AccountId>,
	amount: Balance,
}
impl TipsBalance {
	pub fn new(tips_balance_info: &TipsBalanceInfo) -> Self {
		Self {
			tips_balance_info: tips_balance_info.clone(),
			account_id: None,
			amount: Zero::zero(),
		}
	}

	pub fn get_tips_balance_info(&self) -> &TipsBalanceInfo {
		&self.tips_balance_info
	}

	pub fn key(&self) -> TipsBalanceKey {
		self.tips_balance_info.key()
	}

	pub fn get_ft_identifier(&self) -> &str {
		self.get_tips_balance_info().get_ft_identifier()
	}

	pub fn get_account_id(&self) -> &Option<AccountId> {
		&self.account_id
	}

	pub fn get_amount(&self) -> Balance {
		self.amount
	}

	pub fn get_amount_str(&self) -> String {
		(self.amount).to_string()
	}

	pub fn add_balance(mut self, amount: Balance) -> Self {
		self.amount += amount;
		self
	}

	pub fn set_balance(mut self, amount: Balance) -> Self {
		self.amount = amount;
		self
	}

	pub fn set_account_id(mut self, account_id: &AccountId) -> Self {
		self.account_id = Some(account_id.clone());
		self
	}

	pub fn set_reference(mut self, reference_type: &str, reference_id: &str) -> Self {
		let tips_balance_info = self.tips_balance_info.set_reference(reference_type, reference_id);
		self.tips_balance_info = tips_balance_info;
		self
	}
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct TipsBalanceInfo {
	server_id: ServerId,
	reference_type: ReferenceType,
	reference_id: ReferenceId,
	ft_identifier: FtIdentifier,
}
impl TipsBalanceInfo {
	pub fn new(
		server_id: &str,
		reference_type: &str,
		reference_id: &str,
		ft_identifier: &str,
	) -> Self {
		Self {
			server_id: server_id.to_string(),
			reference_type: reference_type.to_string(),
			reference_id: reference_id.to_string(),
			ft_identifier: ft_identifier.to_string(),
		}
	}

	pub fn key(&self) -> TipsBalanceKey {
		TipsBalanceKey::new(
			&self.server_id,
			&self.reference_type,
			&self.reference_id,
			&self.ft_identifier,
		)
	}

	pub fn get_reference_type(&self) -> &str {
		&self.reference_type
	}

	pub fn get_reference_id(&self) -> &str {
		&self.reference_id
	}

	pub fn get_ft_identifier(&self) -> &str {
		&self.ft_identifier
	}

	pub fn get_ft_id(&self) -> Option<AccountId> {
		let ft_identifier = (&self.ft_identifier).to_string();
		match ft_identifier.parse::<AccountId>() {
			Ok(ft_identifier) => Some(ft_identifier),
			Err(_) => None,
		}
	}

	pub fn reference(&self) -> String {
		let reference_type = &self.reference_type;
		let reference_id = &self.reference_id;

		format!("{}/{}", reference_type, reference_id)
	}

	pub fn set_reference(mut self, reference_type: &str, reference_id: &str) -> Self {
		self.reference_type = reference_type.to_string();
		self.reference_id = reference_id.to_string();
		self
	}
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TipsBalanceWithFormattedBalance {
	pub tips_balance: TipsBalance,
	pub symbol: String,
	pub formatted_amount: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct TipsBalanceWithPagination {
	pub data: Vec<TipsBalanceWithFormattedBalance>,
	pub meta: Metadata,
}
impl Default for TipsBalanceWithPagination {
	fn default() -> Self {
		let data = Vec::new();
		let meta = Metadata::default();

		Self { data, meta }
	}
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct FtIdentifierInfo {
	ft_identifier: FtIdentifier,
	decimals: u8,
	symbol: String,
}
impl FtIdentifierInfo {
	pub fn new(ft_identifier: &str, decimals: u8, symbol: &str) -> Self {
		Self { ft_identifier: ft_identifier.to_string(), decimals, symbol: symbol.to_string() }
	}

	pub fn get_ft_identifier(&self) -> &str {
		&self.ft_identifier
	}

	pub fn get_symbol(&self) -> &str {
		&self.symbol
	}

	pub fn get_decimals(&self) -> u8 {
		self.decimals
	}
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct FtIdentifierWithPagination {
	pub data: Vec<FtIdentifierInfo>,
	pub meta: Metadata,
}
impl Default for FtIdentifierWithPagination {
	fn default() -> Self {
		let data = Vec::new();
		let meta = Metadata::default();

		Self { data, meta }
	}
}
