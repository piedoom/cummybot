use bevy::prelude::*;
mod discord;
mod runtime;

use dotenvy as dotenby;

fn main() {
    dotenby::dotenv().unwrap();
    println!("Hello, doomy!");
    println!("hai guyssssss~");
    println!("hewwo UwU");

    let mut app = App::new();

    app.add_plugins(MinimalPlugins);

    app.add_plugins((runtime::plugin, discord::plugin));

    app.run();
}
