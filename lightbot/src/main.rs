// use lightbot::json_to_hashmap;
use lightbot::bot::Bot;
use std::fs::read_to_string;
use anyhow::Context as _;
use serde_json::Value;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use symspell::{AsciiStringStrategy, SymSpell};
use tracing::info;

#[shuttle_runtime::main]
async fn serenity(
  #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
  // Get the discord token set in `Secrets.toml`
  let token = secrets
    .get("DISCORD_TOKEN")
    .context("'DISCORD_TOKEN' was not found")?;

  // Set gateway intents, which decides what events the bot will be notified about
  let intents = GatewayIntents::GUILD_MESSAGE_REACTIONS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

  let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();
  info!("loading dictionary...");
  symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
  info!("loaded dictionary!");
  info!("loading bigram dictionary...");
  symspell.load_bigram_dictionary("data/frequency_bigramdictionary_en_243_342.txt", 0, 2, " ");
  info!("loaded bigram dictionary!");

  info!("loading tags...");
  let tags_json = read_to_string("data/tags.json").expect("could not read tags.json");
  let tags: Value = serde_json::from_str(&tags_json).expect("could not parse tags.json");
  // info!("tags: {:#?}", tags);
  info!("loaded tags!");

  let bot = Bot {
    symspell,
    tags,
  };

  info!("starting...");

  let client = Client::builder(&token, intents)
    .event_handler(bot)
    .await
    .expect("Err creating client");

  Ok(client.into())
}
