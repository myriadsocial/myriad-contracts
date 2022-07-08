use crate::*;

#[near_bindgen]
impl TippingContract {
	pub fn get_tips_balance(
		&self,
		tips_balance_info: TipsBalanceInfo,
	) -> Option<TipsBalanceWithFormattedBalance> {
		let ft_identifier = tips_balance_info.get_ft_identifier().to_string();
		let tips_balance = self.tips_balances.get(&tips_balance_info.key())?;
		let ft_identifier_info = self.ft_identifiers.get(&ft_identifier)?;

		let symbol = ft_identifier_info.get_symbol();
		let decimals = ft_identifier_info.get_decimals().into();
		let balance = tips_balance.get_amount_str();
		let formatted = self.formatted_balance(balance.as_str(), decimals);

		let result = TipsBalanceWithFormattedBalance {
			tips_balance,
			symbol: symbol.to_string(),
			formatted_amount: formatted,
			unclaimed_reference_ids: Vec::new(),
		};

		Some(result)
	}

	#[allow(clippy::too_many_arguments)]
	pub fn get_tips_balances(
		&self,
		server_id: ServerId,
		reference_type: ReferenceType,
		reference_ids: Vec<ReferenceId>,
		main_ref_type: ReferenceType,
		main_ref_id: ReferenceId,
		page_number: Option<u64>,
		page_limit: Option<u64>,
	) -> TipsBalanceWithPagination {
		if self.ft_identifiers.is_empty() {
			return TipsBalanceWithPagination::default()
		}

		let total_item = self.ft_identifiers.len();
		let meta = self.metadata(total_item, page_number, page_limit);

		if meta.current_page.is_none() {
			return TipsBalanceWithPagination::default()
		}

		let page_number = meta.current_page.unwrap();
		let page_limit = meta.items_per_page;
		let from_index: u64 = (page_number - 1) * page_limit;
		let values = self.ft_identifiers.values_as_vector();

		let data = (from_index..min(from_index + page_limit, total_item))
			.filter_map(|index| {
				if let Some(ft_identifier_info) = values.get(index) {
					let ft_identifier = ft_identifier_info.get_ft_identifier();
					let mut total_tips: Balance = Zero::zero();
					let mut unclaimed_reference_ids = Vec::<String>::new();

					for reference_id in reference_ids.iter() {
						let key = TipsBalanceKey::new(
							&server_id,
							&reference_type,
							reference_id,
							ft_identifier,
						);

						let tips_balance = self.tips_balances.get(&key);

						if let Some(tips_balance) = tips_balance {
							if tips_balance.get_amount() > 0 {
								total_tips += tips_balance.get_amount();
								unclaimed_reference_ids
									.push(tips_balance.get_reference_id().to_string());
							}
						}
					}

					let symbol = ft_identifier_info.get_symbol();
					let decimals = ft_identifier_info.get_decimals().into();
					let key = TipsBalanceKey::new(
						&server_id,
						&main_ref_type,
						&main_ref_id,
						ft_identifier,
					);

					let tips_balance = self
						.tips_balances
						.get(&key)
						.unwrap_or_else(|| {
							let tips_balance_info = TipsBalanceInfo::new(
								&server_id,
								&main_ref_type,
								&main_ref_id,
								ft_identifier,
							);

							TipsBalance::new(&tips_balance_info)
						})
						.add_balance(total_tips);

					let balance = tips_balance.get_amount_str();
					let formatted = self.formatted_balance(balance.as_str(), decimals);

					let result = TipsBalanceWithFormattedBalance {
						tips_balance,
						symbol: symbol.to_string(),
						formatted_amount: formatted,
						unclaimed_reference_ids,
					};

					Some(result)
				} else {
					None
				}
			})
			.collect();

		TipsBalanceWithPagination { data, meta }
	}
}
