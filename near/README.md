Tipping Contract
================
```rust
struct TipsBalance {
	tips_balance_info: TipsBalanceInfo,
	account_id: Option<AccountId>,
	amount: Balance,
}
```
```rust
struct TipsBalanceInfo {
	server_id: String,
	reference_type: String,
	reference_id: String,
	ft_identifier: String,
}
```
```rust
struct TipsBalanceWithFormattedBalance {
	tips_balance: TipsBalance,
	symbol: String,
	formatted_amount: String,
}
```
```rust
struct FtIdentifierInfo {
	ft_identifier: FtIdentifier,
	decimals: u8,
	symbol: String,
}
```
```rust
struct TipsBalanceWithPagination {
	data: Vec<TipsBalanceWithFormattedBalance>,
	meta: Metadata,
}
```
```rust
struct FtIdentifierWithPagination {
	data: Vec<FtIdentifierInfo>,
	meta: Metadata,
}
```
```rust
struct Metadata {
	total_item_count: u64,
	total_page_count: u64,
	items_per_page: u64,
	current_page: Option<u64>,
	next_page: Option<u64>,
	previous_page: Option<u64>,
}
```
Calls
-----
### Send Tip
#### Send Tip With NEAR
```rust
fn send_tip(tips_balance_info: TipsBalanceInfo)
```
#### Send Tip With Fungible Token (Use FT Address)
```rust
fn ft_transfer_call(
	receiver_id: AccountId, // TippingContract Address
	amount: String,
	msg: String, // string of tips_balance_info
)
```
### Claim Tip
```rust
fn claim_tip(tips_balance_info: TipsBalanceInfo)
```
### Claim Reference
```rust
fn claim_reference(
	tips_balance_info: TipsBalanceInfo,
	reference_type: String,
	reference_id: String,
	account_id: AccountId,
)
```
Views
=====
### Contract Owner
```rust
fn get_owner()
```
### Fungible Token List
```rust
fn get_ft_identifiers(
	page_number: Option<u64>,
	page_limit: Option<u64>,
) -> FtIdentifierWithPagination
```
### TipsBalance
#### TipsBalance
```rust
fn get_tips_balance(tips_balance_info: TipsBalanceInfo) -> Option<TipsBalanceWithFormattedBalance>
```
#### TipsBalance List
```rust
fn get_tips_balances(
	server_id: String,
	reference_type: String,
	reference_id: String,
	page_number: Option<u64>,
	page_limit: Option<u64>,
) -> TipsBalanceWithPagination

```
