use crate::action::ActionRequest;
use crate::common::SequenceNumber;
use crate::config::Config;
use crate::world::World;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Job {
    pub seq_num: SequenceNumber,
    pub request: ActionRequest,
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .request
            .timestamp
            .cmp(&self.request.timestamp)
            .then_with(|| other.seq_num.cmp(&self.seq_num))
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Engine {
    pub world: World,
    pub config: Config,
    jobs: BinaryHeap<Job>,
    job_seq_num: SequenceNumber,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            world: World::new(),
            config: Config::new(),
            jobs: BinaryHeap::new(),
            job_seq_num: 0,
        }
    }

    // dispatch to an action handler
    // return result
    // action handlers encode result data into the engine structure similarly to the C errno
    // pattern
    pub fn execute(&mut self, action: ActionRequest) {}
}
