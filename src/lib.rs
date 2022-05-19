use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::Deserialize;
use near_sdk::{env, near_bindgen, AccountId, serde::{Serialize}};
use witgen::witgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StatusMessage {
    records: LookupMap<AccountId, Message>,
}

/// A simple message with a title
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[witgen]
pub struct Message {
    /// Title that describes the message
    title: String,
    /// body of the  message
    body: String,
}

impl Default for StatusMessage {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"r"),
        }
    }
}

#[near_bindgen]
impl StatusMessage {
    /// Set the status message for the current logged in user
    pub fn set_status_message(&mut self, message: Message) {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
    }

    /// Get the status message for a given account id
    pub fn get_status_message(&self, account_id: AccountId) -> Option<Message> {
        self.records.get(&account_id)
    }
}
