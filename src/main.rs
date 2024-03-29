mod util {
    pub mod globals;
    pub mod utils;
}


mod commands {
    pub mod say;
    pub mod age;
    pub mod move_here;
    pub mod register;
    pub mod steal_profile_pic;
    pub mod ping;
    pub mod warn;pub mod self_update;
    pub mod shutdown;
    pub mod music;
}


use songbird::SerenityInit;
use poise::{serenity_prelude as serenity};
use dotenv::dotenv;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {

    dotenv().unwrap_or_else(|e| panic!("Failed to load .env file: {}", e));
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::register::register(),
                commands::age::age(), 
                commands::say::say(),
                commands::move_here::move_here(),
                commands::steal_profile_pic::steal_profile_pic(),
                commands::ping::ping(),
                commands::warn::warn(),
                commands::self_update::self_update(),
                commands::shutdown::shutdown(),
                commands::music::clear::clear(),
                commands::music::leave::leave(),
                commands::music::nowplaying::now_playing(),
                commands::music::pause::pause(),
                commands::music::play::play(),
                commands::music::queue::queue(),
                commands::music::resume::resume(),
                commands::music::shuffle::shuffle(),
                commands::music::skip::skip(),
                commands::music::stop::stop(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;
    client.unwrap().start().await.unwrap();
}