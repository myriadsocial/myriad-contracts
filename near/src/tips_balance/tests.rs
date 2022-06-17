#[cfg(all(test, not(target_arch = "wasm32")))]
mod tips_balance_tests {
	use crate::*;
	use near_sdk::{
		test_utils::{accounts, VMContextBuilder},
		testing_env,
	};

	fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
		let mut builder = VMContextBuilder::new();
		builder
			.current_account_id(accounts(0))
			.signer_account_id(predecessor_account_id.clone())
			.predecessor_account_id(predecessor_account_id);

		builder
	}

	#[test]
	fn send_tip_works() {
		// Initialize contract
		let mut context = get_context(accounts(1));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new("myriad", "user", "user_id", "native");
		let tip: Balance = 100000000000000000000000; // 0.1 NEAR

		// Send tip to user-user_id
		testing_env!(context.predecessor_account_id(accounts(3)).attached_deposit(tip).build());
		contract.send_tip(tips_balance_info.clone());

		// Test
		let expected_tips_balance = TipsBalance::new(&tips_balance_info).set_balance(tip);
		let tips_balance = contract.get_tips_balance(tips_balance_info).unwrap();
		let formatted_tips_balance = TipsBalanceWithFormattedBalance {
			tips_balance: expected_tips_balance,
			symbol: String::from("NEAR"),
			formatted_amount: String::from("0.1"),
		};

		assert_eq!(formatted_tips_balance, tips_balance);
	}

	#[test]
	fn claim_reference_works() {
		// Initialize contract
		let mut context = get_context(accounts(0));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info_1 = TipsBalanceInfo::new("myriad", "people", "people_id", "native");
		let tip: Balance = 100000000000000000000000; // 0.1 NEAR

		// Send tip to user-user_id
		testing_env!(context.predecessor_account_id(accounts(3)).attached_deposit(tip).build());
		contract.send_tip(tips_balance_info_1.clone());

		// Payload
		let reference_type = String::from("user");
		let reference_id = String::from("user_id");
		let tips_balance_info_2 = TipsBalanceInfo::new("myriad", "user", "user_id", "native");

		// Claim reference
		testing_env!(context.predecessor_account_id(accounts(0)).attached_deposit(1).build());
		contract.claim_reference(
			tips_balance_info_1.clone(),
			reference_type,
			reference_id,
			accounts(5),
		);

		// Test
		let tips_balance = contract.get_tips_balance(tips_balance_info_1);
		assert_eq!(tips_balance, None);
		let tips_balance = TipsBalance::new(&tips_balance_info_2).set_balance(tip);

		let expected_tips_balance = tips_balance.set_account_id(&accounts(5));
		let tips_balance = contract.get_tips_balance(tips_balance_info_2).unwrap();
		let formatted_tips_balance = TipsBalanceWithFormattedBalance {
			tips_balance: expected_tips_balance,
			symbol: String::from("NEAR"),
			formatted_amount: String::from("0.1"),
		};

		assert_eq!(formatted_tips_balance, tips_balance);
	}

	#[test]
	fn claim_tips_works() {
		// Initialize contract
		let mut context = get_context(accounts(0));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info_1 = TipsBalanceInfo::new("myriad", "people", "people_id", "native");
		let tip: Balance = 100000000000000000000000; // 0.1 NEAR

		// Send tip to user-user_id
		testing_env!(context.predecessor_account_id(accounts(3)).attached_deposit(tip).build());
		contract.send_tip(tips_balance_info_1.clone());

		// Payload
		let reference_type = String::from("user");
		let reference_id = String::from("user_id");
		let tips_balance_info_2 = TipsBalanceInfo::new("myriad", "user", "user_id", "native");

		// Claim reference
		testing_env!(context.signer_account_id(accounts(0)).attached_deposit(1).build());
		contract.claim_reference(
			tips_balance_info_1.clone(),
			reference_type,
			reference_id,
			accounts(4),
		);

		// Claim tip
		testing_env!(context.signer_account_id(accounts(4)).attached_deposit(1).build());
		contract.claim_tip(tips_balance_info_2.clone());

		// Test
		let tips_balance = contract.get_tips_balance(tips_balance_info_1);
		assert_eq!(tips_balance, None);
		let tips_balance = TipsBalance::new(&tips_balance_info_2);

		let expected_tips_balance = tips_balance.set_account_id(&accounts(4));
		let tips_balance = contract.get_tips_balance(tips_balance_info_2).unwrap();
		let formatted_tips_balance = TipsBalanceWithFormattedBalance {
			tips_balance: expected_tips_balance,
			symbol: String::from("NEAR"),
			formatted_amount: String::from("0"),
		};

		assert_eq!(formatted_tips_balance, tips_balance);
	}

	#[test]
	#[should_panic(expected = "OnlyForNear")]
	fn cant_send_tip_when_tipping_with_fungible_token() {
		// Initialize contract
		let mut context = get_context(accounts(1));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new("myriad", "user", "user_id", "fungible_token");

		// Send tip to user-user_id
		testing_env!(context.predecessor_account_id(accounts(3)).attached_deposit(0).build());
		contract.send_tip(tips_balance_info);
	}

	#[test]
	#[should_panic(expected = "TipsMustMoreThanMinimum")]
	fn cant_send_tip_when_tipping_amount_less_than_minimum() {
		// Initialize contract
		let mut context = get_context(accounts(1));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new("myriad", "user", "user_id", "native");

		// Send tip to user-user_id
		testing_env!(context.predecessor_account_id(accounts(3)).attached_deposit(0).build());
		contract.send_tip(tips_balance_info);
	}

	#[test]
	#[should_panic(expected = "InsufficientBalance")]
	fn cant_send_tip_when_insufficient_balance() {
		// Initialize contract
		let mut context = get_context(accounts(1));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new("myriad", "user", "user_id", "native");

		// Send tip to user-user_id
		testing_env!(context
			.predecessor_account_id(accounts(3))
			.account_balance(0)
			.attached_deposit(10)
			.build());
		contract.send_tip(tips_balance_info);
	}

	#[test]
	#[should_panic(expected = "Requires attached deposit of exactly 1 yoctoNEAR")]
	fn cant_claim_reference_without_min_attached_deposit() {
		// Initialize contract
		let mut context = get_context(accounts(1));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new("myriad", "people", "people_id", "native");
		let tip: Balance = 100000000000000000000000; // 0.1 NEAR

		// Send tip to user-user_id
		testing_env!(context.predecessor_account_id(accounts(3)).attached_deposit(tip).build());
		contract.send_tip(tips_balance_info.clone());

		// Payload
		let reference_type = String::from("user");
		let reference_id = String::from("user_id");

		// Claim reference
		contract.claim_reference(tips_balance_info, reference_type, reference_id, accounts(5));
	}

	#[test]
	#[should_panic(expected = "Unauthorized")]
	fn cant_claim_reference_when_unauthorized() {
		// Initialize contract
		let mut context = get_context(accounts(1));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new("myriad", "people", "people_id", "native");
		let tip: Balance = 100000000000000000000000; // 0.1 NEAR

		// Send tip to user-user_id
		testing_env!(context.predecessor_account_id(accounts(3)).attached_deposit(tip).build());
		contract.send_tip(tips_balance_info.clone());

		// Payload
		let reference_type = String::from("user");
		let reference_id = String::from("user_id");

		// Claim reference
		testing_env!(context.signer_account_id(accounts(3)).attached_deposit(1).build());
		contract.claim_reference(tips_balance_info, reference_type, reference_id, accounts(5));
	}

	#[test]
	#[should_panic(expected = "WrongFormat")]
	fn cant_claim_reference_when_reference_id_not_same() {
		// Initialize contract
		let mut context = get_context(accounts(0));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new("myriad", "user", "user_id", "native");
		let tip: Balance = 100000000000000000000000; // 0.1 NEAR

		// Send tip to user-user_id
		testing_env!(context.predecessor_account_id(accounts(3)).attached_deposit(tip).build());
		contract.send_tip(tips_balance_info.clone());

		// Payload
		let reference_type = String::from("user");
		let reference_id = String::from("user_idd");

		// Claim reference
		testing_env!(context.predecessor_account_id(accounts(0)).attached_deposit(1).build());
		contract.claim_reference(tips_balance_info, reference_type, reference_id, accounts(4));
	}

	#[test]
	#[should_panic(expected = "NothingToClaimed")]
	fn cant_claim_tips_when_nothing_to_claimed() {
		// Initialize contract
		let mut context = get_context(accounts(0));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let reference_type = String::from("user");
		let reference_id = String::from("user_id");
		let tips_balance_info = TipsBalanceInfo::new("myriad", "user", "user_id", "native");

		// Claim reference
		testing_env!(context.predecessor_account_id(accounts(0)).attached_deposit(1).build());
		contract.claim_reference(
			tips_balance_info.clone(),
			reference_type,
			reference_id,
			accounts(4),
		);

		// Claim tip
		testing_env!(context.predecessor_account_id(accounts(4)).attached_deposit(1).build());
		contract.claim_tip(tips_balance_info);
	}

	#[test]
	#[should_panic(expected = "ReceiverNotExists")]
	fn cant_claim_tips_when_receiver_not_exists() {
		// Initialize contract
		let mut context = get_context(accounts(0));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info_1 = TipsBalanceInfo::new("myriad", "user", "user_id", "native");
		let tip: Balance = 100000000000000000000000; // 0.1 NEAR

		// Send tip to user-user_id
		testing_env!(context.predecessor_account_id(accounts(3)).attached_deposit(tip).build());
		contract.send_tip(tips_balance_info_1.clone());

		// Claim tip
		testing_env!(context.predecessor_account_id(accounts(4)).attached_deposit(1).build());
		contract.claim_tip(tips_balance_info_1);
	}

	#[test]
	#[should_panic(expected = "Unauthorized")]
	fn cant_claim_tips_when_unauthorized() {
		// Initialize contract
		let mut context = get_context(accounts(0));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new("myriad", "user", "user_id", "native");
		let tip: Balance = 100000000000000000000000; // 0.1 NEAR

		// Send tip to user-user_id
		testing_env!(context.predecessor_account_id(accounts(3)).attached_deposit(tip).build());
		contract.send_tip(tips_balance_info.clone());

		// Payload
		let reference_type = String::from("user");
		let reference_id = String::from("user_id");

		// Claim reference
		testing_env!(context.predecessor_account_id(accounts(0)).attached_deposit(1).build());
		contract.claim_reference(
			tips_balance_info.clone(),
			reference_type,
			reference_id,
			accounts(4),
		);

		// Claim tip
		testing_env!(context.predecessor_account_id(accounts(5)).attached_deposit(1).build());
		contract.claim_tip(tips_balance_info);
	}

	#[test]
	#[should_panic(expected = "TipsBalanceNotExists")]
	fn cant_claim_tips_when_data_not_exists() {
		// Initialize contract
		let mut context = get_context(accounts(1));
		testing_env!(context.build());
		let mut contract = TippingContract::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new("myriad", "user", "user_id", "native");

		// Claim tip
		testing_env!(context.predecessor_account_id(accounts(5)).attached_deposit(1).build());
		contract.claim_tip(tips_balance_info);
	}
}
