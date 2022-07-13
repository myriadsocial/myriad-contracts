use crate::*;

impl Tipping {
	pub(crate) fn create_ft_identifier(
		&mut self,
		ft_identifier: &FtIdentifier,
		symbol: &str,
		decimals: &u8,
	) {
		let ft_identifier_info = self.ft_identifiers.get(ft_identifier);

		if ft_identifier_info.is_none() {
			let ft_identifier_info = FtIdentifierInfo::new(ft_identifier, *decimals, symbol);
			self.ft_identifiers.insert(ft_identifier, &ft_identifier_info);
		}
	}

	pub(crate) fn internal_resolve_ft_identifier(&mut self, ft_identifier: FtIdentifier) {
		require!(env::current_account_id() == env::predecessor_account_id());
		require!(env::promise_results_count() == 1);

		match env::promise_result(0) {
			PromiseResult::Successful(val) => {
				if let Ok(ft_metadata) = from_slice::<FungibleTokenMetadata>(&val) {
					let symbol = ft_metadata.get_symbol();
					let decimals = ft_metadata.get_decimals();
					self.create_ft_identifier(&ft_identifier, symbol, decimals);
				}
			},
			_ => log!("This token {} not exists", ft_identifier),
		}
	}

	pub(crate) fn metadata(
		&self,
		total_item: u64,
		page_number: Option<u64>,
		page_limit: Option<u64>,
	) -> Metadata {
		let page_number = if let Some(number) = page_number {
			if number == 0 {
				1
			} else {
				number
			}
		} else {
			1
		};

		let page_limit = if let Some(limit) = page_limit {
			if limit == 0 {
				5
			} else {
				limit
			}
		} else {
			5
		};

		let total_item_count = total_item;
		let items_per_page = if page_limit > total_item { total_item } else { page_limit };

		let total_page_count = ((total_item_count as f64) / (items_per_page as f64)).ceil();
		let total_page_count = total_page_count as u64;

		let mut meta = Metadata {
			total_item_count,
			total_page_count,
			items_per_page,
			current_page: None,
			next_page: None,
			previous_page: None,
		};

		if page_number <= meta.total_page_count {
			meta.current_page = Some(page_number);

			if page_number != 1 {
				meta.previous_page = Some(page_number - 1);
			}

			if page_number != meta.total_page_count {
				meta.next_page = Some(page_number + 1)
			}
		}

		if let Some(next_page) = meta.next_page {
			if next_page == page_number {
				meta.items_per_page = if page_limit > total_item { total_item } else { page_limit }
			}
		}

		meta
	}

	pub(crate) fn formatted_balance(&self, balance: &str, decimals: usize) -> String {
		if balance == "0" {
			return balance.to_string()
		}

		let mut formatted = String::new();

		if balance.len() - 1 < decimals {
			let corrected_decimal = decimals - balance.len();

			formatted.push_str("0.");
			formatted.push_str("0".repeat(corrected_decimal).as_str());
			formatted.push_str(balance);
		} else {
			let corrected_decimal = balance.len() - decimals;
			let balance_decimal = &balance.to_string()[corrected_decimal..balance.len()];
			let has_decimal = !balance_decimal.replace('0', "").trim().is_empty();
			let mut corrected_balance = String::new();

			if has_decimal {
				corrected_balance.push('.');
				corrected_balance.push_str(balance_decimal);
			} else {
				corrected_balance.push('.');
			}

			formatted.push_str(&balance.to_string()[0..corrected_decimal]);
			formatted.push_str(corrected_balance.as_str());
		}

		formatted.trim_end_matches('0').trim_end_matches('.').to_string()
	}

	pub(crate) fn tip_log(
		&self,
		sender: &AccountId,
		amount: &str,
		symbol: &str,
		decimals: usize,
		reference: Option<String>,
	) {
		let formatted = self.formatted_balance(amount, decimals);
		if let Some(reference) = reference {
			log!("{} tipped {} {} to {}", sender, formatted, symbol, reference);
		} else {
			log!("{} received {} {}", sender, formatted, symbol);
		}
	}

	pub(crate) fn claim_reference_log(&self, reference_type: &str, reference_id: &str) {
		let reference = format!("{}/{}", reference_type, reference_id);

		log!("{} has been claimed", reference);
	}
}
