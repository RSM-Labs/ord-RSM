use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use ordinals::{Rune, RuneId};
use ordinals::dao::Dao;
use ordinals::trading::Trading;
use crate::amm::AmmCalculateResult;
use crate::context::OperateContext;
use crate::state::State;

pub trait Contract: Any + Send + Sync {

    fn clone_box(&self) -> Arc<dyn Contract>;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_info(&self) -> String;
    fn dump_state(&self) -> String;
    fn get_state(&self, state_name: State, address: &str, ticker: &str) -> f64;

    //for AMM
    fn add_liquidity(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult>;
    fn remove_liquidity(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult>;
    fn swap(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult>;
    fn query_add_liquidity_result(&self, a0e: f64, a1e: f64, slippage: f64, deadline: u64) -> AmmCalculateResult;
    fn query_remove_liquidity_result(&self, lp_amount: f64, slippage: f64, deadline: u64) -> AmmCalculateResult;
    fn query_swap_result(&self, ticker_in: &str, amount_in: f64, slippage: f64, deadline: u64) -> AmmCalculateResult;

    fn sb2_mint(&mut self, address: &str, ticker: &str, value: f64);
    fn sb3_mint(&mut self, address: &str, ticker: &str, value: f64);
    fn sba2_mint(&mut self, ticker: &str, value: f64);
    fn sba3_mint(&mut self, ticker: &str, value: f64);
    fn sb2_burn(&mut self, address: &str, ticker: &str, value: f64);
    fn sb3_burn(&mut self, address: &str, ticker: &str, value: f64);

    fn get_another_ticker(&self, ticker: &str) -> String;

    fn get_ticker_pair(&self) -> (String, String);

    //for Lending
    // fn borrow(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult>;
    // fn refund(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult>;
    // fn deposit(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult>;
    // fn withdraw(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult>;
    // fn liquidate(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WrappedRuneContract {
    pub parent: Option<RuneId>,
    pub myself: RuneId,
    pub rune: Rune,
    pub contract: u8,
    pub mint2_amount: u128,
    pub burn3_able_rune_ids: (Option<Rune>, Option<Rune>),
    pub trading: Option<Trading>,
    pub dao: Option<Dao>,
    pub sba2: HashMap<String, f64>,
    pub sba3: HashMap<String, f64>,
    pub sb2: HashMap<String, HashMap<String, f64>>,
    pub sb3: HashMap<String, HashMap<String, f64>>,
}

impl Default for WrappedRuneContract {
    fn default() -> Self {
        WrappedRuneContract {
            parent: Default::default(),
            myself: Default::default(),
            rune: Default::default(),
            contract: 0,
            mint2_amount: 0,
            burn3_able_rune_ids: (None, None),
            trading: None,
            dao: None,
            sba2: Default::default(),
            sba3: Default::default(),
            sb2: Default::default(),
            sb3: Default::default(),
        }
    }
}

impl WrappedRuneContract {

    pub fn get_state(&self, state_name: State, address: &str, ticker: &str) -> f64{
        match state_name {
            State::StateBalanceForMint2 => {
                if let Some(user_sb2) = self.sb2.get(address) {
                    *user_sb2.get(ticker).unwrap_or(&0.0)
                } else {
                    0.0
                }
            },
            State::StateBalanceOfApplicationForMint2 => {
                *self.sba2.get(ticker).unwrap_or(&0.0)
            },
            State::StateBalanceForMint3 => {
                if let Some(user_sb3) = self.sb3.get(address) {
                    *user_sb3.get(ticker).unwrap_or(&0.0)
                } else {
                    0.0
                }
            },
            State::StateBalanceOfApplicationForMint3 => {
                *self.sba3.get(ticker).unwrap_or(&0.0)
            },
            _ => 0.0,
        }
    }

    pub fn burn_state(&mut self, state_name: State, address: &str, ticker: &str, value: f64) {
        match state_name {
            State::StateBalanceForMint2 => {
                self.sb2_burn(address, ticker, value);
            },
            State::StateBalanceForMint3 => {
                self.sb3_burn(address, ticker, value);
            },
            _ => {}
        }
    }

    pub fn sb2_mint(&mut self, address: &str, ticker: &str, value: f64) {
        if address.is_empty() || ticker.is_empty() || value.is_nan() || value <= 0.0 {
            return;
        }
        let user_sb2 = self.sb2.entry(address.to_string()).or_insert_with(HashMap::new);
        let balance = user_sb2.entry(ticker.to_string()).or_insert(0.0);
        *balance += value;

        self.sba2_mint(ticker, value);
    }

    pub fn sb3_mint(&mut self, address: &str, ticker: &str, value: f64) {
        if address.is_empty() || ticker.is_empty() || value.is_nan() || value <= 0.0 {
            return;
        }
        let user_sb3 = self.sb3.entry(address.to_string()).or_insert_with(HashMap::new);
        let balance = user_sb3.entry(ticker.to_string()).or_insert(0.0);
        *balance += value;

        self.sba3_mint(ticker, value);
    }

    pub fn sba2_mint(&mut self, ticker: &str, value: f64) {
        if ticker.is_empty() || value.is_nan() || value <= 0.0 {
            return;
        }
        let sba2_value = self.sba2.entry(ticker.to_string()).or_insert(0.0);
        *sba2_value += value;
    }

    pub fn sba3_mint(&mut self, ticker: &str, value: f64) {
        if ticker.is_empty() || value.is_nan() || value <= 0.0 {
            return;
        }
        let sha3_value = self.sba3.entry(ticker.to_string()).or_insert(0.0);
        *sha3_value += value;
    }

    pub fn sb2_burn(&mut self, address: &str, ticker: &str, value: f64) {
        if address.is_empty() || ticker.is_empty() || value.is_nan() || value <= 0.0 {
            return;
        }
        let entry = self.sb2.entry(address.to_string()).or_insert_with(HashMap::new);
        let current_value = entry.entry(ticker.to_string()).or_insert(0.0);
        if *current_value >= value {
            *current_value -= value;
        }
        self.sba2_burn(ticker, value);
    }

    pub fn sb3_burn(&mut self, address: &str, ticker: &str, value: f64) {
        if address.is_empty() || ticker.is_empty() || value.is_nan() || value <= 0.0 {
            return;
        }
        let entry = self.sb3.entry(address.to_string()).or_insert_with(HashMap::new);
        let current_value = entry.entry(ticker.to_string()).or_insert(0.0);
        if *current_value >= value {
            *current_value -= value;
        }
        self.sba3_burn(ticker, value);
    }

    pub fn sba2_burn(&mut self, ticker: &str, value: f64) {
        if ticker.is_empty() || value.is_nan() || value <= 0.0 {
            return;
        }
        let current_value = self.sba2.entry(ticker.to_string()).or_insert(0.0);
        if *current_value >= value {
            *current_value -= value;
        }
    }

    pub fn sba3_burn(&mut self, ticker: &str, value: f64) {
        if ticker.is_empty() || value.is_nan() || value <= 0.0 {
            return;
        }
        let current_value = self.sba3.entry(ticker.to_string()).or_insert(0.0);
        if *current_value >= value {
            *current_value -= value;
        }
    }

    pub fn get_myself_ticker(&self) -> String{
        self.myself.to_str()
    }
}

#[derive(Debug)]
pub enum ContractExecResult {
    Success(String),
    ValidationError(String),
    UnknownException(String),
}

impl ContractExecResult {
    fn describe(&self) -> String {
        match self {
            ContractExecResult::Success(msg) => format!("Success: {}", msg),
            ContractExecResult::ValidationError(msg) => format!("Validation Error: {}", msg),
            ContractExecResult::UnknownException(msg) => format!("Unknown Exception: {}", msg),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContractTemplate {
    Base = 0,
    Governance = 1,
    AMM = 2,
    Staking = 4,
    Stablecoin = 6,
    Lending = 7,
}

impl ContractTemplate {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ContractTemplate::Base),
            1 => Some(ContractTemplate::Governance),
            2 => Some(ContractTemplate::AMM),
            4 => Some(ContractTemplate::Staking),
            6 => Some(ContractTemplate::Stablecoin),
            7 => Some(ContractTemplate::Lending),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuneContractInfo {
    pub parent: Option<RuneId>,
    pub myself: RuneId,
    pub rune: Rune,
    pub contract: u8,
    pub mint2_amount: u128,
    pub burn3_able_rune_ids: (Option<Rune>, Option<Rune>),
    pub trading: Option<Trading>,
    pub dao: Option<Dao>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseContract {
    pub wrapped_rune_contract: WrappedRuneContract,
}

impl Contract for BaseContract {
    fn clone_box(&self) -> Arc<dyn Contract> {
        Arc::new(self.clone())
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_info(&self) -> String {
        todo!()
    }

    fn dump_state(&self) -> String {
        let json = serde_json::to_string(self).unwrap();
        json
    }

    fn get_state(&self, state_name: State, address: &str, ticker: &str) -> f64 {
        self.wrapped_rune_contract.get_state(state_name, address, ticker)
    }

    fn add_liquidity(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult> {
        todo!()
    }

    fn remove_liquidity(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult> {
        todo!()
    }

    fn swap(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult> {
        todo!()
    }

    fn query_add_liquidity_result(&self, a0e: f64, a1e: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        todo!()
    }

    fn query_remove_liquidity_result(&self, lp_amount: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        todo!()
    }

    fn query_swap_result(&self, ticker_in: &str, amount_in: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        todo!()
    }

    fn sb2_mint(&mut self, address: &str, ticker: &str, value: f64) {
        self.wrapped_rune_contract.sb2_mint(address, ticker, value)
    }

    fn sb3_mint(&mut self, address: &str, ticker: &str, value: f64) {
        self.wrapped_rune_contract.sb3_mint(address, ticker, value)
    }

    fn sba2_mint(&mut self, ticker: &str, value: f64) {
        self.wrapped_rune_contract.sba2_mint(ticker, value)
    }

    fn sba3_mint(&mut self, ticker: &str, value: f64) {
        self.wrapped_rune_contract.sba3_mint(ticker, value)
    }

    fn sb2_burn(&mut self, address: &str, ticker: &str, value: f64) {
        self.wrapped_rune_contract.sb2_burn(address, ticker, value)
    }

    fn sb3_burn(&mut self, address: &str, ticker: &str, value: f64) {
        self.wrapped_rune_contract.sb3_burn(address, ticker, value)
    }

    fn get_another_ticker(&self, ticker: &str) -> String {
        todo!()
    }

    fn get_ticker_pair(&self) -> (String, String) {
        todo!()
    }
}