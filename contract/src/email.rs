use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}, AccountId, serde::{Serialize, Deserialize}};
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Email {
    pub email_id: u128,
    pub title: String,
    pub content:String,
    pub owner:AccountId,
    pub receiver: AccountId,
    pub time:u64
}
