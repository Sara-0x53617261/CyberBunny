use std::collections::{HashMap, BTreeMap};
use std::vec;

use crate::{Context, Error};
use crate::commands::statics;
use crate::tools::downloader::download;
use serenity::prelude::Mentionable;
use tracing::error;
use poise::serenity_prelude as prelude;
use serenity::futures::StreamExt;
use serenity::model::channel::AttachmentType;
use poise::serenity_prelude::Timestamp;
use chrono::{NaiveDateTime, Utc};

use itertools::Itertools;


/// Info parent command
#[poise::command(slash_command, subcommands("server", "user", "bot", "oldest"))]
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

    let created_on = time_format(guild.id.created_at().unix_timestamp());
    let created_since = time_since(guild.id.created_at().unix_timestamp());

    ctx.send(|b| {
        b.embed(|e| {
            e.title(format!("Server info for {}", guild.name))
            .description(format!(
                "**Owner:** {}#{}\n**Users:** {}\nBoost level: {}\nThanks to: {} cool people\nCreated on: \n`{}` | `{}` days ago",
                owner.name, owner.discriminator, guild.member_count, 
                guild.premium_tier.num(), guild.premium_subscription_count,
                created_on, created_since,
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
    let file_full = format!("tmp/{}", &file_s);
    let file = AttachmentType::from(file_full.as_str());

    let user_icon_url = user.avatar_url().unwrap().replace(".webp", ".png");
    _ = download(user_icon_url.as_str(), &file_s).await;

    let created_on = time_format(user.created_at().unix_timestamp());
    let created_since = time_since(user.created_at().unix_timestamp());

    let member = ctx.guild().unwrap().member(&ctx, user.id).await?;
    let time = member.joined_at.unwrap().unix_timestamp();
    let joined_on = time_format(time);
    let joined_since = time_since(time);

    ctx.send(|b| {
        b.embed(|e| {
            e.title(format!("Info for {}", user.name))
            .description(format!(
                "User ID: `{}`\nUsername: {}\nDiscriminator: {}\nBot: {}\nAccount created on;\n `{}` | `{}` days ago\nJoined on;\n `{}` | `{}` days ago\n",
                user.id,
                user.name,
                user.discriminator,
                if user.bot {"Yes"} else {"No"},
                created_on, created_since,
                joined_on, joined_since,
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

/// Gives some information about the bot.
#[poise::command(slash_command)]
pub async fn bot(ctx: Context<'_>) -> Result<(), Error> {
    let file = AttachmentType::from("bot.jpeg");

    ctx.send(|b| {
        b.embed(|e| {
            e.title("CyberBunny Info")
            .description("Hello, I am CyberBunny!
                I am a bot made in Rust as a practice project by Sara
                You can look at my source code on github! - https://github.com/Sara-0x53617261/CyberBunny
                I am build on the poise and serenity library.")
            .thumbnail("attachment://bot.jpeg")
            .footer(|f| {
                f.text("CyberBunny - [Bot Info]")
                .icon_url(statics::BOT_ICON)
            })
            .color(statics::EMBED_COLOR)
            .timestamp(Timestamp::now())
        });
        b.attachment(file)
    }).await?;
    Ok(())
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct OldUsers {
    created_at: i64,
    user: prelude::User,
}

// The User object does not support implementing Ord and PartialOrd
// because.. reasons ig?
// So i had to implement it manually.
impl Ord for OldUsers {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.created_at.cmp(&other.created_at)
    }
}
impl PartialOrd for OldUsers {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.created_at.cmp(&other.created_at))
    }
}

/// Lists the oldest x amount of users.
#[poise::command(slash_command)]
pub async fn oldest(
    ctx: Context<'_>,
    #[description = "List the top ... amount of users"]
    #[min = 1] users: Option<usize>,
) -> Result<(), Error> {
    let top_count = users.unwrap_or(10);
    let mut members = ctx.guild_id().unwrap().members_iter(&ctx).boxed();

    let mut top: Vec<OldUsers> = vec![];
    let mut ind: usize;

    while let Some(member) = members.next().await {

        match member {
            Ok(member) => {
                if top.len() < top_count {
                    // Populate the top list
                    top.push(OldUsers { 
                        created_at: member.user.created_at().unix_timestamp(), 
                        user: member.user 
                    });
                } else {
                    // find index of youngest user in list; 
                    // if current user is older than youngest user in list
                    // pop youngest user; push current user to list
                    ind = top.iter().position_min().unwrap();

                    if member.user.created_at().unix_timestamp() < top[ind].created_at {
                        top.push(OldUsers { 
                            created_at: member.user.created_at().unix_timestamp(), 
                            user: member.user 
                        });
                        top.remove(ind);
                    }
                }
            },
            Err(err) => { error!("Error getting oldest users for a guild\n{:?}", err); }
        }
    }

    let mut userlist: String = String::new();

    for val in top.iter() {
        userlist.push_str(format!(
        "**User:** {}\nCreated on: `{}`\n\n",
        val.user.mention(), time_format(val.created_at)).as_str());
    }
    ctx.send(|b| {
        b.embed(|e| {
            e.title("Oldest users")
            .description(userlist)
            .footer(|f| {
                f.text("CyberBunny - [Oldest users]")
                .icon_url(statics::BOT_ICON)
            })
            .color(statics::EMBED_COLOR)
            .timestamp(Timestamp::now())
        })
    }).await?;

    Ok(())
}



// TODO: Move these 2 to a util/tools module
fn time_format(unix_time: i64) -> String {
    let time = NaiveDateTime::from_timestamp_opt(unix_time, 0).unwrap();
    format!("{}", time.format("%d-%m-%Y %H:%M:%S"))
}

fn time_since(unix_time: i64) -> String {
    let time = NaiveDateTime::from_timestamp_opt(unix_time, 0).unwrap();
    format!("{}", Utc::now().naive_utc().signed_duration_since(time).num_days() )
}