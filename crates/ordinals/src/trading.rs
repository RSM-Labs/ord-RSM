use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
pub struct Trading {
    pub lp_fee_percentage: Option<u32>,
    pub service_fee_percentage: Option<u32>,
}

impl Trading {
    pub fn new() -> Self {
        Trading {
            lp_fee_percentage: Default::default(),
            service_fee_percentage: Default::default(),
        }
    }
}