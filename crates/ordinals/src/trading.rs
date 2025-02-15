use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
pub struct Trading {
    pub black_hole_percentage: Option<u32>,
    pub tax_percentage: Option<u32>,
    pub lp_fee_percentage: Option<u32>,
    pub service_fee_percentage: Option<u32>,
}

impl Trading {
    pub fn new() -> Self {
        Trading {
            black_hole_percentage: Default::default(),
            tax_percentage: Default::default(),
            lp_fee_percentage: Default::default(),
            service_fee_percentage: Default::default(),
        }
    }
}