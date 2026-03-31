use bevy::prelude::*;
use dotenvy as dotenby;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use tokio::task::JoinHandle;

use crate::runtime::AsyncRuntime;

pub(super) fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Startup, init_discord_client);
}

fn init_discord_client(mut commands: Commands, runtime: Res<AsyncRuntime>) {
    let client = runtime.block_on(async {
        let token = dotenby::var("TOKEN").unwrap();

        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        Client::builder(&token, intents)
            .event_handler(Handler)
            .await
            .unwrap()
    });

    let handle = runtime.spawn(run_client(client));

    commands.insert_resource(DiscordClient { handle })
}

async fn run_client(mut client: serenity::prelude::Client) {
    if let Err(im_bout_takum) = client.start().await {
        panic!("came: {im_bout_takum}");
    }
}

#[derive(Resource)]
pub struct DiscordClient {
    #[expect(unused)]
    handle: JoinHandle<()>,
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event. This is called whenever a new message
    // is received.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events
    // can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an authentication error,
            // or lack of permissions to post in the channel, so log to stdout
            // when some error happens, with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard
    // is booted, and a READY payload is sent by Discord. This payload contains
    // data like the current user's guild Ids, current user data, private
    // channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        println!("my ass is ready");
    }
}
