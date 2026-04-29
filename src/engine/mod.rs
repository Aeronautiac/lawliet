use crate::Timestamp;
use crate::action::{
    ActionContext, ActionError, ActionInterface, ActionRequest, ActionResponse, ActionResult,
};
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
    pub time: Timestamp,
    jobs: BinaryHeap<Job>,
    job_seq_num: SequenceNumber,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            world: World::new(),
            config: Config::new(),
            jobs: BinaryHeap::new(),
            time: 0,
            job_seq_num: 0,
        }
    }

    pub fn schedule(&mut self, request: ActionRequest) {
        let job = Job {
            seq_num: self.job_seq_num,
            request,
        };
        self.jobs.push(job);
        self.job_seq_num += 1;
    }

    pub fn is_future_timestamp(&self, timestamp: Timestamp) -> bool {
        timestamp >= self.time
    }

    // attempt to execute an action atomically
    // first run a validation pass. this will propagate any sub action failures upwards without
    // modifying game state. after this, run the execution pass. this will crash on failure
    // (although this should never happen in practice due to the validation pass).
    // overflows are a non-issue. no action creates large enough of a chain to naturally overflow
    // the stack. if a stack overflow occurs it is due to an infinite recursion bug, and in this
    // case, a crash is necessary regardless.
    fn execute_atomic(
        &mut self,
        ctx: &mut ActionContext,
        mut action: ActionRequest,
    ) -> ActionResult {
        let old_time = self.time;
        self.time = action.timestamp;
        let dry_result = action.payload.handle(self, ctx, &action.actor, 0, false);
        if dry_result.is_err() {
            self.time = old_time;
            return dry_result;
        }
        self.time = action.timestamp;
        let result = action.payload.handle(self, ctx, &action.actor, 0, true);
        result
            .as_ref()
            .expect("Validate and execute pass desync detected.");
        result
    }

    // store a command buffer
    // check action timestamp and execute any pending jobs that happen before/at the timestamp
    // execute the requested action
    // recursively execute sub-actions
    // return only top level result (with the combined command buffer)
    pub fn execute(
        &mut self,
        action: ActionRequest,
    ) -> Result<(ActionResponse, ActionContext), ActionError> {
        let mut ctx = ActionContext { commands: vec![] };

        // first execute pending jobs
        loop {
            if self.jobs.is_empty() {
                break;
            }
            let job = self.jobs.peek().unwrap();
            if job.request.timestamp > action.timestamp {
                break;
            }

            // ignore the errors of scheduled jobs.
            let pending_action = self.jobs.pop().unwrap().request;
            let _ = self.execute_atomic(&mut ctx, pending_action);
        }

        // also need to return command buffer
        let main_response = self.execute_atomic(&mut ctx, action)?;
        Ok((main_response, ctx))
    }
}
