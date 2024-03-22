//! This module handles Satellite oracle functionality
//! such as: receiving oracle data, storing data to user entry
//!
use soroban_sdk::{vec, map, Address, Env, Map, Symbol, Vec};

use crate::storage_types::{DataKey, State, UserData, UserDataEntry};

pub fn initialize_satellite(
    e: &Env,
    admin: Address,
    satellite: Address,
) {
    e.storage().instance().set(&DataKey::Admin, &admin);
    e.storage().instance().set(&DataKey::Satellite, &satellite);
}

pub fn add_user(
    e: &Env,
    satellite: Address,
    user: Address,
) -> Vec<UserData> {
    satellite.require_auth();

    let mut users = get_users(&e);
    let new_user_data = UserData {
        user: user,
        data: vec![&e],
    };

    // TODO: Check that user doesn't already exist in contract.
    users.push_back(new_user_data);

    // Save users
    set_user_data(&e, users)
}

pub fn process_received_data(
    e: &Env,
    date: u64,
    satellite: Address,
    user: Address,
    value: u64,
    source: Symbol) -> Vec<UserData> {
        satellite.require_auth();
        assert_eq!(
            satellite,
            get_satellite(e),
            "Invalid Satelllite address, unauthorized."
        );
        // Get UserData
        let mut users = get_users(&e);

        // Get user by user Address (maybe change to 9 character ID?)
        for i in 0..users.len() {
            let mut user_data = users.get(i).unwrap();
            if user_data.user == user {
                let new_value = UserDataEntry {
                    date: date,
                    value: value,
                    source: source.clone(),
                };
                user_data.data.push_back(new_value);
                users.set(i, user_data);
            }
        }

        set_user_data(&e, users)
    }

// Getters

pub fn get_users(e: &Env) -> Vec<UserData> {
    e.storage()
        .instance()
        .get::<_, Vec<UserData>>(&DataKey::Users)
        .expect("Users Not Initialized")
}

pub fn get_user_data_for_range(e: &Env, start_date: u64, end_date: u64) -> Map<Address, u64> {
    let users = get_users(&e).clone();
    let mut summed_user_data: Map<Address, u64> = map![&e];

    for i in 0..users.len() {
        let fetched_user = users.get(i).unwrap();
        for j in 0..fetched_user.data.len() {
            let entry = fetched_user.data.get(j).unwrap();
            if entry.date > start_date && entry.date < end_date {
                // In the range
                if summed_user_data.contains_key(fetched_user.user.clone()) {
                    // +=
                    let current_value = summed_user_data.get(fetched_user.user.clone()).unwrap();
                    summed_user_data.set(fetched_user.user.clone(), current_value + entry.value);
                } else {
                    // init key
                    summed_user_data.set(fetched_user.user.clone(), entry.value);
                }
            }
        }
    }

    summed_user_data
}

pub fn get_ledger_timestamp(e: &Env) -> u64 {
    e.ledger().timestamp()
}

pub fn get_satellite(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<_, Address>(&DataKey::Satellite)
        .expect("not initialized")
}

// Setters

pub fn set_user_data(e: &Env, users: Vec<UserData>) -> Vec<UserData> {
    let key = DataKey::Users;
    e.storage().instance().set(&key, &users);
    users
}

pub fn update_satellite_address(
        e: &Env,
        satellite: Address,
        new_satellite: Address,
    ) -> Address {
        satellite.require_auth();
        assert_eq!(
            satellite,
            get_satellite(e),
            "Caller is not the satellite address"
        );

        e.storage().instance().set(&DataKey::Satellite, &new_satellite);
        new_satellite
    }
