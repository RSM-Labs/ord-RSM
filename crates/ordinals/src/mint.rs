use serde::{Deserialize, Serialize};
use crate::burn::Proof;
use crate::Edict;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Mint2 {
    pub state_transition_function: Option<u32>,
    pub edicts: Option<Vec<Edict>>,
    pub proof: Option<Proof>,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Mint3 {
    pub state_transition_function: Option<u32>,
    pub edicts: Option<Vec<Edict>>,
    pub proof: Option<Proof>,
}