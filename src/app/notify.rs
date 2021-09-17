use std::convert::TryInto;

use serenity::{model::id::ChannelId, Client};
use toml::Value;

/// Will send a message to the discord channel
pub fn notify(ty: String, cfg: &Value) {
    let token: &str = cfg["discord_token"].as_str().unwrap_or("");

    if token.is_empty() {
        return;
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let mut channels: Vec<u64> = Vec::new();
            for z in cfg["channel_ids"].as_array().unwrap() {
                channels.push(z.as_integer().unwrap().try_into().unwrap());
            }

            let client = Client::builder(&token).await.expect("Err creating client");

            for i in channels {
                let channel = ChannelId(i);

                let _ = channel
                    .say(
                        client.cache_and_http.http.clone(),
                        ty.to_owned().to_string(),
                    )
                    .await;
            }
        });
}
