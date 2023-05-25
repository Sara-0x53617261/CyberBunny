use crate::{Context, Error};
use serenity::builder::{CreateInputText, CreateActionRow};

/// report parent command
#[poise::command(slash_command, subcommands("bug",))]
pub async fn report(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn bug(ctx: Context<'_>) -> Result<(), Error> {

    /*ctx.send(|e| {
        e.components(|c| {
            c.create_action_row(|r| {
                r.create_input_text(|i| {
                    i.custom_id("bug_description")
                    .label("Bug Description")
                    .required(true)
                    .style(poise::serenity_prelude::InputTextStyle::Paragraph)
                })
            })
        })
    }).await?;*/
    ctx.say("TODO").await?;


    Ok(())
}
