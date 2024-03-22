
use crate::payouts::{process_payout};

use crate::satellite::{
    add_user,
    get_users,
    set_user_data,
    update_satellite_address,
    initialize_satellite,
    process_received_data,
    get_satellite,
    get_user_data_for_range
};

use soroban_sdk::{
    contract,
    contractimpl,
    contractmeta,
    symbol_short,
    token,
    vec,
    Address,
    Env,
    Map,
    Symbol,
    Vec,
    String,
};

use crate::storage_types::{
    DataKey,
    UserData,
    INSTANCE_BUMP_AMOUNT,
    INSTANCE_LIFETIME_THRESHOLD
};

use crate::we_work_out::{
    get_end_date,
    get_interval,
    get_interval_target,
    get_start_date,
    get_state,
    init_we_work_out
};

// TODO: Satellite Trait required for contract to be able to receive data from oracle (WIP)
// pub trait SatelliteTrait {
//     fn add_user(e: Env, satellite: Address, user: Address);
//
//     fn add_data(e: Env, date: u64, satellite: Address, user: Address, value: u64, source: Symbol);
//
//     fn get_contract_state(e: Env);
//
//     fn init_satellite(e: &Env, admin: Address, satellite: Address);
//
//     fn update_satellite_address(e: Env, satellite: Address, new_satellite: Address);
// }

// Metadata that is added on to the WASM custom section
contractmeta!(
    key = "Description",
    val = "The Satellite contract for payout out based on set parameters for received oracle data."
);

#[contract]
pub struct Satellite;

#[contractimpl]
impl Satellite {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }

    // Convenience initializer for setting up Satellite and We Work Out functionality
    pub fn initialize(
        e: Env,
        admin: Address, // admin = Contract Admin (initializor of contract via satellite)
        interval: u64,
        interval_target: u64,
        payout_mode: Symbol, // convert to PayoutMode enum value
        satellite: Address, // relayer = Satellite Address
        start_date: u64, // TODO: Replace with some other data
        end_date: u64,
        token: Address, // Contract address
        ) {

        assert!(
            !e.storage().instance().has(&DataKey::Initialized),
            "Contract already initialized"
        );

        Self::init_satellite(&e, admin.clone(), satellite);
        init_we_work_out(&e, interval, interval_target, payout_mode, start_date, end_date);

        let admin_data = UserData {
            user: admin.clone(),
            data: vec![&e],
        };

        // TODO: After testing, no need to set admin in contract's user data
        set_user_data(&e, vec![&e, admin_data.clone()]);

        e.storage().instance().set(&DataKey::Initialized, &true);
        e.storage().instance().set(&DataKey::Token, &token);
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }

    pub fn init_satellite(
        e: &Env,
        admin: Address,
        satellite: Address,
    ) {
        initialize_satellite(&e, admin.clone(), satellite);
    }

    pub fn process_payout(e: Env, satellite: Address, token_address: Address) {
        satellite.require_auth();

        let user_data = get_user_data_for_range(&e, 0, 9999999999);

        process_payout(&e, satellite, token_address, user_data);
    }

    pub fn get_contract_state(
        e: Env,
    ) {
        get_state(&e);
    }

    pub fn add_user(
        e: Env,
        satellite: Address,
        user: Address,
    ) -> Vec<UserData> {
            add_user(&e, satellite, user)
        }

    pub fn get_user_data_for_range(
        e: Env,
        start_date: u64,
        end_date: u64,
    ) -> Map<Address, u64> {
            get_user_data_for_range(&e, start_date, end_date)
        }

    pub fn add_data(
        e: Env,
        date: u64,
        satellite: Address,
        user: Address,
        value: u64,
        source: Symbol,
    ) -> Vec<UserData> {
            process_received_data(&e, date, satellite, user, value, source)
        }

    pub fn get_users(
            e: Env,
        ) -> Vec<UserData> {
            get_users(&e)
        }

    pub fn update_satellite_address(
            e: Env,
            satellite: Address,
            new_satellite: Address,
        ) {
            update_satellite_address(&e, satellite, new_satellite);
        }

    // TODO: Fund contract functionality (rather than token minting)

    // TODO: Terminate contract
}
