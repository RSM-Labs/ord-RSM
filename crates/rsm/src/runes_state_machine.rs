use std::collections::HashMap;
use bitcoin::{Address, Transaction, Txid};
use bitcoin::Network::Bitcoin;
use ordinals::{Etching, RuneId, Runestone};
use ordinals::burn::{Burn2, Burn3};
use ordinals::mint::{Mint2, Mint3};
use crate::amm::AutomatedLiquidityContract;
use crate::context::{OperateContext};
use crate::contract::{Contract, ContractExecResult, ContractTemplate, WrappedRuneContract};
use crate::state::StateDB;

pub struct RunesStateMachine {
    pub state_db: StateDB,
    pub rsm_interpreter: RSMInterpreter,
}

impl RunesStateMachine {

    pub fn new() -> Self {
        RunesStateMachine {
            state_db: Default::default(),
            rsm_interpreter: RSMInterpreter::new(),
        }
    }

    pub fn init_contract(&mut self, block: u64, tx: u32, etching: &Etching){
        match ContractTemplate::from_u8(etching.contract.unwrap()) {
            Some(ContractTemplate::Governance) => {
                println!("Unsupported contract!");
            }
            Some(ContractTemplate::AMM) => {
                if let (Some(block), Some(tx)) = etching.parent {
                    let key = format!("{}:{}", block, tx);
                    if !self.rsm_interpreter.contracts.contains_key(&key) {
                        return println!("Parent contract is not exist!");
                    }
                }

                let contract_id = etching.rune.unwrap().to_string();

                let amm_liquidity_contract = Box::new(AutomatedLiquidityContract {
                    ticker0: etching.burn3_able_rune_ids.0.unwrap().to_string(),
                    ticker0_decimals: 0,
                    ticker1: etching.burn3_able_rune_ids.1.unwrap().to_string(),
                    ticker1_decimals: 0,
                    supply: 0.0,
                    reserve0: 0.0,
                    reserve1: 0.0,
                    wrapped_rune_contract: WrappedRuneContract {
                        parent: RuneId{ block: etching.parent.0.unwrap(), tx: etching.parent.1.unwrap() },
                        myself: RuneId{ block, tx },
                        rune: etching.rune.unwrap(),
                        contract: etching.contract.unwrap_or_default(),
                        mint2_amount: 0,
                        burn3_able_rune_ids: etching.burn3_able_rune_ids,
                        trading: etching.trading,
                        dao: etching.dao,
                        sba2: Default::default(),
                        sba3: Default::default(),
                        sb2: Default::default(),
                        sb3: Default::default(),
                    },
                });

                self.rsm_interpreter.contracts.insert(contract_id, amm_liquidity_contract);
            }
            Some(ContractTemplate::Staking) => {
                println!("Unsupported contract!");
            }
            Some(ContractTemplate::Stablecoin) => {
                println!("Unsupported contract!");
            }
            Some(ContractTemplate::Lending) => {
                println!("Unsupported contract!");
            }
            _ => {
                println!("Unsupported contract!");
            }
        }
    }

    pub fn invoke_contract_event(&mut self, runestone: &Runestone, transaction: &Transaction, block_height: u64, block_time: u32, tx: u32, tx_id: Txid){

        if let Some(etching) = &runestone.etching {
            self.init_contract(block_height, tx, etching);
        }

        if let Some(burn2) = &runestone.burn2s {
            let operate_context = OperateContext {
                block_height,
                block_timestamp: u64::from(block_time),
                tx,
                tx_id,
                rune_id: Default::default(),
                transaction: transaction.clone(),
                mint2s: None,
                mint3s: None,
                burn2s: Option::from(burn2.clone()),
                burn3s: None,
                address: Self::extract_burn2_address(burn2, transaction),
            };
            self.rsm_interpreter.execute(format!("{}:{}", block_height, tx), burn2.state_transition_function.to_string(), &operate_context);
        }

        if let Some(burn3) = &runestone.burn3s {
            let operate_context = OperateContext {
                block_height,
                block_timestamp: u64::from(block_time),
                tx,
                tx_id,
                rune_id: Default::default(),
                transaction: transaction.clone(),
                mint2s: None,
                mint3s: None,
                burn2s: None,
                burn3s: Option::from(burn3.clone()),
                address: Self::extract_burn3_address(burn3, transaction),
            };
            self.rsm_interpreter.execute(format!("{}:{}", block_height, tx), StateTransitionName::from_u32(burn3.state_transition_function).unwrap(), &operate_context);
        }

        if let Some(mint2s) = &runestone.mint2s {
            for mint2 in mint2s {
                let operate_context = OperateContext {
                    block_height,
                    block_timestamp: u64::from(block_time),
                    tx,
                    tx_id,
                    rune_id: Default::default(),
                    transaction: transaction.clone(),
                    mint2s: Option::from(mint2s.clone()),
                    mint3s: None,
                    burn2s: None,
                    burn3s: None,
                    address: Self::extract_mint2_address(mint2, transaction),
                };
                self.rsm_interpreter.execute(format!("{}:{}", block_height, tx), StateTransitionName::from_u32(mint2.state_transition_function.unwrap()).unwrap(), &operate_context);
            }
        }

        if let Some(mint3s) = &runestone.mint3s {
            for mint3 in mint3s {
                let operate_context = OperateContext {
                    block_height,
                    block_timestamp: u64::from(block_time),
                    tx,
                    tx_id,
                    rune_id: Default::default(),
                    transaction: transaction.clone(),
                    mint2s: None,
                    mint3s: Option::from(mint3s.clone()),
                    burn2s: None,
                    burn3s: None,
                    address: Self::extract_mint3_address(mint3, transaction),
                };

                self.rsm_interpreter.execute(format!("{}:{}", block_height, tx), StateTransitionName::from_u32(mint3.state_transition_function.unwrap()).unwrap(), &operate_context);
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
}

pub struct RSMInterpreter {
    jump_table: JumpTable,
    contracts: HashMap<String, Box<dyn Contract>>,
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
                let _ = (operation.method)(contract.as_mut(), operate_context);
            }
            ContractExecResult::Success("RSM invoke contract successfully.".to_string())
        } else {
            ContractExecResult::UnknownException("Contract not found.".to_string())
        }
    }

    pub fn register_contract_app(&mut self, contract_app_id: String, contract_app: Box<dyn Contract>) {
        self.contracts.insert(contract_app_id, contract_app);
    }

    pub fn register_operation(&mut self, key: String, operation: Operation) {
        self.jump_table.register_operation(key, operation);
    }
}

pub struct JumpTable {
    pub operations: HashMap<String, Operation>,
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

    pub fn get_operation(&self, key: String) -> Option<&Operation> {
        self.operations.get(&key)
    }

    pub fn register_operation(&mut self, key: String, operation: Operation) {
        self.operations.insert(key, operation);
    }
}

pub struct Operation {
    pub function_name: String,
    pub method: fn(&mut dyn Contract, operate_context: &OperateContext) -> Result<(), ContractExecResult>,
}

pub enum StateTransitionName {
    Al,
    Rl,
    Sw,
}

impl StateTransitionName {
    fn from_u32(value: u32) -> Option<String> {
        match value {
            1 => Some(String::from("al")),
            2 => Some(String::from("rl")),
            3 => Some(String::from("sw")),
            _ => None,
        }
    }
}