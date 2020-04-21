use rand::random;
use std::collections::HashMap;

use cosmwasm::errors::{contract_err, unauthorized, Result};
use cosmwasm::traits::{Api, Extern, Storage};
use cosmwasm::types::{Env, HumanAddr, Response};

use cw_storage::serialize;

use crate::msg::{CountResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, CertDetails, InstituteDetails, State, User};

pub fn init<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    env: Env,
    msg: InitMsg,
) -> Result<Response> {
    let state = State {
        users: HashMap::new(),
        institutes: HashMap::new(),
        hashes: HashMap::new(),
        owner: env.message.signer,
        count: 0,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(Response::default())
}

pub fn handle<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    env: Env,
    msg: HandleMsg,
) -> Result<Response> {
    match msg {
        HandleMsg::AddInstitute {
            name,
            year,
            website,
            email,
            location,
            wallet_address,
        } => add_institute(
            deps,
            env,
            name,
            year,
            website,
            email,
            location,
            wallet_address,
        ),
        HandleMsg::StoreCertificate {
            name,
            join_date,
            end_date,
            domain,
            address,
        } => store_certificate(deps, env, name, join_date, end_date, domain, address),
    }
}

pub fn add_institute<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    env: Env,
    name: String,
    year: u128,
    website: String,
    email: String,
    location: String,
    wallet_address: HumanAddr,
) -> Result<Response> {
    let api = deps.api;
    config(&mut deps.storage).update(&|mut state| {
        if env.message.signer != state.owner {
            unauthorized()
        } else {
            let institute_addr = api.canonical_address(&wallet_address)?;
            let institute = InstituteDetails {
                name: name,
                year: year,
                website: website,
                email: email,
                location: location,
                wallet_address: institute_addr,
            };
            state
                .institutes
                .insert(wallet_address.as_str().to_owned(), institute);
            Ok(state)
        }
    })?;

    Ok(Response::default())
}

pub fn store_certificate<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    env: Env,
    name: String,
    join_date: String,
    end_date: String,
    domain: String,
    address: HumanAddr,
) -> Result<Response> {
    let api = deps.api;
    config(&mut deps.storage).update(&|mut state| {
        let ins_human_addr = api.human_address(&env.message.signer)?;
        let user_address = api.canonical_address(&address)?;
        // Checks whether institute is registered or not
        if let Some(institute) = state
            .institutes
            .get_mut(&ins_human_addr.as_str().to_owned())
        {
            let hash = rand_string();
            let certificate = CertDetails {
                name: name,
                institute: institute.name,
                join_date: join_date,
                end_date: end_date,
                domain: domain,
                hash: hash,
                user_address: user_address,
                institute_address: institute.wallet_address,
            };
            let address_str: String = address.as_str().to_owned();
            if let Some(user) = state.users.get_mut(&address_str) {
                user.count += 1;
                user.details.insert(hash, certificate);
            } else {
                let mut user = User {
                    count: 1,
                    details: HashMap::new(),
                };
                user.details.insert(hash, certificate);
                state.users.insert(address_str, user);
            }
            state.hashes.insert(hash, certificate);
            state.count += 1;
            Ok(state)
        } else {
            contract_err("No institute registered with signed address")
        }
    })?;
    Ok(Response::default())
}

fn rand_string() -> String {
    (0..4)
        .map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char)
        .collect()
}

pub fn query<S: Storage, A: Api>(deps: &Extern<S, A>, msg: QueryMsg) -> Result<Vec<u8>> {
    match msg {
        QueryMsg::GetCount {} => query_count(deps),
        QueryMsg::GetCertificateByHash { hash } => query_certificate_by_hash(deps, hash),
        QueryMsg::GetUserCertificates { user } => query_certificates_by_user(deps, user),
    }
}

fn query_count<S: Storage, A: Api>(deps: &Extern<S, A>) -> Result<Vec<u8>> {
    let state = config_read(&deps.storage).load()?;

    let resp = CountResponse { count: state.count };

    let out = serialize(&resp)?;

    Ok(out)
}

fn query_certificate_by_hash<S: Storage, A: Api>(
    deps: &Extern<S, A>,
    hash: String,
) -> Result<Vec<u8>> {
    let mut state = config_read(&deps.storage).load()?;

    let resp = state.hashes.get_mut(&hash).unwrap();
    let out = serialize(&resp)?;

    Ok(out)
}

fn query_certificates_by_user<S: Storage, A: Api>(
    deps: &Extern<S, A>,
    user: HumanAddr,
) -> Result<Vec<u8>> {
    let mut state = config_read(&deps.storage).load()?;

    let user_str: String = user.as_str().to_owned();

    let resp = state.users.get_mut(&user_str).unwrap();
    let out = serialize(&resp)?;

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm::mock::{dependencies, mock_env};
    use cosmwasm::serde::from_slice;
    use cosmwasm::types::coin;

    #[test]
    fn proper_initialization() {
        let mut deps = dependencies(20);

        let msg = InitMsg {};
        let env = mock_env(&deps.api, "creator", &coin("1000", "earth"), &[]);

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_slice(&res).unwrap();
        assert_eq!(0, value.count);
    }
}
