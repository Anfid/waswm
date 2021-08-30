use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;
use wasmer::WasmerEnv;

use crate::layout::*;

#[derive(Clone, WasmerEnv)]
pub struct State(Arc<Mutex<StateInner>>);

pub struct StateInner {
    pub focused: usize,
    pub layout: Box<dyn Layout>,
}

impl State {
    pub fn get(&self) -> MutexGuard<StateInner> {
        self.0.lock()
    }

    pub fn try_get(&self) -> Option<MutexGuard<StateInner>> {
        self.0.try_lock()
    }
}

impl Default for State {
    fn default() -> Self {
        let inner = Default::default();
        State(Arc::new(Mutex::new(inner)))
    }
}

impl Default for StateInner {
    fn default() -> Self {
        Self {
            focused: 0,
            layout: Box::new(Floating),
        }
    }
}
