#![cfg(test)]
extern crate std;

use std::println;
use crate::{contract::Satellite, SatelliteClient};

use soroban_sdk::{
    symbol_short,
    vec,
    testutils::{Address as _},
    token,
    Env,
    String,
    Address};

// fn create_token<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
//     let token = TokenClient::new(e, &e.register_contract(None, Token {}));
//     token.initialize(admin, &7, &"name".into_val(e), &"symbol".into_val(e));
//     token
// }

fn e() -> Env {
    Env::default()
}

fn init_contract(e: Env) -> SatelliteClient<'static> {
    e.mock_all_auths();
    let contract_id = e.register_contract(None, Satellite);
    let client = SatelliteClient::new(&e, &contract_id);

    client
}

// Creates arbitrary token for testing.
fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    // let contract_address = Address::from_string(&String::from_str(&e, "CAV5C6MNCZFD2DQQP7E3QJ2UJ2RYZRODRBNRO6VEG2H46FJSHI27EKRZ"));
    let contract_address = e.register_stellar_asset_contract(admin.clone());
    (
        token::Client::new(e, &contract_address),
        token::StellarAssetClient::new(e, &contract_address),
    )
}

#[test]
fn hello() {
    let e = e();
    let client = init_contract(e.clone());

    let words = client.hello(&symbol_short!("Dev"));
    assert_eq!(
        words,
        vec![&e, symbol_short!("Hello"), symbol_short!("Dev"),]
    );
}

// TODO: Test add user to contract
#[test]
fn test_add_users() {
    let e = e();
    let client = init_contract(e.clone());

    let satellite = Address::generate(&e);
    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::from_string(&String::from_str(&e, "GDIYDMRMJ6PEVSYUFB6TWXSALOXG7SKDPOZO72EECPDTHAXQHWWA7APS"));

    let (token, token_admin) = create_token_contract(&e, &satellite);
    // client.create_token(satellite, &0, &"WorkBoat".into_val(e), &"WORK".into_val(e));

    client.initialize(&admin, &3600, &5, &symbol_short!("WinTakAll"), &satellite, &1705433843, &1705436843, &token_admin.address);

    let users = client.add_user(&satellite, &user1);

    assert_eq!(
        users.len(),
        2,
    );

    let more_users = client.add_user(&satellite, &user2);

    assert_eq!(
        more_users.len(),
        3,
    );

    // Add data entry to user 1
    client.add_data(&1705433943, &satellite, &user1, &(4 * 100), &symbol_short!["Strava"]);
    client.add_data(&1705435943, &satellite, &user1, &(2 * 100), &symbol_short!["Strava"]);
    client.add_data(&1705436943, &satellite, &user2, &(12 * 100), &symbol_short!["Strava"]);

    println!("\nWith Data:");
    let data_users = client.get_users();

    for i in 0..data_users.len() {
        let user = data_users.get(i).unwrap().clone();
        if user.user == user1 {
            println!("Thats a match: {:?}, {:?}, {:?}", user.user, user1, user.data);
        }
    }

    println!("\n\nHere we go...\n\n");

    let fetched_map_one = client.get_user_data_for_range(&1705433942, &1705435944);

    let fetched_map_all = client.get_user_data_for_range(&0, &2705435944);

    println!("fetched users' data: \n{:?}, \n{:?}", fetched_map_one, fetched_map_all);

    // let token_address = Address::from_string(&String::from_str(&e, "GCFBPRPMZF7WOKL7JUYWO5NC3QEZRLM2LPCLX5O6DKHS7PYE7KVYIVW7"));
    // token_address.mock_all_auths();
    // client.process_payout(&satellite, &token_admin.address);

    token_admin.mock_all_auths().mint(&user1, &1000);
    let user_1_balance = token.balance(&user1);
    // let user_2_balance = token.balance(&user2);

    println!("balance: \n{:?},", user_1_balance);
}

#[test]
fn mint_token() {
    println!("mint_token\n");
    let e = e();
    let client = init_contract(e.clone());

    let satellite = Address::generate(&e);
    let token_admin = Address::generate(&e);
    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);

    let (token, token_admin) = create_token_contract(&e, &token_admin);

    client.initialize(&admin, &3600, &5, &symbol_short!("SplitEven"), &satellite, &1705433843, &1705436843, &token_admin.address);

    let users = client.add_user(&satellite, &user1);

    token_admin.mock_all_auths().mint(&user1, &1000);

    let balance = token.balance(&user1);

    println!("Token: {:?}", token.name());
    println!("Symbol: {:?}", token.symbol().len());
    println!("Decimals: {:?}", token.decimals());
    println!("Balance: {:?}\n", balance);
}

// #[test]
// fn mint_to_user() {
//     println!("mint_to_user\n");
//     // let e = e();
//     // let client = init_contract(e.clone());
//     //
//     // let satellite = Address::generate(&e);
//     // let token_admin = Address::generate(&e);
//     // let admin = Address::generate(&e);
//     // let user1 = Address::generate(&e);
//     // let user2 = Address::generate(&e);
//     //
//     // let (token, token_admin) = create_token_contract(&e, &token_admin);
//     //
//     // client.initialize(&admin, &3600, &5, &symbol_short!("SplitEven"), &satellite, &1705433843, &1705436843, &token_admin.address);
//     //
//     // let users = client.add_user(&satellite, &user1);
//     // // client.add_data(&1705433943, &satellite, &user1, &(4 * 100), &symbol_short!["Strava"]);
//     // // client.process_payout(&satellite);
//     // // token_admin.mock_all_auths().mint(&user1, &1000);
//     // client.mint(&user1, &1973);
//     //
//     // let user_1_balance = client.get_user_balance(&user1);
//     //
//     // println!("User Balance: \n{:?}", user_1_balance);
//     // //
//     // // println!("Token: {:?}", client.read_name());
//     // // println!("Symbol: {:?}", client.read_symbol());
//     // println!("Decimal: {:?}", client.read_decimal());
//     // println!("Balance: {:?}", user_1_balance);
// }

// fn e() -> Env {
//     Env::default()
// }
//
// fn add_user_data() {
//
// }
//
// // TODO: Test issue payout
// fn issue_payout() {
//
// }
//
// // TODO: Test fund contract
// fn fund_contract() {
//
// }
//
// // TODO: Test terminate contract
// fn terminate_contract() {
//
// }
//
// // TODO: Test changing Satellite address
// fn update_satellite_address() {
//
// }
