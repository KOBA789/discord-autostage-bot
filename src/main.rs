use std::env;

use serenity::model::gateway::Ready;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
use serenity::{async_trait, json::JsonMap};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn voice_state_update(&self, ctx: Context, new: VoiceState) {
        let Some(guild_id) = new.guild_id.as_ref() else {
            return;
        };
        let Some(channel_id) = new.channel_id.as_ref() else {
            return;
        };

        // https://discord.com/developers/docs/resources/stage-instance#definitions
        let is_requesting_to_speak = new.request_to_speak_timestamp.is_some() && new.suppress;
        if is_requesting_to_speak {
            println!("approve: {}", new.user_id);

            // https://discord.com/developers/docs/resources/guild#modify-user-voice-state
            let map = JsonMap::from_iter([
                ("channel_id".into(), (*channel_id.as_u64()).into()),
                ("suppress".into(), false.into()),
            ]);
            ctx.http
                .edit_voice_state(*guild_id.as_u64(), *new.user_id.as_u64(), &map)
                .await
                .unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILDS;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
