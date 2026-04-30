use std::collections::BTreeMap;

use crate::ID;

#[derive(Debug)]
pub enum NotebookError {
    NoOwner,
    OnCooldown,
    NotOwned,
}

#[derive(Debug, PartialEq)]
pub struct Notebook {
    pub volatile: bool, // if a notebook is volatile, it will be destroyed when the original owner's
    // role changes
    pub fake: bool, // if a notebook is fake, it cannot actually kill people
    pub original_owner: Option<ID>,
    pub owner: Option<ID>,    // the person this notebook currently belongs to
    pub borrowed: Option<ID>, // the person the notebook is being borrowed from (if any)
    pub iteration_successes: BTreeMap<ID, u8>, // success counts (correct names)
    pub iteration_failures: BTreeMap<ID, u8>, // failed counts (wrong names)
}

impl Notebook {
    pub fn new(fake: bool) -> Self {
        Notebook {
            fake,
            volatile: false,
            original_owner: None,
            owner: None,
            borrowed: None,
            iteration_successes: BTreeMap::new(),
            iteration_failures: BTreeMap::new(),
        }
    }

    pub fn set_original_owner(&mut self, id: ID, volatile: bool) {
        self.original_owner = Some(id);
        self.volatile = volatile;
    }

    pub fn set_true_owner(&mut self, id: ID) {
        self.borrowed = None;
        self.owner = Some(id);
    }

    pub fn can_lend(&self, id: ID) -> Result<(), NotebookError> {
        if self.owner != Some(id) {
            return Err(NotebookError::NotOwned);
        }
        if self.borrowed.is_some() {
            return Err(NotebookError::NotOwned);
        }
        Ok(())
    }

    pub fn lend(&mut self, id: ID) -> Result<(), NotebookError> {
        if let Some(owner) = self.owner {
            self.borrowed = Some(owner);
            self.owner = Some(id);
            Ok(())
        } else {
            Err(NotebookError::NoOwner)
        }
    }

    pub fn is_owner_borrowing(&self) -> bool {
        self.borrowed.is_some()
    }

    pub fn iteration_reset(&mut self) {
        self.iteration_successes = BTreeMap::new();
        self.iteration_failures = BTreeMap::new();
        if let Some(true_owner) = self.borrowed {
            self.set_true_owner(true_owner);
        }
    }

    pub fn on_write_success(&mut self, id: ID) {
        self.iteration_successes
            .entry(id)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn on_write_failure(&mut self, id: ID) {
        self.iteration_failures
            .entry(id)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn can_write(
        &self,
        id: ID,
        fail_limit: u8,
        success_limit: u8,
    ) -> Result<(), NotebookError> {
        if self.owner != Some(id) {
            return Err(NotebookError::NotOwned);
        }

        if self.iteration_failures.get(&id).copied().unwrap_or(0u8) >= fail_limit {
            return Err(NotebookError::OnCooldown);
        }
        if self.iteration_successes.get(&id).copied().unwrap_or(0u8) >= success_limit {
            return Err(NotebookError::OnCooldown);
        }

        Ok(())
    }
}
