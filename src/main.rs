mod commands;

use commands::{fetch_data, now};
use deadpool_postgres::{Config, Pool};
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use std::sync::Arc;

struct Data {
    db_pool: Arc<Pool>,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Bot is starting...");

    let mut cfg = Config::new();
    cfg.host = Some(std::env::var("DB_HOST").expect("missing DB_HOST"));
    cfg.user = Some(std::env::var("DB_USERNAME").expect("missing DB_USERNAME"));
    cfg.password = Some(std::env::var("DB_PASSWORD").expect("missing DB_PASSWORD"));
    cfg.dbname = Some(std::env::var("DB_NAME").expect("missing DB_NAME"));
    let pool = cfg.create_pool(None, tokio_postgres::NoTls).unwrap();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let guild_id = std::env::var("GUILD_ID")
        .expect("missing GUILD_ID")
        .parse::<u64>()
        .expect("invalid GUILD_ID");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![now(), fetch_data()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            let guild_id = serenity::GuildId::new(guild_id);
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id)
                    .await?;
                Ok(Data {
                    db_pool: Arc::new(pool),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
