use std::any::Any;
use crate::context::OperateContext;
use crate::contract::{Contract, ContractExecResult, WrappedRuneContract};
use crate::state::State;

pub struct AutomatedLiquidityContract {
    pub ticker0: String,
    pub ticker0_decimals: i64,
    pub ticker1: String,
    pub ticker1_decimals: i64,
    pub supply: f64,
    pub reserve0: f64,
    pub reserve1: f64,
    pub wrapped_rune_contract: WrappedRuneContract,
}

impl Contract for AutomatedLiquidityContract {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn add_liquidity(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult> {
        let address = operate_context.address.as_ref().ok_or_else(|| {
            ContractExecResult::ValidationError("Address is missing!".to_string())
        })?;

        let address_hex = address.to_string();
        let block_timestamp = operate_context.block_timestamp;

        let burn3 = operate_context.burn3s.as_ref().ok_or_else(|| {
            ContractExecResult::ValidationError("Burn3 is missing!".to_string())
        })?;

        let edict0 = &burn3.edicts[0];
        let amount0_desired = edict0.amount;

        let edict1 = &burn3.edicts[1];
        let amount1_desired = edict1.amount;

        let ext = burn3.ext.as_ref().ok_or_else(|| {
            ContractExecResult::ValidationError("Extension data is missing!".to_string())
        })?;

        let amount0_min = ext.amount0_min.ok_or_else(|| {
            ContractExecResult::ValidationError("amount0_min is missing!".to_string())
        })?;
        let amount1_min = ext.amount1_min.ok_or_else(|| {
            ContractExecResult::ValidationError("amount1_min is missing!".to_string())
        })?;
        let deadline = ext.deadline.ok_or_else(|| {
            ContractExecResult::ValidationError("Deadline is missing!".to_string())
        })?;

        let exec_result = self.do_add_liquidity(
            address_hex,
            amount0_desired as f64,
            amount1_desired as f64,
            amount0_min as f64,
            amount1_min as f64,
            u64::from(deadline),
            block_timestamp
        );
        match exec_result {
            ContractExecResult::Success(_0) => Ok(()),
            err => Err(err),
        }
    }

    fn remove_liquidity(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult> {
        let address = operate_context.address.as_ref().ok_or_else(|| {
            ContractExecResult::ValidationError("Address is missing!".to_string())
        })?;

        let address_hex = address.to_string();
        let block_timestamp = operate_context.block_timestamp;

        let burn2 = operate_context.burn2s.as_ref().ok_or_else(|| {
            ContractExecResult::ValidationError("Burn2 is missing!".to_string())
        })?;

        let edict0 = &burn2.edicts[0];
        let lp_amount = edict0.amount;

        let ext = burn2.ext.as_ref().ok_or_else(|| {
            ContractExecResult::ValidationError("Extension data is missing!".to_string())
        })?;

        let amount0_min = ext.amount0_min.ok_or_else(|| {
            ContractExecResult::ValidationError("amount0_min is missing!".to_string())
        })?;
        let amount1_min = ext.amount1_min.ok_or_else(|| {
            ContractExecResult::ValidationError("amount1_min is missing!".to_string())
        })?;
        let deadline = ext.deadline.ok_or_else(|| {
            ContractExecResult::ValidationError("Deadline is missing!".to_string())
        })?;

        let exec_result = self.do_remove_liquidity(
            address_hex,
            lp_amount as f64,
            amount0_min as f64,
            amount1_min as f64,
            u64::from(deadline),
            block_timestamp
        );
        match exec_result {
            ContractExecResult::Success(_0) => Ok(()),
            err => Err(err),
        }
    }

    fn swap(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult> {
        let address = operate_context.address.as_ref().ok_or_else(|| {
            ContractExecResult::ValidationError("Address is missing!".to_string())
        })?;

        let address_hex = address.to_string();
        let block_timestamp = operate_context.block_timestamp;

        let burn3 = operate_context.burn3s.as_ref().ok_or_else(|| {
            ContractExecResult::ValidationError("Burn3 is missing!".to_string())
        })?;

        let edict = &burn3.edicts[0];
        let ticker_in = format!("{}:{}", edict.id.block, edict.id.tx);
        let amount_in = edict.amount;

        let ext = burn3.ext.as_ref().ok_or_else(|| {
            ContractExecResult::ValidationError("Extension data is missing!".to_string())
        })?;

        let amount_out_min = ext.amount_out_min.ok_or_else(|| {
            ContractExecResult::ValidationError("Amount out min is missing!".to_string())
        })?;
        let deadline = ext.deadline.ok_or_else(|| {
            ContractExecResult::ValidationError("Deadline is missing!".to_string())
        })?;

        let exec_result = self.do_swap(
            address_hex,
            ticker_in,
            amount_in as f64,
            amount_out_min as f64,
            u64::from(deadline),
            block_timestamp
        );
        match exec_result {
            ContractExecResult::Success(_0) => Ok(()),
            err => Err(err),
        }
    }
}

impl AutomatedLiquidityContract {

    pub fn new() -> Self {
        AutomatedLiquidityContract {
            ticker0: "".to_string(),
            ticker0_decimals: 0,
            ticker1: "".to_string(),
            ticker1_decimals: 0,
            supply: 0.0,
            reserve0: 0.0,
            reserve1: 0.0,
            wrapped_rune_contract: Default::default(),
        }
    }

    /**
     * AMM swap.
     */
    pub fn do_swap(&mut self, address: String, ticker_in: String, amount_in: f64,
                   amount_out_min: f64, deadline: u64, block_timestamp: u64) -> ContractExecResult {
        todo!();
    }

    /**
    * AMM add liquidity.
    */
    pub fn do_add_liquidity(&mut self, address: String, amount0_desired: f64, amount1_desired: f64,
                            amount0_min: f64, amount1_min: f64, deadline: u64, block_timestamp: u64) -> ContractExecResult {
        todo!();
    }

    /**
     * AMM remove liquidity.
     */
    pub fn do_remove_liquidity(&mut self, address: String, lp_amount: f64, amount0_min: f64,
                               amount1_min: f64, deadline: u64, block_timestamp: u64) -> ContractExecResult {
        todo!();
    }

    fn get_state(&mut self, state_name: State, address: String, ticker: String) -> f64{
        match state_name {
            State::Reserve0 => {
                self.reserve0
            },
            State::Reserve1 => {
                self.reserve1
            },
            State::Supply => {
                self.supply
            },
            _ => self.wrapped_rune_contract.get_state(state_name, address, ticker),
        }
    }

    pub fn update_reserve0(&mut self, new_value: f64) {
        self.reserve0 = new_value;
    }

    pub fn reserve0_mint(&mut self, value: f64) {
        if value <= 0.0 {
            println!("Value must be greater than 0");
            return
        }
    }

    pub fn reserve0_transfer(&mut self, value: f64, address: String) {
        if value <= 0.0 || address.is_empty() {
            println!("Value must be greater than 0, or address must not be empty.");
            return;
        }
        if self.reserve0 < value {
            println!("Insufficient reserve0 balance.");
            return;
        }
        self.update_reserve0(self.reserve0 - value);
        self.wrapped_rune_contract.sb3_mint(address, self.ticker0.clone(), value)
    }

    pub fn update_reserve1(&mut self, new_value: f64) {
        self.reserve1 = new_value;
    }

    pub fn reserve1_mint(&mut self, value: f64) {
        if value <= 0.0 {
            println!("Value must be greater than 0");
            return;
        }
        self.update_reserve1(self.reserve1 + value);
    }

    pub fn reserve1_transfer(&mut self, value: f64, address: String) {
        if value <= 0.0 || address.is_empty() {
            println!("Value must be greater than 0, or address must not be empty.");
            return;
        }
        if self.reserve1 < value {
            println!("Insufficient reserve0 balance.");
            return;
        }
        self.update_reserve1(self.reserve1 - value);
        self.wrapped_rune_contract.sb3_mint(address, self.ticker1.clone(), value)
    }
}

pub fn validation_error(message: &str) -> ContractExecResult {
    ContractExecResult::ValidationError(message.to_string())
}

pub struct AmmCalculateResult {
    pub ticker0: String,
    pub ticker0_decimals: i64,
    pub ticker1: String,
    pub ticker1_decimals: i64,
    pub supply: f64,

    pub reserve0: f64,
    pub reserve1: f64,
    pub dl: u64,

    a0e: f64,
    a1e: f64,
    a0m: f64,
    a1m: f64,
    aelp: f64,
    ap: f64,

    ticker_in: String,
    ai: f64,
    aoe: f64,
    aom: f64,
    tbhp: f64,
    ttp: f64,
    tt: f64,
    tbh: f64,
    fee: f64,
    fee_percentage: f64,
    slippage: f64,
    pool_share: f64,
    return_ticker0: String,
    return_ticker1: String,
    price_impact: f64,
    success: bool,
}

impl AmmCalculateResult {
    pub fn new() -> Self {
        AmmCalculateResult {
            ticker0: "".to_string(),
            ticker0_decimals: 0,
            ticker1: "".to_string(),
            ticker1_decimals: 0,
            supply: 0.0,
            reserve0: 0.0,
            reserve1: 0.0,
            dl: 0,
            a0e: 0.0,
            a1e: 0.0,
            a0m: 0.0,
            a1m: 0.0,
            aelp: 0.0,
            ap: 0.0,
            ticker_in: "".to_string(),
            ai: 0.0,
            aoe: 0.0,
            aom: 0.0,
            tbhp: 0.0,
            ttp: 0.0,
            tt: 0.0,
            tbh: 0.0,
            fee: 0.0,
            fee_percentage: 0.0,
            slippage: 0.0,
            pool_share: 0.0,
            return_ticker0: "".to_string(),
            return_ticker1: "".to_string(),
            price_impact: 0.0,
            success: false,
        }
    }
}