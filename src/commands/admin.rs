use crate::{Context, Error};
use crate::tools;
use crate::commands::statics;
use poise::serenity_prelude::Timestamp;

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
    let usage = tools::process_info::get_process_info(pid).await?;
    ctx.send(|msg| {
        msg.embed(|e| {
            e.title("Bot Memory usage")
            .description(
                format!("Peak: `{}` KB | `{:.2}` GB\nCurrent: `{}` KB | `{:.2}` GB", 
                usage.memory_peak_kb, usage.memory_peak_gb,
                usage.memory_kb, usage.memory_gb)
            ).thumbnail("attachment://bot.jpeg")
            .color(statics::EMBED_COLOR)
            .footer(|f| {
                f.text("Cyberbunny - [Memory usage]")
                .icon_url(statics::BOT_ICON)
            })
            .timestamp(Timestamp::now())
        })
    }).await?;
    // ctx.say(tools::process_info::get_process_info(pid).await?).await?;
    Ok(())
}