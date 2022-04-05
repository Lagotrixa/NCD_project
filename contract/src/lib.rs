use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,Promise};

// 1 NEAR
const AMOUNT: u128 = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    key: String,
}

#[near_bindgen]
impl Contract {
    // INITIALIZE CONTRACT, PREVENTS CHANGES TO THE CONTRACT
    #[init]
    pub fn wallet(_key: String) -> Self {
            Self{key: _key}
    }

    // ADD CONTRACT METHODS HERE
    #[payable]
    pub fn get_money(&mut self, _key: String) -> bool {
        let hashed_input = env::sha256(_key.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        if hashed_input_hex == self.key {
            Promise::new(env::predecessor_account_id()).transfer(AMOUNT);
            env::log_str("Key is correct. Paid!");
            return true;
        } else {
            env::log_str("Key is wrong!");
            return false;
        }
    }

}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn check_get_money() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = Counter { val: 0 };

        let mut guess_result = contract.get_money("WrongKey".to_string());
        assert!(!(guess_result), "Expected a failure from the WrongKey");
        assert_eq!(get_logs(), ["Key is wrong!"], "Expected a failure log.");

        guess_result = contract.get_money("CorrectKey".to_string());
        assert!(guess_result, "Expected the CorrectKey to return true.");
        assert_eq!(get_logs(), ["Key is correct. Paid!"], "Expected a successful log.");
    }
}
