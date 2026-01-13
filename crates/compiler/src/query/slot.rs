use std::sync::{Arc, Condvar, Mutex};

use common::diagnostic::Diagnostics;

use crate::query::state::QueryState;

#[derive(Debug)]
pub struct QuerySlot<T> {
    state: Mutex<QueryState<T>>,
    ready: Condvar,
}

impl<T> QuerySlot<T> {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(QueryState::InProgress),
            ready: Condvar::new(),
        }
    }

    pub fn wait(&self) -> Result<Arc<T>, Arc<Diagnostics>> {
        let mut state = self.state.lock().unwrap();

        loop {
            match &*state {
                QueryState::Completed(value) => {
                    return Ok(value.clone());
                }
                QueryState::Failed(diags) => {
                    return Err(diags.clone());
                }
                QueryState::InProgress => {
                    state = self.ready.wait(state).unwrap();
                }
            }
        }
    }

    pub fn complete(&self, value: Arc<T>) {
        let mut state = self.state.lock().unwrap();
        *state = QueryState::Completed(value);
        self.ready.notify_all();
    }

    pub fn fail(&self, diagnostics: Arc<Diagnostics>) {
        let mut state = self.state.lock().unwrap();
        *state = QueryState::Failed(diagnostics);
        self.ready.notify_all();
    }
}
