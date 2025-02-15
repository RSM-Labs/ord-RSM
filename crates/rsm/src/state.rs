use std::fmt;
use std::fmt::Formatter;
use chrono::{DateTime, Utc};

pub struct StateDB {
    pub block_height: u32,
    pub block_hash: u32,
    pub block_time: DateTime<Utc>,
}

impl Default for StateDB {
    fn default() -> Self {
        StateDB {
            block_height: 0,
            block_hash: 0,
            block_time: Default::default(),
        }
    }
}

pub enum State {
    RecoverableStateBalanceForMint2,
    RecoverableStateBalanceForMint3,
    StateBalanceOfApplicationForMint2,
    StateBalanceOfApplicationForMint3,
    StateBalanceForMint2,
    StateBalanceForMint3,
    StateBalanceForOwnerMint,
    Ticker0,
    Ticker1,
    Reserve0,
    Reserve1,
    Supply,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let state_str = match self {
            State::RecoverableStateBalanceForMint2 => "rsb2",
            State::RecoverableStateBalanceForMint3 => "rsb3",
            State::StateBalanceOfApplicationForMint2 => "sba2",
            State::StateBalanceOfApplicationForMint3 => "sba3",
            State::StateBalanceForMint2 => "sb2",
            State::StateBalanceForMint3 => "sb3",
            State::StateBalanceForOwnerMint => "sbom",
            State::Ticker0 => "ticker0",
            State::Ticker1 => "ticker1",
            State::Reserve0 => "reserve0",
            State::Reserve1 => "reserve1",
            State::Supply => "supply",
        };
        write!(f, "{}", state_str)
    }
}

