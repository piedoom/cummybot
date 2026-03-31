use std::str::FromStr;

use bevy::prelude::*;
use strum::EnumString;

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

    if let Ok(cum) = Cummand::from_str(msg) {
        use Cummand::*;
        match cum {
            Ping => {
                writer.write(message.say("pon'j freaking ponj"));
            }
            Penis => {
                writer.write(message.say("8=d"));
            }
            Cum => {
                writer.write(message.say("我爱中国！"));
            }
            _ => {}
        }
    } else {
        writer.write(message.say("cummand not supportead"));
    }
}

#[derive(EnumString, Message)]
#[strum(serialize_all = "snake_case")]
enum Cummand {
    Ping,
    Penis,
    Cum,
}
