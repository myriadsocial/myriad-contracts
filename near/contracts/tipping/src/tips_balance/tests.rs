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
		let mut contract = Tipping::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new(&accounts(4), "user", "user_id", "native");
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
			unclaimed_reference_ids: Vec::new(),
		};

		assert_eq!(formatted_tips_balance, tips_balance);
	}

	#[test]
	#[should_panic(expected = "OnlyForNear")]
	fn cant_send_tip_when_tipping_with_fungible_token() {
		// Initialize contract
		let mut context = get_context(accounts(1));
		testing_env!(context.build());
		let mut contract = Tipping::new(None);

		// Payload
		let tips_balance_info =
			TipsBalanceInfo::new(&accounts(4), "user", "user_id", "fungible_token");

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
		let mut contract = Tipping::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new(&accounts(4), "user", "user_id", "native");

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
		let mut contract = Tipping::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new(&accounts(4), "user", "user_id", "native");

		// Send tip to user-user_id
		testing_env!(context
			.predecessor_account_id(accounts(3))
			.account_balance(0)
			.attached_deposit(10)
			.build());
		contract.send_tip(tips_balance_info);
	}

	#[test]
	#[should_panic(expected = "ReceiverNotExists")]
	fn cant_claim_tips_when_receiver_not_exists() {
		// Initialize contract
		let mut context = get_context(accounts(0));
		testing_env!(context.build());
		let mut contract = Tipping::new(None);

		// Payload
		let tips_balance_info_1 = TipsBalanceInfo::new(&accounts(4), "user", "user_id", "native");
		let tip: Balance = 100000000000000000000000; // 0.1 NEAR

		// Send tip to user-user_id
		testing_env!(context.predecessor_account_id(accounts(3)).attached_deposit(tip).build());
		contract.send_tip(tips_balance_info_1.clone());

		// Claim tip
		testing_env!(context.predecessor_account_id(accounts(4)).attached_deposit(1).build());
		contract.claim_tip(tips_balance_info_1);
	}

	#[test]
	#[should_panic(expected = "TipsBalanceNotExists")]
	fn cant_claim_tips_when_data_not_exists() {
		// Initialize contract
		let mut context = get_context(accounts(1));
		testing_env!(context.build());
		let mut contract = Tipping::new(None);

		// Payload
		let tips_balance_info = TipsBalanceInfo::new(&accounts(4), "user", "user_id", "native");

		// Claim tip
		testing_env!(context.predecessor_account_id(accounts(5)).attached_deposit(1).build());
		contract.claim_tip(tips_balance_info);
	}
}
