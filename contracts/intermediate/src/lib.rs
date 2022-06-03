use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env, near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};

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

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Message {
    title: String,
    body: String,
}

#[near_bindgen]
impl StatusMessage {
    pub fn set_status(&mut self, title: String, body: String) {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &Message { title, body });
    }

    pub fn get_status(&self, account_id: AccountId) -> Option<String> {
        self.records
            .get(&account_id)
            .map(|Message { body, title }| format!(r#"{{"body":"{body}", "title": "{title}"}}"#))
    }
}
