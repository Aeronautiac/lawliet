use crate::{
    ID,
    common::{ChargeCount, IterationCount, LinkWeight},
};
use std::cmp::max;

#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
pub enum PoolLinkType {
    Limit, // every linked ability loses charges with the amount depending on weight. if at least
    // one ability cannot afford it, then usage fails.
    Pool, // same subtraction policy, but only fails of none of the linked abilities can afford the
          // cost.
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct PoolSpecifier {
    pub charges: ChargeCount,
    pub reset_time: IterationCount,
}

#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct PoolLink {
    pub link_type: PoolLinkType,
    pub link_dest: ID,
    pub weight: LinkWeight,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct ChargePool {
    pub charges: ChargeCount,
    pub base_charges: ChargeCount,
    pub iterations_to_reset: IterationCount,
    pub base_reset_time: IterationCount,
    pub ref_count: u32,
}

impl ChargePool {
    pub fn new(base_charges: ChargeCount, base_reset_time: IterationCount) -> Self {
        ChargePool {
            charges: base_charges,
            base_charges,
            iterations_to_reset: 0,
            base_reset_time,
            ref_count: 0,
        }
    }

    pub fn can_use_charges(&self, charges: ChargeCount) -> bool {
        self.charges >= charges
    }

    pub fn use_charges(&mut self, charges: ChargeCount) {
        self.charges = max(self.charges - charges, 0);
        if self.iterations_to_reset > 0 {
            self.iterations_to_reset = self.base_reset_time;
        }
    }

    /// these are parameters because they may change throughout the game
    pub fn on_iteration(&mut self) {
        self.iterations_to_reset -= 1;
        if self.iterations_to_reset == 0 {
            self.charges = self.base_charges;
        }
    }

    pub fn on_link(&mut self) {
        self.ref_count += 1;
    }

    /// if the reference count hits zero, it returns true to signal that the pool should be
    /// destroyed (if applicable)
    pub fn on_unlink(&mut self) -> bool {
        self.ref_count -= 1;
        self.ref_count == 0
    }
}
