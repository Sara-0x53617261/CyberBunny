use crate::{Context, Error};
use crate::commands::statics;
use poise::serenity_prelude as prelude;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};
use serenity::utils::Colour;
use tracing::debug;

/// Info parent command
#[poise::command(slash_command, subcommands("server", "user", "bot",))]
pub async fn info(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Gives some information about the server.
#[poise::command(slash_command)]
pub async fn server(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let owner = guild.owner_id.to_user(ctx).await.unwrap();

    let mut embed = CreateEmbed::default();

    let mut author = CreateEmbedAuthor::default();
    author.name(&guild.name);
    if let Some(url) = guild.icon_url() {
        debug!("Author icon url: {}", &url);
        author.icon_url(url);
    }
    embed.set_author(author);
    embed.color(Colour::new(statics::EMBED_COLOR));

    embed.description(format!(
            "**Server Owner:** {}\nUsers: {}\nBoost level: {}\nThanks to: {} cool people",
            owner.name, guild.member_count, 
            guild.premium_tier.num(), guild.premium_subscription_count,

    ));
    
    // Refuses to show in the embed; no clue why - workaround with the author section
    if let Some(url) = guild.icon_url() {
        debug!("Thumbnail url: {}", &url);
        embed.thumbnail(&url);
        embed.image(&url);
    }
    
    embed.set_footer(get_bot_footer().await);

    ctx.send(|b| {
        b.embed(|b| { *b = embed; b })
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
    let mut embed = CreateEmbed::default();
    // embed.title(format!("User Info for {}", user.name));
    
    let mut author = CreateEmbedAuthor::default();
    author.name(&user.name);
    author.icon_url(user.avatar_url().unwrap());
    embed.set_author(author);

    embed.description(format!(
        "**User ID:** {}\nUsername: {}\nDiscriminator: {}\nBot: {}\nAccount created on {}",
        user.id,
        user.name,
        user.discriminator,
        user.bot,
        user.created_at().to_rfc2822(),
    ));

    embed.set_footer(get_bot_footer().await);
    embed.color(Colour::new(statics::EMBED_COLOR));

    ctx.send(|b| {
        b.embed(|b| { *b = embed; b })
    }).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn bot(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("TODO: add bot info here").await?;
    Ok(())
}

async fn get_bot_footer() -> CreateEmbedFooter {
    let mut footer = CreateEmbedFooter::default();
    
    footer.icon_url(statics::BOT_ICON.to_string());
    footer.text("CyberBunny");

    footer
}