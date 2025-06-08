use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use bitcoin::{Address, OutPoint, Transaction, Txid};
use bitcoin::Network::Bitcoin;
use rand::Rng;
use serde::Serialize;
use ordinals::{opi_log, Edict, Etching, RuneId, Runestone};
use ordinals::burn::{Burn2, Burn3};
use ordinals::mint::{Mint2, Mint3};
use crate::amm::{AmmCalculateResult, AutomatedLiquidityContract};
use crate::context::{OperateContext};
use crate::contract::{BaseContract, Contract, ContractExecResult, ContractTemplate, ContractValidator, WrappedRuneContract};
use crate::state::State;
use std::default::Default;

#[derive(Clone)]
pub struct RunesStateMachine {
    pub rsm_interpreter: RSMInterpreter,
}

impl RunesStateMachine {

    pub fn new() -> Self {
        RunesStateMachine {
            rsm_interpreter: RSMInterpreter::new(),
        }
    }

    pub fn init_contract(&mut self, block: u64, tx: u32, etching: &Etching, chain: &str){
        match ContractTemplate::from_u8(etching.contract.unwrap_or_default()) {
            Some(ContractTemplate::Governance) => {
                if ContractValidator::can_init(block, chain, ContractTemplate::Governance).is_ok() {
                    return println!("cannot init Governance contract at block {}", block)
                }
                println!("Unsupported contract!");
            }
            Some(ContractTemplate::AMM) => {
                if ContractValidator::can_init(block, chain, ContractTemplate::AMM).is_ok() {
                    return println!("cannot init AMM contract at block {}", block)
                }
                if let Some(RuneId { block, tx }) = etching.parent {
                    let key = format!("{}:{}", block, tx);
                    if !self.rsm_interpreter.contracts.contains_key(&key) {
                        return println!("Parent contract is not exist!");
                    }
                }

                let contract_id = format!("{}:{}", block, tx);

                let amm_liquidity_contract = Arc::new(RwLock::new(AutomatedLiquidityContract {
                    ticker0: etching.burn3able_rune_ids.0.unwrap().to_string(),
                    ticker0_decimals: 0,
                    ticker1: etching.burn3able_rune_ids.1.unwrap().to_string(),
                    ticker1_decimals: 0,
                    supply: 0.0,
                    reserve0: 0.0,
                    reserve1: 0.0,
                    wrapped_rune_contract: WrappedRuneContract {
                        parent: etching.parent,
                        myself: RuneId{ block, tx },
                        rune: etching.rune.unwrap(),
                        contract: ContractTemplate::AMM.to_u8(),
                        burn3able_rune_ids: etching.burn3able_rune_ids,
                        trading: etching.trading,
                        sba2: Default::default(),
                        sba3: Default::default(),
                        sb2: Default::default(),
                        sb3: Default::default(),
                    },
                }));

                self.rsm_interpreter.contracts.insert(contract_id, amm_liquidity_contract);
            }
            Some(ContractTemplate::Staking) => {
                if ContractValidator::can_init(block, chain, ContractTemplate::Staking).is_ok() {
                    return println!("cannot init Staking contract at block {}", block)
                }
                println!("Unsupported contract!");
            }
            Some(ContractTemplate::Stablecoin) => {
                if ContractValidator::can_init(block, chain, ContractTemplate::Stablecoin).is_ok() {
                    return println!("cannot init Stablecoin contract at block {}", block)
                }
                println!("Unsupported contract!");
            }
            Some(ContractTemplate::Lending) => {
                if ContractValidator::can_init(block, chain, ContractTemplate::Lending).is_ok() {
                    return println!("cannot init Lending contract at block {}", block)
                }
                println!("Unsupported contract!");
            }
            _ => {
                if let Some(RuneId { block, tx }) = etching.parent {
                    let key = format!("{}:{}", block, tx);
                    if !self.rsm_interpreter.contracts.contains_key(&key) {
                        return println!("Parent contract is not exist!");
                    }
                }

                let contract_id = format!("{}:{}", block, tx);

                let base_contract = Arc::new(RwLock::new(BaseContract {
                    wrapped_rune_contract: WrappedRuneContract {
                        parent: etching.parent,
                        myself: RuneId{ block, tx },
                        rune: etching.rune.unwrap_or_default(),
                        contract: ContractTemplate::Base.to_u8(),
                        burn3able_rune_ids: (None, None),
                        trading: None,
                        sba2: Default::default(),
                        sba3: Default::default(),
                        sb2: Default::default(),
                        sb3: Default::default(),
                    },
                }));

                self.rsm_interpreter.contracts.insert(contract_id, base_contract);
            }
        }
    }

    pub fn invoke_contract_event(&mut self, runestone: &Runestone, transaction: &Transaction, block_height: u64, block_time: u32, tx: u32, tx_id: Txid, chain: &str) {
        // cmd;<height>;tx_events_mint2;<txid>;<stf>;<outpoint>;<fromRuneId>;<id>;<amount>;<scriptpubkey>
        // cmd;<height>;tx_events_mint3;<txid>;<stf>;<outpoint>;<fromRuneId>;<id>;<amount>;<scriptpubkey>
        // cmd;<height>;tx_events_burn2;<txid>;<stf>;<outpoint>;<toRuneId>;<id>;<amount>;<scriptpubkey>
        // cmd;<height>;tx_events_burn3;<txid>;<stf>;<outpoint>;<toRuneId>;<id>;<amount>;<scriptpubkey>
        let log_result = |chain: &str, operate_event: &str, stf: &str, out_point: u32, amount: u128, script_pub_key: &str, to_contract_id: &str| {
            opi_log::log_to_file(chain, format!("cmd;{0};{1};{2};{3};{4};{5};{6};{7}", block_height, operate_event, tx, stf, out_point, format!("{}:{}", block_height, tx), amount, script_pub_key),
                                 false, block_height as u32, &mut false);
        };

        //Etching
        if let Some(etching) = &runestone.etching {
            self.init_contract(block_height, tx, etching, chain);
            log_result(chain, "tx_events_init_contract", "init_contract", 0, 0, "", format!("{}:{}", block_height, tx).as_str());
        }

        //Burn2
        if let Some(burn2) = &runestone.burn2s {
            let context = OperateContext {
                block_height,
                block_timestamp: block_time.into(),
                tx,
                tx_id,
                rune_id: burn2.to,
                transaction: transaction.clone(),
                mint2s: None,
                mint3s: None,
                burn2s: Some(burn2.clone()),
                burn3s: None,
                address: Self::extract_burn2_address(burn2, transaction),
            };

            let stf_name = StateTransitionName::from_u32(burn2.state_transition_function);
            self.rsm_interpreter.execute(burn2.to.to_str(), stf_name.clone().unwrap(), &context);

            if StateTransitionName::from(burn2.state_transition_function) == StateTransitionName::Rl && !burn2.edicts.is_empty() {
                for edict in &burn2.edicts {
                    if let Some(addr) = Self::extract_edict_address(edict, transaction) {
                        self.rsm_interpreter.execute_burn2_remove_liquidity_edict(burn2.to.to_str().as_str(), addr.clone(), edict);
                        log_result(chain, "tx_events_burn2", stf_name.as_ref().unwrap().as_str(), edict.output,  edict.amount, addr.clone().to_string().as_str(), burn2.to.to_str().as_str());
                    }
                }
            }
        }

        //Burn3
        if let Some(burn3) = &runestone.burn3s {
            let context = OperateContext {
                block_height,
                block_timestamp: block_time.into(),
                tx,
                tx_id,
                rune_id: burn3.to,
                transaction: transaction.clone(),
                mint2s: None,
                mint3s: None,
                burn2s: None,
                burn3s: Some(burn3.clone()),
                address: Self::extract_burn3_address(burn3, transaction),
            };

            let stf_name = StateTransitionName::from_u32(burn3.state_transition_function);
            self.rsm_interpreter.execute(burn3.to.to_str(), stf_name.clone().unwrap(), &context);

            match StateTransitionName::from(burn3.state_transition_function) {
                StateTransitionName::Al | StateTransitionName::Sw if !burn3.edicts.is_empty() => {
                    for edict in &burn3.edicts {
                        if let Some(addr) = Self::extract_edict_address(edict, transaction) {
                            match StateTransitionName::from(burn3.state_transition_function) {
                                StateTransitionName::Al =>
                                    {
                                        self.rsm_interpreter.execute_burn3_add_liquidity_edict(burn3.to.to_str().as_str(), addr.clone(), edict);
                                        log_result(chain, "tx_events_burn3", stf_name.as_ref().unwrap().as_str(), edict.output,  edict.amount, addr.clone().to_string().as_str(), burn3.to.to_str().as_str());
                                    },
                                StateTransitionName::Sw =>
                                    {
                                        self.rsm_interpreter.execute_burn3_swap_edict(burn3.to.to_str().as_str(), addr.clone(), edict);
                                        log_result(chain, "tx_events_burn3", stf_name.as_ref().unwrap().as_str(), edict.output,  edict.amount, addr.clone().to_string().as_str(), burn3.to.to_str().as_str());
                                    },
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        //Mint2
        if let Some(mint2s) = &runestone.mint2s {
            for mint2 in mint2s {
                let operate_context = OperateContext {
                    block_height,
                    block_timestamp: block_time.into(),
                    tx,
                    tx_id,
                    rune_id: mint2.from,
                    transaction: transaction.clone(),
                    mint2s: Some(mint2s.clone()),
                    mint3s: None,
                    burn2s: None,
                    burn3s: None,
                    address: Self::extract_mint2_address(mint2, transaction),
                };

                if let Some(stf) = mint2.state_transition_function {
                    let stf_name = StateTransitionName::from_u32(stf);
                    self.rsm_interpreter.execute(mint2.from.to_str(), stf_name.clone().unwrap(), &operate_context);


                    if StateTransitionName::from(stf) == StateTransitionName::W2 {
                        if let Some(edicts) = &mint2.edicts {
                            for edict in edicts {
                                if let Some(addr) = Self::extract_edict_address(edict, transaction) {
                                    self.rsm_interpreter.execute_mint2_w2_edict(addr.clone(), edict);
                                    log_result(chain, "tx_events_mint2", stf_name.as_ref().unwrap().as_str(), edict.output,  edict.amount, addr.clone().to_string().as_str(), mint2.from.to_str().as_str());
                                }
                            }
                        }
                    }
                }
            }
        }

        //Mint3
        if let Some(mint3s) = &runestone.mint3s {
            for mint3 in mint3s {
                let operate_context = OperateContext {
                    block_height,
                    block_timestamp: u64::from(block_time),
                    tx,
                    tx_id,
                    rune_id: mint3.from,
                    transaction: transaction.clone(),
                    mint2s: None,
                    mint3s: Option::from(mint3s.clone()),
                    burn2s: None,
                    burn3s: None,
                    address: Self::extract_mint3_address(mint3, transaction),
                };

                if let Some(stf) = mint3.state_transition_function {
                    let stf_name = StateTransitionName::from_u32(stf);
                    self.rsm_interpreter.execute(mint3.from.to_str(), stf_name.clone().unwrap(), &operate_context);

                    if StateTransitionName::from(stf) == StateTransitionName::W3 {
                        if let Some(edicts) = &mint3.edicts {
                            for edict in edicts {
                                if let Some(addr) = Self::extract_edict_address(edict, transaction) {
                                    self.rsm_interpreter.execute_mint3_w3_edict(addr.clone(), edict);
                                    log_result(chain, "tx_events_mint3", stf_name.as_ref().unwrap().as_str(), edict.output,  edict.amount, addr.clone().to_string().as_str(), mint3.from.to_str().as_str());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn extract_mint2_address(mint2: &Mint2, transaction: &Transaction) -> Option<Address> {
        if let Some(edicts) = &mint2.edicts {
            let tx_out = &transaction.output[edicts[0].output as usize];
            let script_pub_key = &tx_out.script_pubkey;
            return Address::from_script(script_pub_key, Bitcoin).ok();
        }
        None
    }

    fn extract_mint3_address(mint3: &Mint3, transaction: &Transaction) -> Option<Address> {
        if let Some(edicts) = &mint3.edicts {
            let tx_out = &transaction.output[edicts[0].output as usize];
            let script_pub_key = &tx_out.script_pubkey;
            return Address::from_script(script_pub_key, Bitcoin).ok();
        }
        None
    }

    fn extract_burn2_address(burn2: &Burn2, transaction: &Transaction) -> Option<Address> {
        let edicts = &burn2.edicts;
        let tx_out = &transaction.output[edicts[0].output as usize];
        let script_pub_key = &tx_out.script_pubkey;
        Address::from_script(script_pub_key, Bitcoin).ok()
    }

    fn extract_burn3_address(burn3: &Burn3, transaction: &Transaction) -> Option<Address> {
        let edicts = &burn3.edicts;
        let tx_out = &transaction.output[edicts[0].output as usize];
        let script_pub_key = &tx_out.script_pubkey;
        Address::from_script(script_pub_key, Bitcoin).ok()
    }

    fn extract_edict_address(edict: &Edict, transaction: &Transaction) -> Option<Address> {
        let tx_out = &transaction.output[edict.output as usize];
        let script_pub_key = &tx_out.script_pubkey;
        Address::from_script(script_pub_key, Bitcoin).ok()
    }

    pub fn query_add_liquidity_result(&self, contract_id: &str, a0e: f64, a1e: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        self.rsm_interpreter.query_add_liquidity_result(contract_id, a0e, a1e, slippage, deadline)
    }

    pub fn query_remove_liquidity_result(&self, contract_id: &str, lp_amount: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        self.rsm_interpreter.query_remove_liquidity_result(contract_id, lp_amount, slippage, deadline)
    }

    pub fn query_swap_result(&self, contract_id: &str, ticker_in: &str, amount_in: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        self.rsm_interpreter.query_swap_result(contract_id, ticker_in, amount_in, slippage, deadline)
    }

    pub fn get_state(&self, contract_id: &str, state_name: State, address: &str, ticker: &str) -> f64 {
        self.rsm_interpreter.get_state(contract_id, state_name, address, ticker)
    }

    pub fn get_contract_info(&self, contract_id: &str) -> String {
        self.rsm_interpreter.get_contract_info(contract_id)
    }
}

pub struct RSMInterpreter {
    pub jump_table: JumpTable,
    pub contracts: HashMap<String, Arc<RwLock<dyn Contract>>>,
}

impl Clone for RSMInterpreter {
    fn clone(&self) -> Self {
        Self {
            jump_table: self.jump_table.clone(),
            contracts: self.contracts
                .iter().map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }
    }
}

impl RSMInterpreter {
    pub fn new() -> Self {
        RSMInterpreter {
            jump_table: JumpTable::new(),
            contracts: Default::default(),
        }
    }

    pub fn execute(&mut self, contract_id: String, function_name: String, operate_context: &OperateContext) -> ContractExecResult {
        if let Some(contract) = self.contracts.get_mut(&contract_id) {
            if let Some(operation) = self.jump_table.operations.get(&function_name) {
                 let mut guard = contract.write().unwrap();
                 let _ = (operation.method)(&mut *guard, operate_context);
            }
            ContractExecResult::Success("RSM invoke contract successfully.".to_string())
        } else {
            ContractExecResult::UnknownException("Contract not found.".to_string())
        }
    }

    pub fn execute_mint2_w2_edict(&mut self, user_address: Address, edict: &Edict){
        let from_contract_id = edict.id.to_str();
        if let Some(contract) = self.contracts.get_mut(&from_contract_id) {
            let mut guard = contract.write().unwrap();
            guard.sb2_mint(user_address.to_string().as_str(), edict.id.to_str().as_str(), edict.amount as f64);
        }
    }

    pub fn execute_mint3_w3_edict(&mut self, user_address: Address, edict: &Edict){
        let from_contract_id = edict.id.to_str();
        if let Some(contract) = self.contracts.get_mut(&from_contract_id) {
            let mut guard = contract.write().unwrap();
            guard.sb3_mint(user_address.to_string().as_str(), edict.id.to_str().as_str(), edict.amount as f64);
        }
    }

    pub fn execute_burn2_remove_liquidity_edict(&mut self, to_contract_id: &str, user_address: Address, edict: &Edict) {
        let user_address = user_address.to_string();

        let (ticker0, ticker1) = {
            let to_contract = self.contracts.get_mut(to_contract_id)
                .unwrap_or_else(|| panic!("Contract {} not found", to_contract_id));
            let guard = to_contract.write().unwrap();
            guard.get_ticker_pair()
        };

        if let Some(contract) = self.contracts.get_mut(&ticker0) {
            let mut guard = contract.write().unwrap();
            guard.sb3_mint(user_address.as_str(), &ticker0, edict.amount as f64);
            guard.sb3_burn(to_contract_id, &ticker0, edict.amount as f64);
        }

        if let Some(contract) = self.contracts.get_mut(&ticker1) {
            let mut guard = contract.write().unwrap();
            guard.sb3_mint(user_address.as_str(), &ticker1, edict.amount as f64);
            guard.sb3_burn(to_contract_id, &ticker1, edict.amount as f64);
        }
    }

    pub fn execute_burn3_add_liquidity_edict(&mut self, to_contract_id: &str, user_address: Address, edict: &Edict) {
        let from_contract_id = edict.id.to_str();
        if let Some(contract) = self.contracts.get_mut(&from_contract_id) {
            let mut guard = contract.write().unwrap();
            guard.sb3_mint(to_contract_id, edict.id.to_str().as_str(), edict.amount as f64);
        }
    }

    pub fn execute_burn3_swap_edict(&mut self, to_contract_id: &str, user_address: Address, edict: &Edict) {
        let swap_b_contract_id = {
            let to_contract = match self.contracts.get_mut(to_contract_id) {
                Some(c) => c,
                None => return,
            };
            let id = to_contract.write().unwrap().get_another_ticker(edict.id.to_str().as_str());
            id
        };

        if let Some(swap_b_contract) = self.contracts.get_mut(&swap_b_contract_id) {
            let mut swap_guard = swap_b_contract.write().unwrap();
            swap_guard.sb3_mint(
                user_address.to_string().as_str(),
                edict.id.to_str().as_str(),
                edict.amount as f64,
            );
            swap_guard.sb3_burn(
                to_contract_id,
                edict.id.to_str().as_str(),
                edict.amount as f64,
            );
        }

        let from_contract_id = edict.id.to_str();
        if let Some(from_contract) = self.contracts.get_mut(&from_contract_id) {
            let mut guard = from_contract.write().unwrap();
            guard.sb3_mint(to_contract_id, edict.id.to_str().as_str(), edict.amount as f64);
        }
    }

    pub fn register_contract_app(&mut self, contract_app_id: &str, contract_app: Arc<RwLock<dyn Contract>>) {
        self.contracts.insert(contract_app_id.to_string(), contract_app);
    }

    pub fn register_operation(&mut self, key: &str, operation: Operation) {
        self.jump_table.register_operation(key.to_string(), operation);
    }

    pub fn get_contract_state_snapshot(&self, contract_id: &str) -> String {
        if let Some(contract) = self.contracts.get(contract_id) {
            contract.read().unwrap().dump_state()
        } else {
            String::new()
        }
    }

    pub fn get_state(&self, contract_id: &str, state_name: State, address: &str, ticker: &str) -> f64 {
        if let Some(contract) = self.contracts.get(contract_id) {
            contract.read().unwrap().get_state(state_name, address, ticker)
        } else {
            0.0
        }
    }

    pub fn get_contract_info(&self, contract_id: &str) -> String {
        if let Some(contract) = self.contracts.get(contract_id) {
            contract.read().unwrap().get_info()
        } else {
            String::new()
        }
    }

    pub fn query_add_liquidity_result(&self, contract_id: &str, a0e: f64, a1e: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        if let Some(contract) = self.contracts.get(contract_id) {
            contract.read().unwrap().query_add_liquidity_result(a0e, a1e, slippage, deadline)
        } else {
            AmmCalculateResult::new()
        }
    }

    pub fn query_remove_liquidity_result(&self, contract_id: &str, lp_amount: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        if let Some(contract) = self.contracts.get(contract_id) {
            contract.read().unwrap().query_remove_liquidity_result(lp_amount, slippage, deadline)
        } else {
            AmmCalculateResult::new()
        }
    }

    pub fn query_swap_result(&self, contract_id: &str, ticker_in: &str, amount_in: f64, slippage: f64, deadline: u64) -> AmmCalculateResult {
        if let Some(contract) = self.contracts.get(contract_id) {
            contract.read().unwrap().query_swap_result(ticker_in, amount_in, slippage, deadline)
        } else {
            AmmCalculateResult::new()
        }
    }
}

pub struct JumpTable {
    pub operations: HashMap<String, Operation>,
}

impl Clone for JumpTable {
    fn clone(&self) -> Self {
        let new_ops = self.operations
            .iter()
            .filter(|(k, _)| !k.starts_with("debug_"))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        JumpTable {
            operations: new_ops
        }
    }
}

impl JumpTable {
    fn new() -> Self {
        let mut operations = HashMap::new();

        operations.insert("al".to_string(), Operation{
            function_name: "al".to_string(),
            method: |contract, context| {
                let amm_contract = contract.as_any_mut().downcast_mut::<AutomatedLiquidityContract>().unwrap();
                amm_contract.add_liquidity(context)
            } });
        operations.insert("rl".to_string(), Operation{
            function_name: "rl".to_string(),
            method: |contract, context| {
                let amm_contract = contract.as_any_mut().downcast_mut::<AutomatedLiquidityContract>().unwrap();
                amm_contract.remove_liquidity(context)
            }});
        operations.insert("sw".to_string(), Operation{
            function_name: "sw".to_string(),
            method: |contract, context| {
                let amm_contract = contract.as_any_mut().downcast_mut::<AutomatedLiquidityContract>().unwrap();
                amm_contract.swap(context)
            } });

        JumpTable {
            operations,
        }
    }

    pub fn get_operation(&self, key: String) -> Option<Operation> {
        self.operations.get(&key).cloned()
    }

    pub fn register_operation(&mut self, key: String, operation: Operation) {
        self.operations.insert(key, operation);
    }
}

#[derive(Clone)]
pub struct Operation {
    pub function_name: String,
    pub method: fn(&mut dyn Contract, operate_context: &OperateContext) -> Result<(), ContractExecResult>,
}

#[derive(PartialEq)]
pub enum StateTransitionName {
    Ic = 0,
    Al = 1,
    Rl = 2,
    Sw = 3,
    W2 = 4,
    W3 = 5,
    Unknown = 6,
}

impl StateTransitionName {
    pub fn from_u32(value: u32) -> Option<String> {
        match value {
            0 => Some(String::from("ic")),
            1 => Some(String::from("al")),
            2 => Some(String::from("rl")),
            3 => Some(String::from("sw")),
            4 => Some(String::from("w2")),
            5 => Some(String::from("w3")),
            _ => None,
        }
    }

    pub fn from(code: u32) -> Self {
        match code {
            0 => Self::Ic,
            1 => Self::Al,
            2 => Self::Rl,
            3 => Self::Sw,
            4 => Self::W2,
            5 => Self::W3,
            _ => Self::Unknown,
        }
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Serialize)]
pub struct InvokeResult {
    pub height: u64,
    pub operate_event: String,
    pub tx_id: Txid,
    pub stf: String,
    pub outpoint: OutPoint,
    pub to_contract_id: String,
    pub id: RuneId,
    pub amount: u128,
    pub script_pub_key_hex: String,
}

#[derive(Serialize)]
struct RecoveryLog {
    runestone: Runestone,
    transaction: Transaction,
    block_height: u64,
    block_time: u32,
    tx: u32,
    tx_id: Txid,
}

fn generate_recovery_id(block_height: u64, block_time: u32, tx: u32) -> String {
    let mut rng = rand::rng();
    let random_number: u64 = rng.random();
    let recovery_id = format!("{}_{}_{}_{}", block_height, block_time, tx, random_number);
    recovery_id
}

fn generate_recovery_log(runestone: Runestone, transaction: Transaction, block_height: u64, block_time: u32, tx: u32, tx_id: Txid) -> String {
    let log = RecoveryLog {
        runestone,
        transaction,
        block_height,
        block_time,
        tx,
        tx_id,
    };
    serde_json::to_string(&log).unwrap()
}