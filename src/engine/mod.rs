use crate::action::{ActionActor, ActionRequest, ActionResult, Command, action_dispatch};
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

    pub fn schedule(&mut self, request: ActionRequest) {
        let job = Job {
            seq_num: self.job_seq_num,
            request,
        };
        self.jobs.push(job);
        self.job_seq_num += 1;
    }

    // execute an action and the chain that follows
    // any sub-actions returned by top-level actions are assumed to be possible to execute by the system.
    // the chain will stop on failure, so ensure that validation is done on the top level to prevent
    // half-states.
    pub fn execute_chain(&mut self, action: ActionRequest) -> ActionResult {
        let timestamp = action.timestamp;
        let mut top_response = action_dispatch(self, action)?;
        for next_action in &mut top_response.next_actions {
            let mut bottom_response = self.execute_chain(ActionRequest {
                actor: ActionActor::System,
                timestamp,
                payload: next_action.clone(),
            })?;
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
        let mut earlier_actions: Vec<ActionRequest> = vec![];

        // first execute pending jobs
        while !self.jobs.is_empty() {
            if self.jobs.peek().unwrap().request.timestamp > action.timestamp {
                break;
            }
            let pending_action = self.jobs.pop().unwrap().request;
            earlier_actions.push(pending_action);
        }

        // ignore the errors of scheduled jobs. only append their commands if successful.
        // later a situation might arise where errors themselves give commands to the front-end (for
        // example, an execution failing because the person died), but for now ignore that and focus on main game functionality
        for act in earlier_actions {
            if let Ok(mut job_response) = self.execute_chain(act) {
                commands.append(&mut job_response.commands);
            }
        }

        let mut main_response = self.execute_chain(action)?;
        commands.append(&mut main_response.commands);
        main_response.commands = commands;

        Ok(main_response)
    }
}
