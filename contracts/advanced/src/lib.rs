use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env, near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};
use witgen::witgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StatusMessage {
    records: LookupMap<AccountId, Message>,
}

impl Default for StatusMessage {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"r"),
        }
    }
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

#[near_bindgen]
impl StatusMessage {
    pub fn set_status(&mut self, message: Message) {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
    }

    pub fn get_status(&self, account_id: AccountId) -> Option<Message> {
        self.records.get(&account_id)
    }
}
