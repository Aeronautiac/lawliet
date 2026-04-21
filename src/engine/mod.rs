use crate::action::command::Command;
use crate::action::{ActionActor, ActionInterface, ActionRequest, ActionResult, command};
use crate::common::SequenceNumber;
use crate::config::Config;
use crate::world::World;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Job {
    pub seq_num: SequenceNumber,
    pub request: ActionRequest,
    pub cancelled: bool, // instead of a cancelled flag and weird logic to set it while it is in the
                         // heap, there should instead be a set of "waiting jobs" based on sequence number. to cancel a
                         // job, just remove it from the set. the engine will check if its in the set before executing.
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

    fn schedule(&mut self, request: ActionRequest) {
        let job = Job {
            seq_num: self.job_seq_num,
            request,
            cancelled: false,
        };
        self.jobs.push(job);
        self.job_seq_num += 1;
    }

    // execute an action and the chain that follows
    // any sub-actions returned by top-level actions are assumed to be possible to execute by the system.
    // the chain will crash on failure if the action is not top-level. this is to prevent invalid
    // states which can potentially ruin the game.
    // overflows are a non-issue. no action creates large enough of a chain to overflow the stack.
    fn execute_chain(&mut self, action: ActionRequest) -> ActionResult {
        let timestamp = action.timestamp;
        action.payload.validate(self, &action.actor)?;
        let mut top_response = action.payload.execute(self, &action.actor);
        for next_action in &mut top_response.next_actions {
            let mut bottom_response = self
                .execute_chain(ActionRequest {
                    actor: ActionActor::System,
                    timestamp,
                    payload: next_action.clone(),
                })
                .unwrap(); // crash on sub-action failure
            top_response.commands.append(&mut bottom_response.commands);
        }
        Ok(top_response)
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
            if job.cancelled {
                self.jobs.pop();
                continue;
            }
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
