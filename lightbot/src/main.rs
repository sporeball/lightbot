use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use symspell::{AsciiStringStrategy, SymSpell, Verbosity};
use tracing::{error, info};

struct Bot {
  symspell: SymSpell<AsciiStringStrategy>,
}

#[async_trait]
impl EventHandler for Bot {
  async fn message(&self, ctx: Context, msg: Message) {
    // if msg.content == "!hello" {
    //   if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
    //     error!("Error sending message: {:?}", e);
    //   }
    // }
    // info!("lightbot: msg.content  -> bool= {}", msg.content);
    // info!("lightbot: msg.mentions = {:?}", msg.mentions);
    if !msg.mentions_lightbot() {
      return
    }
    let content = &msg.content[22..]
      .trim();
    info!("[m] '{}'", content);
    let corrected = &self.symspell.lookup_compound(content, 2)[0];
    info!("[m] c: '{}' (dst: {})", corrected.term, corrected.distance);
  }
  async fn ready(&self, _: Context, ready: Ready) {
    info!("{} is connected!", ready.user.name);
  }
}

trait MentionsLightbot {
  fn mentions_lightbot(&self) -> bool;
}

impl MentionsLightbot for Message {
  fn mentions_lightbot(&self) -> bool {
    self.content.starts_with("<@1263328534030454875>")
  }
}

#[shuttle_runtime::main]
async fn serenity(
  #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
  // Get the discord token set in `Secrets.toml`
  let token = secrets
    .get("DISCORD_TOKEN")
    .context("'DISCORD_TOKEN' was not found")?;

  // Set gateway intents, which decides what events the bot will be notified about
  let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

  let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();
  info!("loading dictionary...");
  symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
  info!("loaded dictionary!");
  info!("loading bigram dictionary...");
  symspell.load_bigram_dictionary("data/frequency_bigramdictionary_en_243_342.txt", 0, 2, " ");
  info!("loaded bigram dictionary!");

  let bot = Bot {
    symspell,
  };

  let client = Client::builder(&token, intents)
    .event_handler(bot)
    .await
    .expect("Err creating client");

  Ok(client.into())
}
