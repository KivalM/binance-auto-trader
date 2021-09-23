use serenity::{model::id::ChannelId, Client};

use crate::general::ApiInfo;

/// Will send a message to the discord channel
pub fn notify(ty: String, cfg: &ApiInfo) {
    let token: &str = &cfg.config.discord_token;

    if token.is_empty() {
        return;
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let mut channels: Vec<u64> = Vec::new();
            for z in &cfg.config.channel_ids {
                channels.push(*z)
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
