mod input_token;

use poise::{Framework, FrameworkOptions};
use serenity::all::ClientBuilder;
use serenity::prelude::*;

type Context<'a> = poise::Context<'a, (), anyhow::Error>;

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = input_token::input_or_get_token()
        .await
        .expect("Failed get Token");
    let intents = GatewayIntents::all();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                println!("Bot ready!");
                Ok(())
            })
        })
        .build();

    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();
}
