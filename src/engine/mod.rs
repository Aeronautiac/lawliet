use crate::Time;
use crate::action::{
    ActionContext, ActionError, ActionInterface, ActionRequest, ActionResponse, ActionResult,
};
use crate::common::SequenceNumber;
use crate::config::Config;
use crate::world::World;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
pub struct Job {
    pub id: SequenceNumber,
    pub request: ActionRequest,
    pub cancelled: RefCell<bool>,
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .request
            .timestamp
            .cmp(&self.request.timestamp)
            .then_with(|| other.id.cmp(&self.id))
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
    pub time: Time,
    pub jobs: BinaryHeap<Job>,
    next_job_id: SequenceNumber,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            world: World::new(),
            config: Config::new(),
            jobs: BinaryHeap::new(),
            time: 0,
            next_job_id: 0,
        }
    }

    pub fn schedule(&mut self, request: ActionRequest) {
        let job = Job {
            id: self.next_job_id,
            request,
            cancelled: RefCell::new(false),
        };
        self.jobs.push(job);
        self.next_job_id += 1;
    }

    pub fn is_future_timestamp(&self, timestamp: Time) -> bool {
        timestamp >= self.time
    }

    // attempt to execute an action atomically
    // first run a validation pass. this will propagate any sub action failures upwards without
    // modifying game state. after this, run the execution pass. this will crash on failure
    // (although this should never happen in practice due to the validation pass).
    // overflows are a non-issue. no action creates large enough of a chain to naturally overflow
    // the stack. if a stack overflow occurs it is due to an infinite recursion bug, and in this
    // case, a crash is necessary.
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
        if action.timestamp < self.time {
            return Err(ActionError::TimeAlreadyPassed);
        }

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

            let job = self.jobs.pop().unwrap();
            if *job.cancelled.borrow() {
                continue;
            }
            // ignore the errors of scheduled jobs.
            let _ = self.execute_atomic(&mut ctx, job.request);
        }

        // the command sequence matters because the frontend is also event based
        ctx.commands.reverse();

        let main_response = self.execute_atomic(&mut ctx, action)?;
        Ok((main_response, ctx))
    }

    // every update to any place in code after the engine is publicly usable requires the version number to be incremented by 1
    /// return the latest version of the engine
    pub fn version() -> u64 {
        0
    }
}
