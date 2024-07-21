use crate::traits::MentionsLightbot;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::{Message, Reaction};
use serenity::model::gateway::Ready;
use symspell::{AsciiStringStrategy, SymSpell, Verbosity};
use tracing::{error, info};

pub struct Bot {
  pub symspell: SymSpell<AsciiStringStrategy>,
  pub tags: serde_json::Value,
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
    // spellcheck
    info!("[m]   '{}'", content);
    let corrected = &self.symspell.lookup_compound(content, 2)[0];
    info!("[m] = '{}' (dst: {})", corrected.term, corrected.distance);
    // get all tags
    let serde_json::Value::Object(ref all_tags) = self.tags else { unreachable!(); };
    // sort phrases (keys) longest first
    let mut phrases = all_tags.keys().cloned()
      .collect::<Vec<String>>();
    phrases.sort_by(|a, b| {
      let al = a.len();
      let bl = b.len();
      bl.cmp(&al)
    });
    // create tag pattern
    let mut tag_pattern: Vec<String> = vec![];
    let mut temp = corrected.term.clone();
    while temp.len() > 0 {
      let Some(lmp) = phrases.iter().find(|x| temp.starts_with(&**x)) else {
        // remove the first word if it does not resolve to any tag
        let temp2 = temp.split(" ").collect::<Vec<&str>>();
        let Some(first_word) = temp2.first() else { unreachable!(); };
        let l = first_word.to_string().len();
        temp = temp[l..].trim().to_string();
        continue;
      };
      let l = lmp.len();
      // add the tag
      let serde_json::Value::String(tag) = &all_tags[lmp] else { unreachable!(); };
      tag_pattern.push(tag.to_string());
      // remove the matching phrase
      temp = temp[l..].trim().to_string();
    }
    info!("[t]   '{:?}'", tag_pattern);
  }
  // async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
  // }
  async fn ready(&self, _: Context, ready: Ready) {
    info!("{} is connected!", ready.user.name);
  }
}

