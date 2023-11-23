use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::prelude::ChannelId;

use crate::ParsedConfig;

use super::formatters::log_embed_formatter::log_embed_formatter;

pub async fn channel_msg_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }
    let config_hashmap = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<ParsedConfig>()
            .expect("Expected Parsed Config In TypeMap.")
            .clone()
    };

    let log_channel_id: ChannelId;

    // Lock Block
    {
        log_channel_id = ChannelId(config_hashmap.lock().await.log_channel_id);
    }

    let embed: CreateEmbed = log_embed_formatter(ctx, msg).await;

    let _send_message = log_channel_id
        .send_message(&ctx.http, |message| message.set_embed(embed))
        .await;
}