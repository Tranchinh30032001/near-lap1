// Nội dung chương trình: Tạo 1 smartcontract dùng để gửi email (mỗi lần gửi thì sẽ phải trả 1 near tiền phí). Rồi tạo 2 UnorderMap cho người gửi và người nhận chứa thông tin email. Tạo Vector chứa tất các các email đã tạo ra. xem số dư của SC, địa chỉ của SC...

use email::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise};
mod email;

pub type EmailId = u128;

#[derive(BorshStorageKey, BorshSerialize)]
// #[derive(Serialize, Deserialize)]
enum Identify {
    Sender,
    Receiver,
    VecSender,
    VecReceiver,
    VecEmail,
}
const FEE_SEND: U128 = U128(1); // tiền phí mỗi lần send_email.

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]

pub struct Contract {
    sennders: UnorderedMap<AccountId, Vector<EmailId>>,
    receivers: UnorderedMap<AccountId, Vector<EmailId>>,
    count_email: u128,         // số lượng email
    list_email: Vector<Email>, // danh sách tất cả các email đã được gửi
    owner: AccountId,
}
impl Default for Contract {
    fn default() -> Self {
        Self {
            sennders: UnorderedMap::new(Identify::Sender),
            receivers: UnorderedMap::new(Identify::Receiver),
            list_email: Vector::new(Identify::VecEmail),
            count_email: 0,
            owner: env::predecessor_account_id(),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn send_email(&mut self, receiver: AccountId, title: String, content: String) {
        let init_storage = env::storage_usage();
        log!("init_Storage: {}", init_storage);
        println!("helo");
        Promise::new(env::current_account_id()).transfer(FEE_SEND.into());
        let count_id = self.count_email;
        // đây là trường hợp user (sender) này đã gửi email trước đó
        if let Some(mut result) = self.sennders.get(&env::predecessor_account_id()) {
            let new_email = Email {
                email_id: count_id,
                title,
                content,
                owner: env::predecessor_account_id(),
                receiver: receiver.clone(),
                time: env::block_timestamp(),
            };
            result.push(&new_email.email_id);
            self.list_email.push(&new_email);

            self.sennders
                .insert(&env::predecessor_account_id(), &result);
            let storage_final = env::storage_usage();
            let chenhlech = storage_final - init_storage;
            let price = u128::checked_mul(chenhlech.into(), env::storage_byte_cost());
            log!("storage: {}", storage_final);
            log!("price: {}", price.unwrap())
        }
        // đây là lần đầu (sender) gửi email
        else {
            let mut sender_vector: Vector<EmailId> = Vector::new(Identify::VecSender);
            let s1 = env::storage_usage();
            log!("s1: {}", s1);
            sender_vector.push(&count_id);
            let s2 = env::storage_usage();
            log!("s2: {}", s2);
            self.sennders
                .insert(&env::predecessor_account_id(), &sender_vector);
            let s3 = env::storage_usage();
            log!("s3: {}", s3);
        }
        // đây là trường hợp user (receiver) này đã nhận email trước đó
        if let Some(mut result) = self.receivers.get(&receiver) {
            result.push(&count_id);
            self.receivers.insert(&receiver, &result);
        }
        // đây là lần đầu (receiver) nhận email
        else {
            let mut receiver_vector: Vector<EmailId> = Vector::new(Identify::VecReceiver);
            receiver_vector.push(&count_id);
            self.receivers.insert(&receiver, &receiver_vector);
        }
        log!("id: {}", count_id);
        self.count_email += 1;
    }
    pub fn get_content_email(&self, email_id: U128) -> Option<String> {
        let email_id = email_id.0;
        for email in self.list_email.iter() {
            if email.email_id == email_id {
                return Some(email.content.clone());
            }
        }
        return None;
    }
    pub fn get_owner(&self, email_id: U128) -> Option<AccountId> {
        let email_id = email_id.0;
        for email in self.list_email.iter() {
            if email.email_id == email_id {
                return Some(email.owner.clone());
            }
        }
        return None;
    }
    pub fn get_receiver(&self, email_id: U128) -> Option<AccountId> {
        let email_id = email_id.0;
        for email in self.list_email.iter() {
            if email.email_id == email_id {
                return Some(email.receiver.clone());
            }
        }
        return None;
    }

    pub fn get_all_email_sender(&self, account_id: AccountId) -> Vec<EmailId> {
        // Làm sao kiểu tra đó là 1 account_id hợp lệ,
        return self.sennders.get(&account_id).unwrap().to_vec();
    }

    pub fn get_list_email(&self) -> Option<Vec<Email>> {
        if self.list_email.is_empty() {
            return None;
        }
        return Some(self.list_email.to_vec());
    }

    pub fn get_balance_contract(&self) -> u128 {
        return env::account_balance(); // cái này là trả về số dư tài khoản à
                                       // View call: dev-1677665256620-20363563376214.get_balance_contract()
                                       // 1.999991875430676e+26
    }

    pub fn get_address_contract(&self) -> AccountId {
        return self.get_address();
    }

    pub fn get_address(&self) -> AccountId {
        log!("owner: {}", self.owner);
        return env::signer_account_id();
    }

    pub fn get_count(&self) -> u128 {
        return self.count_email;
    }
}

