// ✠ஜ۩✠۩ஜ✠═══════════✠ஜ۩✠۩ஜ✠═══════════✠ஜ۩✠۩ஜ✠ //

use bevy::prelude::*;
use dotenvy as dotenby;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use strum::EnumString;
use tokio::task::JoinHandle;

use crate::runtime::AsyncRuntime;

pub(crate) fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Startup, init_discord_client)
        .add_systems(Update, send_events)
        .add_message::<Cummand>();
}

const MAX_MESSAGE_COUNT: usize = 1024;

fn init_discord_client(mut commands: Commands, runtime: Res<AsyncRuntime>) {
    let (client, receiver) = runtime.block_on(async move {
        let token = dotenby::var("TOKEN").unwrap();

        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        let (sender, receiver) = crossbeam_channel::bounded(MAX_MESSAGE_COUNT);

        (
            Client::builder(&token, intents)
                .event_handler(Handler { queue: sender })
                .await
                .unwrap(),
            receiver,
        )
    });

    let handle = runtime.spawn(run_client(client));

    commands.insert_resource(DiscordClient { handle });
    commands.insert_resource(Cummands(receiver));
}

fn send_events(mut reader: ResMut<Cummands>, mut writer: MessageWriter<Cummand>) {
    for cummand in reader.0.try_recv() {
        writer.write(cummand);
    }
}

async fn run_client(mut client: serenity::prelude::Client) {
    if let Err(im_bout_takum) = client.start().await {
        panic!("came: {im_bout_takum}");
    }
}

#[derive(Resource)]
pub(crate) struct Cummands(crossbeam_channel::Receiver<Cummand>);

#[derive(Resource)]
struct DiscordClient {
    #[expect(unused)]
    handle: JoinHandle<()>,
}

struct Handler {
    queue: crossbeam_channel::Sender<Cummand>,
}

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

#[derive(EnumString, Message)]
#[strum(serialize_all = "snake_case")]
enum Cummand {
    Ping,
}

fn baby_shoes() {
    println!("for sale")
}
