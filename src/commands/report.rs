use crate::{Context, Error, Data};
use poise::Modal;
use tracing::info;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[derive(Debug, Modal)]
#[name = "Bug Info"]
struct BugInfoModal {
    #[name = "Bug information"]
    #[placeholder = "[bug info here]"]
    #[paragraph]
    #[min_length = 5]
    #[max_length = 1000]
    bug_info: String,
}

/// report parent command
#[poise::command(slash_command, subcommands("bug",))]
pub async fn report(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn bug(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let usern = format!("{}#{}", ctx.author().name, ctx.author().discriminator);
    let uuid = ctx.id();

    let report =  BugInfoModal::execute(ctx).await?;
    info!("Got bug report from {}\n{:?}", usern, report);

    // Todo: Find better way to save bug reports

    if report.is_some() {
            ctx.send(|msg| {
                msg.content("Thank you, your bug report was received")
                .ephemeral(true)
            }).await?;

            save_bug_report(report.unwrap(), usern, uuid).await?;
    } else {
            ctx.send(|msg| {
                msg.content("Looks like something went wrong, please let me know directly")
                .ephemeral(true)
            }).await?;
    }

    Ok(())
}

async fn save_bug_report(report: BugInfoModal, usern: String, uuid: u64) -> Result<(), Error> {
    let mut file = File::create(format!("bug_reports/{uuid}.txt")).await?;

    file.write_all(usern.as_bytes()).await?;
    file.write_all("\n".as_bytes()).await?;
    file.write_all(report.bug_info.as_bytes()).await?;
    
    Ok(())
}