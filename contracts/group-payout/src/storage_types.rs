use soroban_sdk::{contracttype, Address, Symbol, Vec};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone, Debug)]
#[contracttype]
pub struct UserData {
    pub user: Address,
    pub data: Vec<UserDataEntry>,
    // pub data: u64,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct UserDataEntry {
    pub date: u64,
    pub value: u64,
    pub source: Symbol, // e.g. Strava, Garmin, etc.
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin, // Owner/Deployer of contract
    BuyInAmount, // The amount required for buying into the competition (only for FundingMode == UserBuyIn)
    EndDate, // Contract expiration
    FundingMode,
    Initialized,
    IntervalTarget, // Value target per interval
    Interval, // Process Interval of contract (in seconds, minimum is one week)
    LastProcessDate,
    PayoutMode,
    Satellite, // Oracle - relayer of data
    StartDate, // Beginning of active period
    Token,
    User(Address),
    Users,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[contracttype]
pub enum State {
    Pending = 0,
    Running = 1,
    Complete = 2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[contracttype]
pub enum PayoutMode {
    SplitEven = 0, // Split evenly for all who qualify.
    WinTakAll = 1, // Winner takes all.
    // Tiered = 2,
    // Teams = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[contracttype]
pub enum FundingMode {
    AdminFunded = 0, // Contract Admin funds it.
    UserBuyIn = 1, // Users all buy in.
}
