// ✠ஜ۩✠۩ஜ✠═══════════✠ஜ۩✠۩ஜ✠═══════════✠ஜ۩✠۩ஜ✠ //

// ~~PRESERVED FOR ARCHIEOLOGICAL PURPOSES~~ RESTORED
//
// ░░░░░░░
// ░░░█░░░
// ░▀▀█▀▀░
// ░░░█░░░
// ░░░█░░░
// ░░░█░░░
// ░░░▀░░░
//
// ░░░░░░░
// ░░░█░░░
// ░▀▀█▀▀░
// ░░░█░░░
// ░░░█░░░
// ░░░█░░░
// ░░░▀░░░
//
// ░░░░░░░
// ░░░█░░░
// ░▀▀█▀▀░
// ░░░█░░░
// ░░░█░░░
// ░░░█░░░
// ░░░▀░░░

use std::sync::Arc;

use bevy::prelude::*;
use tokio::{runtime::Runtime, task::JoinHandle};

#[derive(Resource)]
pub(crate) struct AsyncRuntime {
    runtime: Arc<Runtime>,
}

// DOOMY'S IMPL BLOCK
impl AsyncRuntime {
    pub(crate) fn new(runtime: Arc<Runtime>) -> Self {
        Self { runtime }
    }
}

// NOT DOOMY'S (CORVY'S( IMPL BLOCK
impl AsyncRuntime {
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        self.runtime.block_on(future)
    }
}
