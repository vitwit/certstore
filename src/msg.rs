use crate::state::{CertDetails, User};
use cosmwasm::types::HumanAddr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum HandleMsg {
    AddInstitute {
        name: String,
        year: u128,
        website: String,
        email: String,
        location: String,
        wallet_address: HumanAddr,
    },
    StoreCertificate {
        name: String,
        join_date: String,
        end_date: String,
        domain: String,
        address: HumanAddr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum QueryMsg {
    GetCount {},
    GetUserCertificates { user: HumanAddr },
    GetCertificateByHash { hash: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserCertsResponse {
    pub certificates: User,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HashCertResponse {
    pub certificate: CertDetails,
}
