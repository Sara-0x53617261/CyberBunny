// Environment variables
use dotenv;
use std::{env, collections::HashSet};

// Logger
use tracing::{info, debug, error, instrument};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

// Discord bot library
use poise::serenity_prelude as serenity;

// Local imports
mod commands;
mod tools;

pub struct Data{
    // steam_api_token: String,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    // dotenv
    dotenv::dotenv().ok();

    // get env variables
    let filter = EnvFilter::from_env("CYBERBUN_FILTER");
    let token = env::var("CYBERBUN_TOKEN").expect("TOKEN NOT FOUND");

    // Logger setup
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
    info!("Logger initialized with filter: {:?}", env::var("CYBERBUN_FILTER").unwrap());


    // Bot setup
    let mut owner_list = HashSet::new();
    owner_list.insert(serenity::UserId(190539099831140353));

    let options = poise::FrameworkOptions {
        owners: owner_list,
        commands: vec![
            commands::admin::admin(),
            commands::info::info(),
            commands::report::report(),
            ],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
       .options(options)
       .token(token)
       .intents(serenity::GatewayIntents::non_privileged())
       .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_activity(serenity::Activity::watching(format!("{} server(s) - OwO", ctx.cache.guild_count()))).await;
                Ok(Data {
                    // steam_api_token: env::var("STEAM_API_TOKEN").expect("NO STEAM_API_TOKEN FOUND"),
                })
            })
        });
    
    info!("Bot initialized");
    framework.run().await.unwrap();
}