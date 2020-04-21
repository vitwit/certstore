use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use cosmwasm::traits::Storage;
use cosmwasm::types::CanonicalAddr;
use cw_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub users: HashMap<String, User>,
    pub hashes: HashMap<String, CertDetails>,
    pub institutes: HashMap<String, InstituteDetails>,
    pub owner: CanonicalAddr,
    pub count: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct User {
    pub count: u128,
    pub details: HashMap<String, CertDetails>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CertDetails {
    pub name: String,
    pub institute: String,
    pub join_date: String,
    pub end_date: String,
    pub domain: String,
    pub hash: String,
    pub user_address: CanonicalAddr,
    pub institute_address: CanonicalAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstituteDetails {
    pub name: String,
    pub year: u128,
    pub website: String,
    pub email: String,
    pub location: String,
    pub wallet_address: CanonicalAddr,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
