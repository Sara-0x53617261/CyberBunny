use poise::serenity_prelude::UserId;

use crate::{Context, Error};
use crate::commands::statics;
use crate::tools;

/// Admin parent command
#[poise::command(
    slash_command,
    subcommands("register", "status"),
)]
pub async fn admin(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Register new commands
#[poise::command(slash_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    if is_owner(&ctx.author().id).await {
        poise::builtins::register_application_commands_buttons(ctx).await?;
    } else {
        ctx.say(format!("Sorry, only the owner can execute this command!")).await?;
    }
    Ok(())
}

/// Bot usage stats
#[poise::command(slash_command)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    if is_owner(&ctx.author().id).await {
        let pid: u32 = std::process::id();
        ctx.say(tools::process_info::get_process_info(pid).await?).await?;
    } else {
        ctx.say(format!("Sorry, only the owner can execute this command!")).await?;
    }
    Ok(())
}

// Is owner check
async fn is_owner(user_id: &UserId) -> bool {
    match user_id.as_u64().cmp(&statics::OWNER_ID) {
        std::cmp::Ordering::Equal => true,
        _ => false,
    }
}