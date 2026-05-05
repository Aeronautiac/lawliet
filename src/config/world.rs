use indexmap::IndexMap;

use crate::chargepool::PoolSpecifier;

// define base world states
#[derive(Hash, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum WorldChargePoolName {
    Prosecution,
}

pub struct WorldConfig {
    pub charge_pools: IndexMap<WorldChargePoolName, PoolSpecifier>,
}

impl WorldConfig {
    pub fn new() -> Self {
        let mut pools = IndexMap::new();

        pools.insert(
            WorldChargePoolName::Prosecution,
            PoolSpecifier {
                charges: 2,
                reset_time: 1,
            },
        );

        WorldConfig {
            charge_pools: pools,
        }
    }
}
