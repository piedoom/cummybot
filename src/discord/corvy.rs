// ✠ஜ۩✠۩ஜ✠═══════════✠ஜ۩✠۩ஜ✠═══════════✠ஜ۩✠۩ஜ✠ //

use bevy::prelude::*;
use dotenvy as dotenby;
use serenity::{
    async_trait,
    builder::CreateMessage,
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Last, drain_balls);
}

pub async fn run_app(channel: crate::app_channel::AppChannel) {
    let token = dotenby::var("TOKEN").unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    Client::builder(&token, intents)
        .event_handler(Handler { channel })
        .await
        .unwrap()
        .start()
        .await
        .unwrap();
}

struct Handler {
    channel: crate::app_channel::AppChannel,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        self.channel
            .clone()
            .with(|app| {
                let message = DiscordMessage {
                    message: msg,
                    context: ctx,
                };

                let world = app.world_mut();
                world.trigger(message.clone());
                world.write_message(message.clone());
                app.update();
            })
            .await
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is so close mpf,,,!", ready.user.name);
    }
}

#[derive(Event, Message, Clone)]
pub struct DiscordMessage {
    pub message: Message,
    pub context: Context,
}

#[derive(Message, Clone)]
pub struct Say {
    pub message: CreateMessage,
    pub context: Context,
    pub channel_id: ChannelId,
}

impl DiscordMessage {
    pub fn say(&self, message: impl Into<String>) -> Say {
        Say {
            message: CreateMessage::default().content(message),
            context: self.context.clone(),
            channel_id: self.message.channel_id,
        }
    }
}

/// Drains the ECS messages and writes them out in a task.
fn drain_balls(mut reader: MessageReader<Say>) {
    let messages: Vec<_> = reader.read().cloned().collect();

    tokio::spawn(async move {
        for message in messages {
            if let Err(e) = message
                .channel_id
                .send_message(&message.context.http, message.message)
                .await
            {
                tracing::error!("Failed to send message: {}", e);
            }
        }
    });
}
