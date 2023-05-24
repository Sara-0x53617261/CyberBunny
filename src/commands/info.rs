use crate::{Context, Error};
use crate::commands::statics;
use crate::tools::downloader::download;
use poise::serenity_prelude as prelude;
use serenity::model::channel::AttachmentType;
use poise::serenity_prelude::Timestamp;


/// Info parent command
#[poise::command(slash_command, subcommands("server", "user", "bot",))]
pub async fn info(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Gives some information about the server.
#[poise::command(slash_command)]
pub async fn server(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id.to_string();
    let owner = guild.owner_id.to_user(ctx).await.unwrap();

    let file_s = format!("{}.png", guild_id);
    let file_full = format!("tmp/{}.png", guild_id);
    let file = AttachmentType::from(file_full.as_str());

    let guild_icon_url = guild.icon_url().unwrap().replace(".webp", ".png");
    _ = download(guild_icon_url.as_str(), &file_s).await;

    ctx.send(|b| {
        b.embed(|e| {
            e.title(format!("Server info for {}", guild.name))
            .description(format!(
                "**Owner:** {}#{}\n**Users:** {}\nBoost level: {}\nThanks to: {} cool people",
                owner.name, owner.discriminator, guild.member_count, 
                guild.premium_tier.num(), guild.premium_subscription_count,
            ).as_str())
            .thumbnail(format!("attachment://{}.png", guild_id))
            .footer(|f| {
                f.text("CyberBunny - [Server Info]")
                .icon_url(statics::BOT_ICON)
            })
            .color(statics::EMBED_COLOR)
            .timestamp(Timestamp::now())
        });
        b.attachment(file)
    }).await?;

    Ok(())
}


/// Gives some information about the user.
#[poise::command(slash_command)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<prelude::User>,
) -> Result<(), Error> {
    let user = user.unwrap_or_else(|| ctx.author().clone());
    let user_id = user.id.to_string();

    let file_s = format!("{}.png", user_id);
    let file_full = format!("tmp/{}.png", user_id);
    let file = AttachmentType::from(file_full.as_str());

    let user_icon_url = user.avatar_url().unwrap().replace(".webp", ".png");
    _ = download(user_icon_url.as_str(), &file_s).await;

    ctx.send(|b| {
        b.embed(|e| {
            e.title(format!("info for {}", user.name))
            .description(format!(
                "**User ID:** {}\nUsername: {}\nDiscriminator: {}\nBot: {}\nAccount created on {}",
                user.id,
                user.name,
                user.discriminator,
                user.bot,
                user.created_at().to_rfc2822(),
            ).as_str())
            .thumbnail(format!("attachment://{}.png", user_id))
            .footer(|f| {
                f.text("CyberBunny - [User Info]")
                .icon_url(statics::BOT_ICON)
            })
            .color(statics::EMBED_COLOR)
            .timestamp(Timestamp::now())
        });
        b.attachment(file)
    }).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn bot(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("TODO: add bot info here").await?;
    Ok(())
}