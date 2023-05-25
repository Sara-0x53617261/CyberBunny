use crate::{Context, Error};
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
#[poise::command(slash_command, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Bot usage stats
#[poise::command(slash_command, owners_only)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    let pid: u32 = std::process::id();
    ctx.say(tools::process_info::get_process_info(pid).await?).await?;
    Ok(())
}