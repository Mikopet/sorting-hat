use anyhow::Context as _;
use reqwest::StatusCode;
use serenity::all::{ReactionType, RoleId};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info, warn};

struct Bot {
    api_url: String,
    api_secret: String,
    role_id: u64,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!activate" {
            // Reacting to notice user we are working
            let _ = msg
                .react(&ctx.http, ReactionType::Unicode("👀".to_string()))
                .await;

            // Calling API
            let client = reqwest::Client::new();
            let url = format!("{}{}", self.api_url, msg.author.name);
            let response = client
                .get(&url)
                .header("Authorization", format!("Basic {}", self.api_secret))
                .send()
                .await;

            // Checking the API response
            let mut answer = "yay";
            match response {
                Err(e) => error!("ERROR: {e}"),
                Ok(q) => match q.status() {
                    StatusCode::UNAUTHORIZED => {
                        error!("Unauthorized call to API: {url}");
                        answer = "It seems like there is an error to the connection with the API";
                    }
                    StatusCode::NOT_FOUND => {
                        warn!("Username not found: {url}");
                        answer = "No record found with your username! Add it on the website!";
                    }
                    StatusCode::OK => {
                        info!("User found! ({url})");
                        match ctx
                            .http
                            .add_member_role(
                                msg.guild_id.unwrap(),
                                msg.author.id,
                                RoleId::new(self.role_id),
                                Some("bot activation"),
                            )
                            .await
                        {
                            Ok(_) => info!("role added"),
                            Err(e) => error!("Error during role addition: {e}"),
                        }

                        answer = "Welcome!";
                    }
                    _ => error!("not handled status code: {}", q.status()),
                },
            };

            if let Err(e) = msg.channel_id.say(&ctx.http, answer).await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: shuttle_runtime::SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secret_store
        .get("DISCORD_TOKEN")
        .context("DISCORD_TOKEN was not found")?;

    let role_id = secret_store
        .get("DISCORD_ROLE_ID")
        .context("DISCORD_ROLE_ID was not found")?
        .parse::<u64>()
        .context("DISCORD_ROLE_ID is not valid")?;

    let api_url = secret_store
        .get("API_URL")
        .context("API_URL was not found")?;

    let api_secret = secret_store
        .get("API_SECRET")
        .context("API_SECRET was not found")?;

    let bot = Bot {
        api_url,
        api_secret,
        role_id,
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
