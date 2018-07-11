#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rust_indy_sdk as indy;
extern crate sovtoken;

mod utils;

use std::collections::HashMap;
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

#[test]
pub fn build_and_submit_nym_with_fees() {
    let payment_method = sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10])
    });
    let Setup {addresses, pool_handle, trustees, ..} = setup;
    let dids = trustees.dids();

    let (req, method) = indy::payments::Payment::build_get_utxo_request(wallet.handle, dids[0], &addresses[0]).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &req).unwrap();
    let res = indy::payments::Payment::parse_get_utxo_response(&method, &res).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    let value = utxos.get(0).unwrap().as_object().unwrap();
    let utxo = value.get("txo").unwrap().as_str().unwrap();

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": addresses[0],
        "amount": 9
    }]).to_string();

    let fees = json!({
        "1": 1
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let parsed_resp = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("paymentAddress").unwrap().as_str().unwrap(), addresses[0]);

    let get_nym_req = indy::ledger::Ledger::build_get_nym_request(dids[0], &did_new).unwrap();
    let get_nym_resp = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &get_nym_req).unwrap();
    let get_nym_resp_json: serde_json::Value = serde_json::from_str(&get_nym_resp).unwrap();
    assert!(get_nym_resp_json.as_object().unwrap().get("result").unwrap().as_object().unwrap().get("data").is_some());

    let fees = json!({
        "1": 0
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);
}

#[test]
pub fn build_and_submit_nym_with_fees_and_get_utxo() {
    let payment_method = sovtoken::utils::constants::general::PAYMENT_METHOD_NAME;
    let wallet = Wallet::new();
    let setup = Setup::new(&wallet, SetupConfig {
        num_addresses: 1,
        num_trustees: 4,
        num_users: 0,
        mint_tokens: Some(vec![10])
    });
    let Setup {addresses, pool_handle, trustees, ..} = setup;
    let dids = trustees.dids();

    let (req, method) = indy::payments::Payment::build_get_utxo_request(wallet.handle, dids[0], &addresses[0]).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &req).unwrap();
    let res = indy::payments::Payment::parse_get_utxo_response(&method, &res).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    let value = utxos.get(0).unwrap().as_object().unwrap();
    let utxo = value.get("txo").unwrap().as_str().unwrap();

    let inputs = json!([utxo]).to_string();
    let outputs = json!([{
        "paymentAddress": addresses[0],
        "amount": 9
    }]).to_string();

    let fees = json!({
        "1": 1
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    let (did_new, verkey_new) = indy::did::Did::new(wallet.handle, "{}").unwrap();

    let nym_req = indy::ledger::Ledger::build_nym_request(dids[0], &did_new,  Some(&verkey_new), None, None).unwrap();
    let nym_req_signed = indy::ledger::Ledger::sign_request(wallet.handle, dids[0], &nym_req).unwrap();
    let (nym_req_with_fees, pm) = indy::payments::Payment::add_request_fees(wallet.handle, dids[0], &nym_req_signed, &inputs, &outputs).unwrap();
    let nym_resp = indy::ledger::Ledger::submit_request(pool_handle, &nym_req_with_fees).unwrap();
    let parsed_resp = indy::payments::Payment::parse_response_with_fees(&pm, &nym_resp).unwrap();

    let parsed_resp_json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&parsed_resp).unwrap();
    assert_eq!(parsed_resp_json.len(), 1);
    assert_eq!(parsed_resp_json[0].get("amount").unwrap().as_u64().unwrap(), 9);
    assert_eq!(parsed_resp_json[0].get("paymentAddress").unwrap().as_str().unwrap(), addresses[0]);

    let fees = json!({
        "1": 0
    }).to_string();

    utils::fees::set_fees(pool_handle, wallet.handle, payment_method, &fees, &dids);

    let (req, method) = indy::payments::Payment::build_get_utxo_request(wallet.handle, dids[0], &addresses[0]).unwrap();
    let res = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, dids[0], &req).unwrap();
    let res = indy::payments::Payment::parse_get_utxo_response(&method, &res).unwrap();

    let res_parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    let utxos = res_parsed.as_array().unwrap();
    assert_eq!(utxos.len(), 1);
    let value = utxos.get(0).unwrap().as_object().unwrap();
    assert_eq!(value.get("paymentAddress").unwrap().as_str().unwrap(), addresses[0]);
    assert_eq!(value.get("amount").unwrap().as_u64().unwrap(), 9);
}