use soroban_sdk::{vec, symbol_short, token, Address, Env, Map, Symbol, Vec};

use crate::storage_types::{DataKey, State, UserData, UserDataEntry};

pub fn get_payout_mode(e: &Env) -> Symbol {
    e.storage()
        .instance()
        .get::<_, Symbol>(&DataKey::PayoutMode)
        .expect("PayoutMode Not Initialized")
}

pub fn get_payout_threshold(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get::<_, u64>(&DataKey::IntervalTarget)
        .expect("not initialized")
}

pub fn process_payout(e: &Env, satellite: Address, token_address: Address, user_data: Map<Address, u64>) {
    satellite.require_auth();

    let payout_mode = get_payout_mode(&e);

    if payout_mode == symbol_short!("WinTakAll") {
        payout_winner_take_all(&e, token_address, user_data);
    } else if payout_mode == symbol_short!("SplitEven") {
        payout_all_meeting_threshold(&e, token_address, user_data);
    }
}

pub fn payout_winner_take_all(e: &Env, token_address: Address, user_data: Map<Address, u64>) {
    let token_client = token::StellarAssetClient::new(&e, &token_address);

    let mut winning_address: Option<Address> = None;
    let mut highest_val: u64 = 0;
    for (key, value) in user_data.iter() {
        if value > highest_val {
            highest_val = value.clone();
            winning_address = Some(key.clone());
        }
    };

    if let Some(val) = winning_address {
        // Mint tokens to winning address
        token_client.admin().require_auth(); // Satellite is configured to be token admin
        token_client.mint(&val, &10000000);
    }
}

pub fn payout_all_meeting_threshold(e: &Env, token_address: Address, user_data: Map<Address, u64>) {
    let token_client = token::StellarAssetClient::new(&e, &token_address);

    let mut winning_addresses: Vec<Address> = vec![&e];
    let mut threshold: u64 = get_payout_threshold(&e); // TODO: Fetch payout threshold

    for (key, value) in user_data.iter() {
        if value > threshold {
            winning_addresses.push_back(key.clone());
        }
    };

    for value in winning_addresses.iter() {
        token_client.admin().require_auth(); // Satellite is configured to be token admin
        token_client.mint(&value, &10000000);
    };
}


// pub fn get_token(e: &Env) -> Address {
//     e.storage()
//         .instance()
//         .get::<_, Address>(&DataKey::Token)
//         .expect("not initialized")
// }
//
// pub fn transfer_token(e: &Env, to: &Address, amount: &i128) {
//     let token_contract_id = &get_token(e);
//     let client = token::Client::new(e, token_contract_id);
//     client.transfer(&e.current_contract_address(), to, amount);
// }
//
// pub fn transfer_xlm(e: &Env, to: &Address, amount: &i128) {
//     // TODO: Make this XLM rather than some token.
//     let token_contract_id = &get_token(e);
//     let client = token::Client::new(e, token_contract_id);
//     client.transfer(&e.current_contract_address(), to, amount);
// }

// pub fn get_elegible_users(e: &Env) {
    // TODO: Get list of users eligible of payout

// }

// TODO: Payout Split evenly

// TODO: Payout Tiered

// TODO: Payout Team-based
