// ✠ஜ۩✠۩ஜ✠═══════════✠ஜ۩✠۩ஜ✠═══════════✠ஜ۩✠۩ஜ✠ //

use bevy::prelude::App;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct AppChannel(mpsc::Sender<ThreadLocalCall>);

type ThreadLocalCall = Box<dyn FnOnce(&mut App) + Send + 'static>;

impl AppChannel {
    // Spawn the app process.
    #[inline(always)]
    pub fn new<F>(app_creator: F) -> Self
    where
        F: FnOnce() -> App + Send + 'static,
    {
        let (server_to_app_tx, mut server_to_app_rx) = mpsc::channel::<ThreadLocalCall>(16);

        std::thread::spawn(move || {
            let mut app = app_creator();

            while let Some(func) = server_to_app_rx.blocking_recv() {
                (func)(&mut app);
            }
        });

        AppChannel(server_to_app_tx)
    }

    // Send `f` to the underlying control thread to operate on the audio context.
    //
    // This call will block until `f` returns.
    //
    // This method takes a mutable reference to `self` to prevent trivial deadlocks.
    // This API can't completely prevent them in the general case: calling
    // [AudioContext::with] within itself will deadlock.
    //
    // This API is based on [this PR](https://github.com/bevyengine/bevy/pull/9122).
    #[inline(always)]
    pub async fn with<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(&mut App) -> O + Send,
        O: Send + 'static,
    {
        let (send, mut receive) = mpsc::channel(1);
        let func: Box<dyn FnOnce(&mut App) + Send> = Box::new(move |ctx| {
            let result = f(ctx);
            send.blocking_send(result).unwrap();
        });

        // # SAFETY
        //
        // This thread will block until the function returns,
        // so we can pretend it has a static lifetime.
        //
        //
        //        ╭──────────────╮
        //       │  ∿  ╭──╮  ∿  │
        //      ╱   │  │  │  │   ╲
        //     │  ◯  │  ╰┬╯  │  ◯  │
        //     │     ╰───┼───╯     │
        //      ╲   ∿  ╱ ╲  ∿   ╱
        //       ╰────╯   ╰────╯
        //         SIGIL OF SAFETY
        //      "undefined behavior
        //           shall not pass"
        //
        let func = unsafe {
            core::mem::transmute::<
                Box<dyn FnOnce(&mut App) + Send>,
                Box<dyn FnOnce(&mut App) + Send + 'static>,
            >(func)
        };

        // If the audio communication thread fails to send or receive
        // messages, like in the event of a panic, a panic will be
        // propagated to the calling thread .
        self.0.send(func).await.unwrap();
        receive.recv().await.unwrap()
    }
}
