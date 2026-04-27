use crate::Timestamp;
use crate::action::{ActionActor, ActionRequest, ActionResult};
use crate::command::Command;
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

    fn handle_chain(&mut self, action: &mut ActionRequest, mutate: bool) -> ActionResult {
        let timestamp = action.timestamp;
        self.time = timestamp;
        let mut top_response = if mutate {
            action.payload.execute(self, &action.actor, 0)
        } else {
            action.payload.dry_run(self, &action.actor, 0)
        }?;
        for next_action in &mut top_response.next_actions {
            let mut bottom_response = self.handle_chain(
                &mut ActionRequest {
                    actor: ActionActor::System,
                    timestamp,
                    payload: next_action.clone(),
                },
                mutate,
            )?;
            top_response.commands.append(&mut bottom_response.commands);
        }
        Ok(top_response)
    }

    // attempt to execute an action and the chain that follows.
    // first run a validation pass. this will propagate any sub action failures upwards without
    // modifying game state. after this, run the execution pass. this will crash on failure
    // (although this should never happen in practice due to the validation pass).
    // overflows are a non-issue. no action creates large enough of a chain to overflow the stack.
    fn execute_chain(&mut self, mut action: ActionRequest) -> ActionResult {
        self.handle_chain(&mut action, false)?;
        let result = self.handle_chain(&mut action, true);
        result
            .as_ref()
            .expect("Validate and execute pass desync detected.");
        result
    }

    // store a command buffer
    // check action timestamp and execute any pending jobs that happen before/at the timestamp
    // execute the requested action
    // recursively execute sub-actions and append command buffers
    // return only top level result (with the combined command buffer)
    pub fn execute(&mut self, action: ActionRequest) -> ActionResult {
        let mut commands: Vec<Command> = vec![];

        // first execute pending jobs
        loop {
            if self.jobs.is_empty() {
                break;
            }
            let job = self.jobs.peek().unwrap();
            if job.request.timestamp > action.timestamp {
                break;
            }

            // ignore the errors of scheduled jobs. only append their commands if successful.
            let pending_action = self.jobs.pop().unwrap().request;
            if let Ok(mut job_response) = self.execute_chain(pending_action) {
                commands.append(&mut job_response.commands);
            }
        }

        let mut main_response = self.execute_chain(action)?;
        commands.append(&mut main_response.commands);
        main_response.commands = commands;

        Ok(main_response)
    }
}
