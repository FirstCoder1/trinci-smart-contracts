// This file is part of TRINCI.
//
// Copyright (C) 2021 Affidaty Spa.
//
// TRINCI is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the
// Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// TRINCI is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License
// for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with TRINCI. If not, see <https://www.gnu.org/licenses/>.

<<<<<<< HEAD
//! Crypto integration tests
use integration::common::{SerdeValue, PUB_KEY2, PVT_KEY2};
use integration::{
    common::{self, AccountInfo, PUB_KEY1, PVT_KEY1},
    TestApp,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use trinci_sdk::tai::AssetTransferArgs;
use trinci_sdk::value;

use std::collections::HashMap;
use trinci_core::{base::serialize::rmp_deserialize, crypto::Hash, Receipt, Transaction};

lazy_static! {
    pub static ref TEST_APP_HASH: Hash = common::app_hash("test.wasm").unwrap();
}

const CALLER_ALIAS: &str = "Owner";
const DESTINATION_ALIAS: &str = "Destination";
=======
//! Test contract integration tests
use integration::{
    common::{self, *},
    TestApp,
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use trinci_core::{base::serialize::rmp_deserialize, crypto::Hash};
use trinci_core::{Receipt, Transaction};

const TEST_ALIAS: &str = "FCK";
>>>>>>> 2265110 (added test contract integration test)

lazy_static! {
    static ref ACCOUNTS_INFO: HashMap<&'static str, AccountInfo> = {
        let mut map = HashMap::new();
<<<<<<< HEAD
        map.insert(CALLER_ALIAS, AccountInfo::new(PUB_KEY1, PVT_KEY1));
        map.insert(DESTINATION_ALIAS, AccountInfo::new(PUB_KEY2, PVT_KEY2));
=======
        map.insert(TEST_ALIAS, AccountInfo::new(PUB_KEY1, PVT_KEY1));
>>>>>>> 2265110 (added test contract integration test)
        map
    };
}

<<<<<<< HEAD
fn echo_generic_tx(account_info: &AccountInfo) -> Transaction {
    let args = value! ({
        "test": true,
        "value": 123,
    });

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "echo_generic",
=======
lazy_static! {
    pub static ref TEST_APP_HASH: Hash = app_hash("test.wasm").unwrap();
}

fn store_data_tx(test_info: &AccountInfo, key: &str, data: &[u8]) -> Transaction {
    let args = value!({
        "key": key,
        "data": data,
    });
    common::create_test_tx(
        &test_info.id,
        &test_info.pub_key,
        &test_info.pvt_key,
        *TEST_APP_HASH,
        "store_data",
>>>>>>> 2265110 (added test contract integration test)
        args,
    )
}

<<<<<<< HEAD
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq, Clone, Default))]
struct SubStruct<'a> {
    pub field1: u32,
    pub field2: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq, Clone, Default))]
struct EchoArgs<'a> {
    pub name: &'a str,
    pub surname: String,
    #[serde(with = "serde_bytes")]
    pub buf: Vec<u8>,
    pub vec8: Vec<u8>,
    pub vec16: Vec<u16>,
    pub map: HashMap<&'a str, SubStruct<'a>>,
}

fn create_echo_typed_args() -> EchoArgs<'static> {
    let mut map = HashMap::<&str, SubStruct>::new();

    map.insert(
        "a",
        SubStruct {
            field1: 42,
            field2: "skynet",
        },
    );

    EchoArgs {
        name: "John",
        surname: "Doe".to_string(),
        buf: vec![7, 11, 13],
        vec8: vec![1, 2, 5],
        vec16: vec![23, 37, 43],
        map,
    }
}
fn echo_typed_tx(account_info: &AccountInfo) -> Transaction {
    let args = create_echo_typed_args();

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "echo_typed",
        args,
    )
}

fn echo_packed_tx(account_info: &AccountInfo) -> Transaction {
    let args = "hello".as_bytes().to_vec();

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "echo_packed",
        args,
    )
}

fn nested_call_tx(account_info: &AccountInfo) -> Transaction {
    let args = value!({"greet": "hello!"});

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "nested_call",
        args,
    )
}

fn mint_tx(account_info: &AccountInfo) -> Transaction {
    let args: u64 = 50;

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "mint",
        args,
    )
}

fn balance_tx(account_info: &AccountInfo) -> Transaction {
    let args = value!({});

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "balance",
        args,
    )
}
fn divide_by_zero_tx(account_info: &AccountInfo) -> Transaction {
    let args = value!({"zero":0u64});

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "divide_by_zero",
        args,
    )
}

fn trigger_panic_tx(account_info: &AccountInfo) -> Transaction {
    let args = value!({});

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "trigger_panic",
        args,
    )
}

fn exhaust_memory_tx(account_info: &AccountInfo) -> Transaction {
    let args = value!({});

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "exhaust_memory",
        args,
    )
}

fn infinite_recursion_tx(account_info: &AccountInfo) -> Transaction {
    let args = value!(false);

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "infinite_recursion",
        args,
    )
}

fn infinite_loop_tx(account_info: &AccountInfo) -> Transaction {
    let args = value!(false);

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "infinite_loop",
        args,
    )
}

fn null_pointer_indirection_tx(account_info: &AccountInfo) -> Transaction {
    let args = value!(false);

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "null_pointer_indirection",
        args,
    )
}

fn random_sequence_tx(account_info: &AccountInfo) -> Transaction {
    let args = value!({});

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "random_sequence",
        args,
    )
}

fn return_hashmap_tx(account_info: &AccountInfo) -> Transaction {
    let args = value!({});

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "return_hashmap",
        args,
    )
}

fn get_time_tx(account_info: &AccountInfo) -> Transaction {
    let args = value!({});

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "get_time",
        args,
    )
}

fn transfer_tx(
    account_info: &AccountInfo,
    destination_info: &AccountInfo,
    units: u64,
) -> Transaction {
    let args = AssetTransferArgs {
        from: &account_info.id,
        to: &destination_info.id,
        units,
    };

    common::create_test_tx(
        &account_info.id,
        &account_info.pub_key,
        &account_info.pvt_key,
        *TEST_APP_HASH,
        "transfer",
=======
fn get_account_keys_tx(test_info: &AccountInfo, pattern: &str) -> Transaction {
    let args = pattern;
    common::create_test_tx(
        &test_info.id,
        &test_info.pub_key,
        &test_info.pvt_key,
        *TEST_APP_HASH,
        "get_account_keys",
>>>>>>> 2265110 (added test contract integration test)
        args,
    )
}

fn get_account_contract(test_info: &AccountInfo, account_id: &str) -> Transaction {
    let args = account_id;
    common::create_test_tx(
        &test_info.id,
        &test_info.pub_key,
        &test_info.pvt_key,
        *TEST_APP_HASH,
        "test_get_account_contract",
        args,
    )
}

fn create_txs() -> Vec<Transaction> {
<<<<<<< HEAD
    let caller_info = ACCOUNTS_INFO.get(CALLER_ALIAS).unwrap();
    let destination_info = ACCOUNTS_INFO.get(DESTINATION_ALIAS).unwrap();

    vec![
        // Input and output serialization.
        // 0. echo generic
        echo_generic_tx(caller_info),
        // 1. echo typed
        echo_typed_tx(caller_info),
        // 2. echo packed
        echo_packed_tx(caller_info),
        // Default smart contract functionalities.
        // 3. nested call
        nested_call_tx(caller_info),
        // 4. mint
        mint_tx(caller_info),
        // 5. balance
        balance_tx(caller_info),
        // 6. transfer
        transfer_tx(caller_info, destination_info, 23),
        // 7. balance
        balance_tx(caller_info),
        // Trigger exceptional conditions.
        // 8. divide by zero
        divide_by_zero_tx(caller_info),
        // 9. trigger_panic,
        trigger_panic_tx(caller_info),
        // 10. exhaust_memory,
        exhaust_memory_tx(caller_info),
        // 11. infinite_recursion,
        infinite_recursion_tx(caller_info),
        // 12. infinite_loop,
        infinite_loop_tx(caller_info),
        // 13. null_pointer_indirection,
        null_pointer_indirection_tx(caller_info),
        // Deterministic contract
        // 14. random_sequence,
        random_sequence_tx(caller_info),
        // 15. random_sequence,
        random_sequence_tx(caller_info),
        // 16. return_hashmap,
        return_hashmap_tx(caller_info),
        // 17. get_time,
        get_time_tx(caller_info),
    ]
}

fn check_rxs(rxs: Vec<Receipt>) {
    // 0. echo generic
    assert!(rxs[0].success);

    let res: SerdeValue = rmp_deserialize(&rxs[0].returns).unwrap();

    assert!(res.get(&value!("test")).unwrap().as_bool().unwrap());
    assert_eq!(res.get(&value!("value")).unwrap().as_i64().unwrap(), 123);

    // 1. echo typed
    assert!(rxs[1].success);

    let res: EchoArgs = trinci_sdk::rmp_deserialize(&rxs[1].returns).unwrap();

    assert_eq!(res, create_echo_typed_args());

    // 2. echo packed
    assert!(rxs[2].success);

    let res: Vec<u8> = trinci_sdk::rmp_deserialize(&rxs[2].returns).unwrap();

    assert_eq!("hello".as_bytes().to_vec(), res);

    // 3. nested call
    assert!(rxs[3].success);
    let res: SerdeValue = trinci_sdk::rmp_deserialize(&rxs[3].returns).unwrap();

    let value = res.get(&value!("greet")).unwrap().as_str().unwrap();
    assert_eq!(value, "hello!");

    // 4. mint
    assert!(rxs[4].success);

    // 5. balance
    assert!(rxs[5].success);

    let balance: u64 = trinci_sdk::rmp_deserialize(&rxs[5].returns).unwrap();
    assert_eq!(balance, 50);

    // 6. transfer
    assert!(rxs[6].success);

    // 7. balance
    assert!(rxs[7].success);

    let balance: u64 = trinci_sdk::rmp_deserialize(&rxs[7].returns).unwrap();
    assert_eq!(balance, 27);

    // 8. divide_by_zero
    assert!(!rxs[8].success);

    assert_eq!(
        "wasm machine fault",
        String::from_utf8_lossy(&rxs[8].returns)
    );

    // 9. trigger_panic
    assert!(!rxs[9].success);

    assert_eq!(
        "wasm machine fault",
        String::from_utf8_lossy(&rxs[9].returns)
    );

    // 10. exhaust_memory
    assert!(!rxs[10].success);

    assert_eq!(
        "wasm machine fault",
        String::from_utf8_lossy(&rxs[10].returns)
    );

    // 11. infinite recursion
    assert!(!rxs[11].success);

    assert_eq!(
        "wasm machine fault",
        String::from_utf8_lossy(&rxs[11].returns)
    );

    // 12. infinite_loop
    assert!(!rxs[12].success);

    assert_eq!(
        "wasm machine fault",
        String::from_utf8_lossy(&rxs[12].returns)
    );

    // 13. null_pointer_indirection
    assert!(!rxs[13].success);

    assert_eq!(
        "wasm machine fault",
        String::from_utf8_lossy(&rxs[13].returns)
    );
    // 14. random_sequence,
    assert!(rxs[14].success);

    assert_eq!(
        &rxs[14].returns,
        &vec![147, 206, 21, 0, 11, 52, 206, 76, 128, 38, 222, 207, 0, 10, 128, 0, 76, 130, 188, 60]
    );
    // 15. random_sequence,
    assert!(rxs[15].success);

    assert_eq!(
        &rxs[15].returns,
        &vec![147, 206, 21, 0, 11, 52, 206, 76, 128, 38, 222, 207, 0, 10, 128, 0, 76, 130, 188, 60]
    );
    // 16. return_hashmap,
    assert!(rxs[16].success);
    assert_eq!(
        &rxs[16].returns,
        &vec![
            131, 164, 118, 97, 108, 49, 123, 164, 118, 97, 108, 50, 205, 1, 200, 164, 118, 97, 108,
            51, 205, 3, 21
        ]
    );

    // 17. get_time,
    assert!(!rxs[17].success);

    assert_eq!(
        "wasm machine fault",
        String::from_utf8_lossy(&rxs[17].returns)
    );
=======
    let test_info = ACCOUNTS_INFO.get(TEST_ALIAS).unwrap();

    vec![
        // 0. Get keys with empty pattern. This shall fail
        get_account_keys_tx(test_info, ""),
        // 1. Get keys with wildcard pattern. This shall return an empty Vec
        get_account_keys_tx(test_info, "*"),
        // 2. Store some data
        store_data_tx(test_info, "abc", &vec![1, 2, 3]),
        // 3. Store some data
        store_data_tx(test_info, "abc:xyz", &vec![1, 2, 3]),
        // 4. Store some data
        store_data_tx(test_info, "xyz", &vec![1, 2, 3]),
        // 5. Get keys with bad pattern. This shall fail
        get_account_keys_tx(test_info, "abc"),
        // 6. Get keys with abc pattern.
        get_account_keys_tx(test_info, "abc*"),
        // 7. Store some data
        store_data_tx(test_info, "*", &vec![1, 2, 3]),
        // 8. Store some data
        store_data_tx(test_info, "abc*", &vec![1, 2, 3]),
        // 9. Store some data
        store_data_tx(test_info, "ab*xyz", &vec![1, 2, 3]),
        // 10. Get keys with ab pattern.
        get_account_keys_tx(test_info, "ab*"),
        // 11. Get keys with wildcard pattern.
        get_account_keys_tx(test_info, "*"),
        // 12. Get test account contract.
        get_account_contract(test_info, &test_info.id),
        // 13. Get test not existing account contract.
        get_account_contract(test_info, "not-existing"),
    ]
}

fn check_basic_rxs(rxs: Vec<Receipt>) {
    // 0. Get keys with empty pattern. This shall fail
    assert!(!rxs[0].success);
    assert_eq!(
        "smart contract fault: last char of search pattern must be '*'",
        String::from_utf8_lossy(&rxs[0].returns)
    );
    // 1. Get keys with wildcard pattern. This shall return an empty Vec
    assert!(rxs[1].success);
    let res: Vec<String> = rmp_deserialize(&rxs[1].returns).unwrap();
    assert_eq!(res, Vec::<String>::new());
    // 2. Store some data
    assert!(rxs[2].success);
    // 3. Store some data
    assert!(rxs[3].success);
    // 4. Store some data
    assert!(rxs[4].success);
    // 5. Get keys with bad pattern. This shall fail
    assert!(!rxs[5].success);
    assert_eq!(
        "smart contract fault: last char of search pattern must be '*'",
        String::from_utf8_lossy(&rxs[5].returns)
    );

    // 6. Get keys with abc pattern.
    assert!(rxs[6].success);
    let mut res: Vec<String> = rmp_deserialize(&rxs[6].returns).unwrap();
    let mut expected = vec!["abc".to_string(), "abc:xyz".to_string()];
    res.sort();
    expected.sort();
    assert_eq!(res, expected);
    // 8. Store some data
    assert!(rxs[8].success);
    // 9. Store some data
    assert!(rxs[9].success);
    // 10. Get keys with ab pattern.
    assert!(rxs[10].success);
    let mut res: Vec<String> = rmp_deserialize(&rxs[10].returns).unwrap();
    let mut expected = vec![
        "ab*xyz".to_string(),
        "abc".to_string(),
        "abc*".to_string(),
        "abc:xyz".to_string(),
    ];
    res.sort();
    expected.sort();
    assert_eq!(res, expected);
    // 11. Get keys with wildcard pattern.
    assert!(rxs[11].success);
    let mut res: Vec<String> = rmp_deserialize(&rxs[11].returns).unwrap();
    let mut expected = vec![
        "abc".to_string(),
        "abc:xyz".to_string(),
        "xyz".to_string(),
        "*".to_string(),
        "abc*".to_string(),
        "ab*xyz".to_string(),
    ];
    res.sort();
    expected.sort();
    assert_eq!(res, expected);
<<<<<<< HEAD
>>>>>>> 2265110 (added test contract integration test)
=======

    // 12. Get test account contract.
    assert!(rxs[12].success);
    let buf: Vec<u8> = rmp_deserialize(&rxs[12].returns).unwrap();
    let hash = Hash::from_bytes(&buf).unwrap();
    assert_eq!(*TEST_APP_HASH, hash);

    // 13. Get test not existing account contract.
    assert!(rxs[13].success);
    let buf: Vec<u8> = rmp_deserialize(&rxs[13].returns).unwrap();
    assert_eq!(buf, Vec::<u8>::new());
>>>>>>> 6e01bdc (trinci-smartcontracts public release)
}

#[test]
fn test_contract() {
    // Instance the application.
    let mut app = TestApp::default();

    // Create and execute transactions.
    let txs = create_txs();
    let rxs = app.exec_txs(txs);
<<<<<<< HEAD
    check_rxs(rxs);
=======
    check_basic_rxs(rxs);
>>>>>>> 2265110 (added test contract integration test)
}
