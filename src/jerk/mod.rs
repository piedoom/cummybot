use bevy::prelude::*;

use crate::discord::corvy::{DiscordMessage, Say};

pub fn plugin(app: &mut App) {
    app.add_observer(ping_pong);
}

fn ping_pong(message: On<DiscordMessage>, mut writer: MessageWriter<Say>) {
    if message.message.content == "ping" {
        writer.write(message.say("pon'j freaking ponj"));
    }
}
