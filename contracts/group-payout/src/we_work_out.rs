
use soroban_sdk::{
    token,
    Address,
    Env,
    Symbol
};

use crate::storage_types::{DataKey, State};

pub fn init_we_work_out(
    e: &Env,
    interval: u64,
    interval_target: u64,
    payout_mode: Symbol,
    start_date: u64,
    end_date: u64,
) {
    e.storage().instance().set(&DataKey::EndDate, &end_date);
    e.storage().instance().set(&DataKey::Interval, &interval);
    e.storage().instance().set(&DataKey::IntervalTarget, &interval_target);
    e.storage().instance().set(&DataKey::PayoutMode, &payout_mode);
    e.storage().instance().set(&DataKey::StartDate, &start_date);
}

pub fn get_state(e: &Env) -> State {
    let end_date = get_end_date(e);
    let start_date = get_start_date(e);
    let current_timestamp = get_ledger_timestamp(e);

    if current_timestamp < start_date {
        return State::Pending;
    } else if current_timestamp > start_date && current_timestamp < end_date {
        return State::Running;
    } else if current_timestamp > end_date {
        return State::Complete
    };

    State::Complete
}

pub fn get_ledger_timestamp(e: &Env) -> u64 {
    e.ledger().timestamp()
}

pub fn get_start_date(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get::<_, u64>(&DataKey::StartDate)
        .expect("not initialized")
}

pub fn get_end_date(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get::<_, u64>(&DataKey::EndDate)
        .expect("not initialized")
}

pub fn get_interval(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get::<_, u64>(&DataKey::Interval)
        .expect("not initialized")
}

pub fn get_interval_target(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get::<_, u64>(&DataKey::IntervalTarget)
        .expect("not initialized")
}
