use bevy::prelude::*;

use crate::discord::corvy::{DiscordMessage, Say};

const TRIGGER_CHARACTER: char = '!';

pub(crate) fn plugin(app: &mut App) {
    app.add_observer(ping_pong);
}

fn ping_pong(message: On<DiscordMessage>, mut writer: MessageWriter<Say>) {
    println!(
        "I'm freaking cummmmmmmmming blood: {}",
        message.message.content
    );

    let Some(msg) = message.message.content.strip_prefix(TRIGGER_CHARACTER) else {
        return;
    };
    match msg {
        "ping" => {
            writer.write(message.say("pon'j freaking ponj"));
        }
        _ => {}
    }
}
