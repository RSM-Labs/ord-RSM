use bitcoin::{Address, Transaction, Txid};
use chrono::{DateTime, Utc};
use ordinals::burn::{Burn2, Burn3};
use ordinals::mint::{Mint2, Mint3};
use ordinals::RuneId;

pub struct BlockContext {
    pub block_height: u32,
    pub block_hash: u32,
    pub block_time: DateTime<Utc>,
}

impl Default for BlockContext {
    fn default() -> Self {
        BlockContext {
            block_height: 0,
            block_hash: 0,
            block_time: Default::default(),
        }
    }
}

pub struct TxContext {
    pub tx_hash: u32,
    pub tx_index: u32,
    pub tx_hex: u32,
}

impl Default for TxContext {
    fn default() -> Self {
        TxContext {
            tx_hash:0,
            tx_index: 0,
            tx_hex: 0,
        }
    }
}

pub struct OperateContext {
    pub block_height: u64,
    pub block_timestamp: u64,
    pub tx: u32,
    pub tx_id: Txid,
    pub rune_id: RuneId,
    pub transaction: Transaction,
    pub mint2s: Option<Vec<Mint2>>,
    pub mint3s: Option<Vec<Mint3>>,
    pub burn2s: Option<Burn2>,
    pub burn3s: Option<Burn3>,
    pub address: Option<Address>,
}