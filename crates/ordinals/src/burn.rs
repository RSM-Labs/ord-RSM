use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::{Edict, RuneId};
use crate::ext::Ext;
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Burn2 {
    pub to: RuneId,
    pub state_transition_function: u32,
    pub edicts: Vec<Edict>,
    pub proof: Option<Proof>,
    pub ext: Option<Ext>
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Burn3 {
    pub to: RuneId,
    pub state_transition_function: u32,
    pub edicts: Vec<Edict>,
    pub proof: Option<Proof>,
    pub ext: Option<Ext>
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Proof {
    pub block: u64,
    pub tx: u32,
    pub output: u32,
    pub amount: u32,
}

impl std::fmt::Debug for Proof {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Proof {{ block: {}, tx: {}, output: {}, amount: {} }}", self.block, self.tx, self.output, self.amount)
    }
}

impl PartialEq for Proof {
    fn eq(&self, other: &Self) -> bool {
        self.block == other.block
            && self.tx == other.tx
            && self.output == other.output
            && self.amount == other.amount
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Proof {}