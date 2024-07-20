use crate::traits::MentionsLightbot;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::{Message, Reaction};
use serenity::model::gateway::Ready;
use symspell::{AsciiStringStrategy, SymSpell, Verbosity};
use tracing::{error, info};

pub struct Bot {
  pub symspell: SymSpell<AsciiStringStrategy>,
}

#[async_trait]
impl EventHandler for Bot {
  async fn message(&self, ctx: Context, msg: Message) {
    // if msg.content == "!hello" {
    //   if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
    //     error!("Error sending message: {:?}", e);
    //   }
    // }
    if !msg.mentions_lightbot() {
      return
    }
    let content = &msg.content[22..]
      .trim();
    info!("[m] '{}'", content);
    let corrected = &self.symspell.lookup_compound(content, 2)[0];
    info!("[m] c: '{}' (dst: {})", corrected.term, corrected.distance);
  }
  // async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
  // }
  async fn ready(&self, _: Context, ready: Ready) {
    info!("{} is connected!", ready.user.name);
  }
}

