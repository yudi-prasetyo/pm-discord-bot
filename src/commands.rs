use crate::{Context, Error};
use chrono::Utc;
use tokio_postgres::NoTls;

// use poise::serenity_prelude as serenity;

/// Displays the current date
///
/// # Examples
///
/// ```text
/// 2022-12-30
/// ```
#[poise::command(slash_command)]
pub async fn now(ctx: Context<'_>) -> Result<(), Error> {
    let now = Utc::now();
    let response = format!("Current date is {}", now);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn fetch_data(ctx: Context<'_>) -> Result<(), Error> {
    let pool = &ctx.data().db_pool;
    let client = pool.get().await?;
    println!("Fetch data...");
    let rows = client.query("SELECT description FROM tasks", &[]).await?;
    println!("Got {} rows", rows.len());

    let mut response = String::new();
    for row in rows {
        println!("Got row: {:?}", row);
        let value: String = row.get(0);
        response.push_str(&format!("{}\n", value));
    }

    if response.is_empty() {
        response.push_str("No data found.");
    }

    ctx.say(response).await?;
    Ok(())
}
