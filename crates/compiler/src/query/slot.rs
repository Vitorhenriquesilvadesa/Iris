use std::sync::{Condvar, Mutex};

use compiler_api::queries::{AnalysisResult, QueryResult};

use crate::query::state::QueryState;

#[derive(Debug)]
pub struct QuerySlot<T: Clone> {
    state: Mutex<QueryState<T>>,
    ready: Condvar,
}

impl<T: Clone> QuerySlot<T> {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(QueryState::InProgress),
            ready: Condvar::new(),
        }
    }

    pub fn wait(&self) -> QueryResult<T> {
        let mut state = self.state.lock().unwrap();

        loop {
            match &*state {
                QueryState::Completed(result) => {
                    return Ok(result.clone());
                }
                QueryState::Failed(error) => {
                    return Err(error.clone());
                }
                QueryState::InProgress => {
                    state = self.ready.wait(state).unwrap();
                }
            }
        }
    }

    pub fn complete(&self, result: AnalysisResult<T>) {
        let mut state = self.state.lock().unwrap();
        *state = QueryState::Completed(result);
        self.ready.notify_all();
    }

    pub fn fail(&self, error: String) {
        let mut state = self.state.lock().unwrap();
        *state = QueryState::Failed(error);
        self.ready.notify_all();
    }
}
