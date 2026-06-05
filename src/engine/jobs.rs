use std::{cmp::Ordering, collections::BinaryHeap};

use indexmap::{IndexSet, indexset};

use crate::{action::ActionRequest, common::JobID};

#[derive(PartialEq, Eq, Debug)]
pub struct Job {
    pub id: JobID,
    pub request: ActionRequest,
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

#[derive(Debug)]
pub struct Jobs {
    jobs: BinaryHeap<Job>,
    cancelled: IndexSet<JobID>,
    next_job_id: JobID,
}

impl Jobs {
    pub fn new() -> Self {
        Jobs {
            jobs: BinaryHeap::new(),
            cancelled: indexset! {},
            next_job_id: 0,
        }
    }

    pub fn cancel_all_cond<F>(&mut self, cond: F, mutate: bool) -> usize
    where
        F: Fn(&Job) -> bool,
    {
        let mut c = 0;
        for job in self.jobs.iter() {
            if !self.cancelled.contains(&job.id) && cond(job) {
                if mutate {
                    self.cancelled.insert(job.id);
                }
                c += 1;
            }
        }
        c
    }

    pub fn cancel_id(&mut self, id: JobID) -> bool {
        self.cancelled.insert(id)
    }

    fn pop_cancelled(&mut self) {
        while !self.jobs.is_empty() {
            let curr_job = self.jobs.peek().unwrap();
            let id = &curr_job.id;
            if self.cancelled.contains(id) {
                self.cancelled.swap_remove(id);
                self.pop();
            } else {
                break;
            }
        }
    }

    pub fn peek(&self) -> Option<&Job> {
        self.jobs.peek()
    }

    pub fn push(&mut self, request: ActionRequest) -> JobID {
        let id = self.next_job_id;
        self.jobs.push(Job { id, request });
        self.next_job_id += 1;
        id
    }

    pub fn pop(&mut self) -> Option<Job> {
        self.pop_cancelled();
        let result = self.jobs.pop();
        self.pop_cancelled();
        result
    }

    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty() || self.cancelled.len() == self.jobs.len()
    }
}
