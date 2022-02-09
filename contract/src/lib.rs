
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Promise};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: i8,
}

#[near_bindgen]
impl Counter {
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    pub fn increment(&mut self, param: i8) {
        self.val += param;
        Promise::new(env::predecessor_account_id()).transfer(1_000_000_000_000_000_00);
        let log_message = format!("Increased number by {} to {}",param, self.val);
        env::log(log_message.as_bytes());
        //after_counter_change();
    }

    pub fn decrement(&mut self) {
        self.val -= 1;
        let log_message = format!("Decreased number to {}", self.val);
        env::log(log_message.as_bytes());
        //after_counter_change();
    }

    pub fn reset(&mut self) {
        self.val = 0;
        env::log(b"Reset counter to zero");
    }
}

fn after_counter_change() {
    env::log("Make sure you don't overflow, my friend.".as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;

    fn to_valid_account(account: &str) -> ValidAccountId {
        ValidAccountId::try_from(account.to_string()).expect("Invalid account")
    }

    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn increment() {

        let context = get_context(to_valid_account("foo.near"));
        testing_env!(context.build());

        let mut contract = Counter { val: 0 };
        contract.increment();
        println!("Value after increment: {}", contract.get_num());

        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = Counter { val: 0 };
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num());

        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = Counter { val: 0 };
        contract.increment();
        contract.reset();
        println!("Value after reset: {}", contract.get_num());

        assert_eq!(0, contract.get_num());
    }
}
