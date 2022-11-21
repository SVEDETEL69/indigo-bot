﻿use crate::prelude::*;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use super::constants::{BUG_OPTION_NAME, FEATURE_OPTION_NAME};
use super::{handle_bug_report, handle_feature_report};

#[instrument(skip(ctx))]
pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    info!("feedback runned");
    let report_type = cmd.data.options.first().unwrap();

    debug!("{:#?}", report_type);

    match report_type.name.as_str() {
        BUG_OPTION_NAME => handle_bug_report(ctx, cmd).await,
        FEATURE_OPTION_NAME => handle_feature_report(ctx, cmd).await,
        _ => {
            error!("invalid option {}", report_type.name);
        }
    }
}
