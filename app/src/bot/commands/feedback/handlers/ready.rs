﻿use crate::bot::commands::feedback::helpers::create_and_pin_message;
use crate::prelude::*;
use serenity::{model::prelude::Ready, prelude::Context};

#[instrument(skip(ctx))]
pub async fn ready(ctx: &Context, _ready: &Ready) {
    debug!("ready");

    let mut settings = Settings::clone_state().await;
    let mut cmd_settings = &mut settings.commands.feedback;

    let channel = &cmd_settings.channel_id;
    let template = cmd_settings
        .template
        .replace(
            "{min_feature_up_votes}",
            &cmd_settings.min_feature_up_votes.to_string(),
        )
        .replace("{vote_up_emoji}", &cmd_settings.vote_up_emoji.to_string())
        .replace(
            "{vote_down_emoji}",
            &cmd_settings.vote_down_emoji.to_string(),
        );

    if let Some(message_id) = cmd_settings.template_message_id {
        let message = cmd_settings
            .channel_id
            .message(&ctx.http, message_id)
            .await
            .ok();

        if let Some(mut message) = message {
            info!("refreshing template message");

            message
                .edit(&ctx.http, |edit| edit.content(&template))
                .await
                .unwrap();
        } else {
            info!("template message was deleted, creating a new one");

            let new_message = create_and_pin_message(ctx, channel, &template).await;
            cmd_settings.template_message_id = Some(new_message.id);
            settings.save();
        }
    } else {
        info!("pinned message with report template not found, creating a new one");

        let new_message = create_and_pin_message(ctx, channel, &template).await;
        cmd_settings.template_message_id = Some(new_message.id);
        settings.save();
    }
}