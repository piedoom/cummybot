// ✠ஜ۩✠۩ஜ✠═══════════✠ஜ۩✠۩ஜ✠═══════════✠ஜ۩✠۩ஜ✠ //

use bevy::prelude::*;
use tokio::{runtime::Runtime, task::JoinHandle};

#[derive(Resource)]
pub struct AsyncRuntime {
    rt: Runtime,
}

impl AsyncRuntime {
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.rt.spawn(future)
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        self.rt.block_on(future)
    }
}

pub(super) fn plugin(app: &mut App) {
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    let runtime = builder.enable_all().build().unwrap();

    let runtime = AsyncRuntime { rt: runtime };

    app.insert_resource(runtime);
}
