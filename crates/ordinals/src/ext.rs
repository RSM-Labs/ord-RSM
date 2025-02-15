use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
pub struct Ext {
    pub amount0_min: Option<u32>,
    pub amount1_min: Option<u32>,
    pub amount_out_min: Option<u32>,
    pub deadline: Option<u32>,
}