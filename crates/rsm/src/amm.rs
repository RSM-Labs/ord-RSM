use std::any::Any;
use std::sync::Arc;
use serde::Deserialize;
use serde_with::serde_derive::Serialize;
use crate::context::OperateContext;
use crate::contract::{Contract, ContractExecResult, RuneContractInfo, WrappedRuneContract};
use crate::state::State;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    fn clone_box(&self) -> Arc<dyn Contract> {
        Arc::new(self.clone())
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_info(&self) -> String {
        let contract_info = RuneContractInfo {
            parent: self.wrapped_rune_contract.parent,
            myself: self.wrapped_rune_contract.myself,
            rune: self.wrapped_rune_contract.rune,
            contract: self.wrapped_rune_contract.contract,
            mint2_amount: self.wrapped_rune_contract.mint2_amount,
            burn3_able_rune_ids: self.wrapped_rune_contract.burn3_able_rune_ids,
            trading: self.wrapped_rune_contract.trading,
            dao: self.wrapped_rune_contract.dao,
        };
        serde_json::to_string(&contract_info).unwrap()
    }

    fn dump_state(&self) -> String {
        let json = serde_json::to_string(self).unwrap();
        json
    }

    fn get_state(&self, state_name: State, address: &str, ticker: &str) -> f64 {
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

    fn add_liquidity(&mut self, operate_context: &OperateContext) -> Result<(), ContractExecResult> {
        let address = operate_context.address.as_ref().ok_or_else(|| {
            ContractExecResult::ValidationError("Address is missing!".to_string())
        })?;

        let binding = address.to_string();
        let address_hex = binding.as_str();
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

        let binding = address.to_string();
        let address_hex = binding.as_str();
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

        let binding = address.to_string();
        let address_hex = binding.as_str();
        let block_timestamp = operate_context.block_timestamp;

        let burn3 = operate_context.burn3s.as_ref().ok_or_else(|| {
            ContractExecResult::ValidationError("Burn3 is missing!".to_string())
        })?;

        let edict = &burn3.edicts[0];
        let binding = edict.id.to_str();
        let ticker_in = binding.as_str();
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

    fn query_add_liquidity_result(&self, a0e: f64, a1e: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        if a0e <= 0.0 {
            log::error!("a0e must be greater than 0.");
            return AmmCalculateResult::new()
        }
        if a1e <= 0.0 {
            log::error!("a1e must be greater than 0.");
            return AmmCalculateResult::new()
        }
        if slippage <= 0.0 || slippage >= 1.0 {
            log::error!("slippage must be in (0, 1).");
            return AmmCalculateResult::new()
        }
        if deadline <= 0 {
            log::error!("deadline must be greater than 0.");
            return AmmCalculateResult::new()
        }

        if self.reserve0 == 0.0 && a1e <= 0.0 {
            log::error!("When pool is empty, a1e should not be 0.");
            return AmmCalculateResult::new()
        }

        // a1e = reserve1 / reserve0 * a0e
        let a1ew: f64 = if self.reserve0 > 0.0 { (self.reserve1 / self.reserve0) * a0e } else { a1e };

        let a0m = a0e * (1.0 - slippage);
        let a1m = a1ew * (1.0 - slippage);

        let lp_amounts = self.calc_lp_amounts_for_add_liquidity(a0e, a1ew);

        AmmCalculateResult {
            ticker0: self.ticker0.clone(),
            ticker0_decimals: self.ticker1_decimals,
            ticker1: self.ticker1.clone(),
            ticker1_decimals: self.ticker1_decimals,
            supply: self.supply,
            reserve0: self.reserve0,
            reserve1: self.reserve1,
            dl: deadline,
            a0e,
            a1e: a1ew,
            a0m,
            a1m,
            aelp: lp_amounts[0],
            ap: 0.0,
            ticker_in: String::new(),
            ai: 0.0,
            aoe: 0.0,
            aom: 0.0,
            tbhp: 0.0,
            ttp: 0.0,
            tt: 0.0,
            tbh: 0.0,
            fee: 0.0,
            fee_percentage: 0.0,
            slippage,
            pool_share: 0.0,
            return_ticker0: String::new(),
            return_ticker1: String::new(),
            price_impact: 0.0,
            success: true,
        }
    }

    fn query_remove_liquidity_result(&self, lp_amount: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        if lp_amount <= 0.0 {
            log::error!("ap must be greater than 0");
            return AmmCalculateResult::new()
        }
        if slippage <= 0.0 || slippage >= 1.0 {
            log::error!("slippage must be in (0, 1)");
            return AmmCalculateResult::new()
        }
        if deadline <= 0 {
            log::error!("deadline must be greater than 0");
            return AmmCalculateResult::new()
        }

        let amounts = self.calc_token_amount_for_remove_liquidity(lp_amount);

        let a0e = amounts[0];
        let a1e = amounts[1];
        let a0m = a0e * (1.0 - slippage);
        let a1m = a1e * (1.0 - slippage);

        AmmCalculateResult {
            ticker0: self.ticker0.clone(),
            ticker0_decimals: self.ticker1_decimals,
            ticker1: self.ticker1.clone(),
            ticker1_decimals: self.ticker1_decimals,
            supply: self.supply,
            reserve0: self.reserve0,
            reserve1: self.reserve1,
            dl: deadline,
            a0e,
            a1e,
            a0m,
            a1m,
            aelp: 0.0,
            ap: 0.0,
            ticker_in: String::new(),
            ai: 0.0,
            aoe: 0.0,
            aom: 0.0,
            tbhp: 0.0,
            ttp: 0.0,
            tt: 0.0,
            tbh: 0.0,
            fee: 0.0,
            fee_percentage: 0.0,
            slippage,
            pool_share: 0.0,
            return_ticker0: String::new(),
            return_ticker1: String::new(),
            price_impact: 0.0,
            success: true,
        }
    }

    fn query_swap_result(&self, ticker_in: &str, amount_in: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        if ticker_in.is_empty() {
            log::error!("ticker_in should not be null");
            return AmmCalculateResult::new()
        }
        if amount_in <= 0.0 {
            log::error!("amount_in must be greater than 0");
            return AmmCalculateResult::new()
        }
        if slippage <= 0.0 || slippage >= 1.0 {
            log::error!("slippage must be in (0, 1)");
            return AmmCalculateResult::new()
        }
        if deadline <= 0 {
            log::error!("deadline must be greater than 0");
            return AmmCalculateResult::new()
        }

        let fees = self.calc_swap(ticker_in, amount_in);

        let ai = amount_in;
        let aoe = fees[5];
        let aom = aoe * (1.0 - slippage);
        let tbh = fees[1];
        let tt = fees[2];

        let mut fee = 0.0;
        if fees[3] > 0.0 || fees[4] > 0.0 {
            fee = fees[3] + fees[4];
        }

        let ticker_in_price_starting = fees[5] / fees[4];
        let ticker_in_price_end = fees[7] / fees[6];
        let price_impact = (ticker_in_price_end - ticker_in_price_starting) / ticker_in_price_starting;

        AmmCalculateResult {
            ticker0: self.ticker0.clone(),
            ticker0_decimals: self.ticker1_decimals,
            ticker1: self.ticker1.clone(),
            ticker1_decimals: self.ticker1_decimals,
            supply: self.supply,
            reserve0: self.reserve0,
            reserve1: self.reserve1,
            dl: deadline,
            a0e: 0.0,
            a1e: 0.0,
            a0m: 0.0,
            a1m: 0.0,
            aelp: 0.0,
            ap: 0.0,
            ticker_in: String::new(),
            ai: 0.0,
            aoe,
            aom,
            tbhp: 0.0,
            ttp: 0.0,
            tt,
            tbh,
            fee,
            fee_percentage: 0.0,
            slippage,
            pool_share: 0.0,
            return_ticker0: String::new(),
            return_ticker1: String::new(),
            price_impact,
            success: price_impact < slippage,
        }
    }

    fn sb2_mint(&mut self, address: &str, ticker: &str, value: f64) {
        self.wrapped_rune_contract.sb2_mint(address, ticker, value)
    }

    fn sb3_mint(&mut self, address: &str, ticker: &str, value: f64){
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
        match ticker {
            t if t == self.ticker0 => self.ticker1.clone(),
            t if t == self.ticker1 => self.ticker0.clone(),
            _ => "".to_string(),
        }
    }

    fn get_ticker_pair(&self) -> (String, String) {
        (self.ticker0.clone(), self.ticker1.clone())
    }
}

impl AutomatedLiquidityContract {

    pub fn new() -> Self {
        AutomatedLiquidityContract {
            ticker0: String::new(),
            ticker0_decimals: 0,
            ticker1: String::new(),
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
    pub fn do_swap(&mut self, address: &str, ticker_in: &str, amount_in: f64,
                   amount_out_min: f64, deadline: u64, block_timestamp: u64) -> ContractExecResult {
        if address.is_empty() {
            return validation_error("address cannot be empty");
        }
        if ticker_in.is_empty() {
            return validation_error("ticker_in cannot be empty");
        }
        if amount_in <= 0.0 {
            return validation_error("amount_in must be greater than 0");
        }
        if amount_out_min <= 0.0 {
            return validation_error("amount_out_min must be greater than 0");
        }
        if block_timestamp <= 0 {
            return validation_error("block_timestamp must be greater than 0");
        }
        if deadline <= block_timestamp {
            return validation_error("deadline must be greater than block_timestamp");
        }
        if self.reserve0 == 0.0 {
            return validation_error("reserve is empty.");
        }

        let service_fee_receiver = address;

        let is_ticker_in0 = ticker_in == self.ticker0;

        let [pay_in, lpfp_fee, sfp_fee, amount_out,
        reserve_in_start, reserve_out_start, reserve_in_end, reserve_out_end] = self.calc_swap(ticker_in.as_ref(), amount_in);

        if amount_out >= amount_out_min {
            if sfp_fee > 0.0 {
                self.wrapped_rune_contract.sb3_mint(service_fee_receiver, ticker_in, sfp_fee);
            }

            // Merge minting and transfer logic
            if is_ticker_in0 {
                if lpfp_fee > 0.0 {
                    self.reserve0_mint(lpfp_fee);
                }
                if pay_in > 0.0 {
                    self.reserve0_mint(pay_in);
                }
            } else {
                if lpfp_fee > 0.0 {
                    self.reserve1_mint(lpfp_fee);
                }
                if pay_in > 0.0 {
                    self.reserve0_mint(pay_in);
                }
            }

            if amount_out > 0.0 {
                self.reserve1_transfer(amount_out, address);
            }
        }
        ContractExecResult::Success("AMM swap execute successfully.".to_string())
    }

    /**
    * AMM add liquidity.
    */
    pub fn do_add_liquidity(&mut self, address: &str, amount0_desired: f64, amount1_desired: f64,
                            amount0_min: f64, amount1_min: f64, deadline: u64, block_timestamp: u64) -> ContractExecResult {
        if address.is_empty() {
            return ContractExecResult::ValidationError("address cannot be empty".to_string());
        }
        if amount0_desired <= 0.0 {
            return ContractExecResult::ValidationError("amount0_desired must be greater than 0".to_string());
        }
        if amount1_desired <= 0.0 {
            return ContractExecResult::ValidationError("amount1_desired must be greater than 0".to_string());
        }
        if amount0_min < 0.0 {
            return ContractExecResult::ValidationError("amount0_min must be greater than or equal to 0".to_string());
        }
        if amount1_min < 0.0 {
            return ContractExecResult::ValidationError("amount1_min must be greater than or equal to 0".to_string());
        }
        if block_timestamp <= 0 {
            return ContractExecResult::ValidationError("block_timestamp must be greater than 0".to_string());
        }
        if deadline <= block_timestamp {
            return ContractExecResult::ValidationError("deadline must be greater than block_timestamp".to_string());
        }

        let [amount0, amount1] = self.calc_token_amounts_for_add_liquidity(amount0_desired, amount1_desired);
        if amount0 < amount0_min || amount1 < amount1_min {
            return ContractExecResult::ValidationError("calculated amounts are less than the minimum required amounts".to_string());
        }

        let [lp_amount_user, lp_amount_block_hole] = self.calc_lp_amounts_for_add_liquidity(amount0, amount1);

        self.reserve0_mint(amount0);
        self.reserve1_mint(amount1);

        self.wrapped_rune_contract.sb2_mint(address, self.wrapped_rune_contract.get_myself_ticker().as_ref(), lp_amount_user);
        self.supply += lp_amount_user;

        if lp_amount_block_hole > 0.0 {
            self.wrapped_rune_contract.sb2_mint("mxxD8soCEyzP3ZJReqKUPsTYC7ZCfeLsvo".to_string().as_ref(), self.wrapped_rune_contract.get_myself_ticker().as_ref(), lp_amount_block_hole);
            self.supply += lp_amount_block_hole;
        }
        ContractExecResult::Success("AMM add liquidity execute successfully.".to_string())
    }

    /**
     * AMM remove liquidity.
     */
    pub fn do_remove_liquidity(&mut self, address: &str, lp_amount: f64, amount0_min: f64,
                               amount1_min: f64, deadline: u64, block_timestamp: u64) -> ContractExecResult {
        if address.is_empty() {
            return ContractExecResult::ValidationError("address cannot be empty".to_string());
        }
        if lp_amount <= 0.0 {
            return ContractExecResult::ValidationError("lp_amount must be greater than 0".to_string());
        }
        if amount0_min < 0.0 {
            return ContractExecResult::ValidationError("amount0_min must be greater than or equal to 0".to_string());
        }
        if amount1_min < 0.0 {
            return ContractExecResult::ValidationError("amount1_min must be greater than or equal to 0".to_string());
        }
        if block_timestamp <= 0 {
            return ContractExecResult::ValidationError("block_timestamp must be greater than 0".to_string());
        }
        if deadline <= block_timestamp {
            return ContractExecResult::ValidationError("deadline must be greater than block_timestamp".to_string());
        }
        if self.reserve0 == 0.0 {
            return ContractExecResult::ValidationError("reserve pool is empty.".to_string());
        }

        let amounts = self.calc_token_amount_for_remove_liquidity(lp_amount);

        let amount0 = amounts[0];
        let amount1 = amounts[1];
        if amount0 < amount0_min || amount1 < amount1_min {
            return ContractExecResult::ValidationError("amount0 and amount1 should be greater than amount0_min and amount1_min, respectively.".to_string());
        }

        self.supply = self.supply - lp_amount;
        self.reserve0_transfer(amount0, address);
        self.reserve1_transfer(amount1, address);

        self.sb2_burn(address, self.wrapped_rune_contract.get_myself_ticker().as_str(), lp_amount);

        ContractExecResult::Success("AMM remove liquidity execute successfully.".to_string())
    }

    pub fn update_reserve0(&mut self, new_value: f64) {
        self.reserve0 = new_value;
    }

    pub fn reserve0_mint(&mut self, value: f64) {
        if value <= 0.0 {
            log::error!("Value must be greater than 0.");
            return
        }
        self.update_reserve0(self.reserve0 + value);
    }

    pub fn reserve0_transfer(&mut self, value: f64, address: &str) {
        if value <= 0.0 || address.is_empty() {
            log::error!("Value must be greater than 0, or address must not be empty.");
            return;
        }
        if self.reserve0 < value {
            log::error!("Insufficient reserve0 balance.");
            return;
        }
        self.update_reserve0(self.reserve0 - value);
        //self.wrapped_rune_contract.sb3_mint(address, self.ticker0.clone(), value)
    }

    pub fn update_reserve1(&mut self, new_value: f64) {
        self.reserve1 = new_value;
    }

    pub fn reserve1_mint(&mut self, value: f64) {
        if value <= 0.0 {
            log::error!("Value must be greater than 0.");
            return;
        }
        self.update_reserve1(self.reserve1 + value);
    }

    pub fn reserve1_transfer(&mut self, value: f64, address: &str) {
        if value <= 0.0 || address.is_empty() {
            log::error!("Value must be greater than 0, or address must not be empty.");
            return;
        }
        if self.reserve1 < value {
            log::error!("Insufficient reserve0 balance.");
            return;
        }
        self.update_reserve1(self.reserve1 - value);
    }

    pub fn calc_swap(&self, ticker_in: &str, amount_in: f64) -> [f64; 8] {
        if ticker_in.is_empty() || amount_in <= 0.0 {
            log::error!("ticker_in should not be null or amount_in must be greater than 0");
            return [0.0; 8]
        }

        let is_ticker_in0 = ticker_in == self.ticker0;

        let (reserve_in, reserve_out, reserve_in_start, reserve_out_start) = if is_ticker_in0 {
            (self.reserve0, self.reserve1, self.reserve0, self.reserve1)
        } else {
            (self.reserve1, self.reserve0, self.reserve1, self.reserve0)
        };

        let trading = self.wrapped_rune_contract.trading.as_ref().unwrap();
        let lp_fee_percentage = trading.lp_fee_percentage.unwrap_or(0);
        let service_fee_percentage = trading.service_fee_percentage.unwrap_or(0);

        let lpfp_fee = self.calculate_fee(amount_in, lp_fee_percentage);
        let sfp_fee = self.calculate_fee(amount_in, service_fee_percentage);

        let pay_in = amount_in - lpfp_fee - sfp_fee;

        let temp1 = reserve_in * reserve_out;
        let temp2 = reserve_in + pay_in;
        let temp3 = temp1 / temp2;

        let amount_out = reserve_out - temp3;

        // Update reserve.
        let (reserve_in_end, reserve_out_end) = if is_ticker_in0 {
            (pay_in + self.reserve0, self.reserve1 - amount_out)
        } else {
            (pay_in + self.reserve1, self.reserve0 - amount_out)
        };

        [
            pay_in,
            lpfp_fee,
            sfp_fee,
            amount_out,
            reserve_in_start,
            reserve_out_start,
            reserve_in_end,
            reserve_out_end,
        ]
    }

    pub fn calc_token_amounts_for_add_liquidity(&self, amount0_desired: f64, amount1_desired: f64) -> [f64; 2] {
        if amount0_desired <= 0.0 || amount1_desired <= 0.0 {
            log::error!("amount0_desired and amount1_desired must be greater than 0");
            return [0.0, 0.0]
        }
        if self.reserve0 == 0.0 {
            return [amount0_desired, amount1_desired];
        }

        let temp1 = self.reserve1 / self.reserve0;
        let amount1_optimal = temp1 * amount0_desired;

        if amount1_optimal <= amount1_desired {
            [amount0_desired, amount1_optimal]
        } else {
            let temp2 = self.reserve0 / self.reserve1;
            let amount0_optimal = temp2 * amount1_desired;
            [amount0_optimal, amount1_desired]
        }
    }

    pub fn calc_lp_amounts_for_add_liquidity(&self, amount0: f64, amount1: f64) -> [f64; 2] {
        let lp_amount_block_hole = 1e-15;

        if self.reserve0 == 0.0 {
            let temp3 = amount0 * amount1;
            let temp4 = temp3.sqrt();
            let lp_amount_user = temp4 - lp_amount_block_hole;
            [lp_amount_user, lp_amount_block_hole]
        } else {
            let lp_starting = self.supply;
            let temp5 = amount0 / self.reserve0;
            let lp_amount_user1 = temp5 * lp_starting;
            let temp6 = amount1 / self.reserve1;
            let lp_amount_user2 = temp6 * lp_starting;

            let lp_amount_user = if lp_amount_user1 > lp_amount_user2 {
                lp_amount_user2
            } else {
                lp_amount_user1
            };
            [lp_amount_user, lp_amount_block_hole]
        }
    }

    pub fn calc_token_amount_for_remove_liquidity(&self, lp_amount: f64) -> [f64; 2] {
        if lp_amount <= 0.0 {
            log::error!("lp_amount must be greater than 0.");
            return [0.0, 0.0]
        }
        if self.reserve0 == 0.0 {
            log::error!("pool is empty.");
            return [0.0, 0.0]
        }
        let temp1 = lp_amount / self.supply;
        let amount0 = temp1 * self.reserve0;
        let amount1 = temp1 * self.reserve1;
        [amount0, amount1]
    }

    fn calculate_fee(&self, amount_in: f64, percentage: u32) -> f64 {
        if percentage > 0 {
            amount_in * (percentage as f64)
        } else {
            0.0
        }
    }
}

pub fn validation_error(message: &str) -> ContractExecResult {
    ContractExecResult::ValidationError(message.to_string())
}

#[derive(Serialize, Deserialize)]
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
            ticker_in: String::new(),
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
            return_ticker0: String::new(),
            return_ticker1: String::new(),
            price_impact: 0.0,
            success: false,
        }
    }
}