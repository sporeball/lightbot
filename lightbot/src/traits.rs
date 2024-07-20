use serenity::model::channel::Message;

pub trait MentionsLightbot {
  fn mentions_lightbot(&self) -> bool;
}

impl MentionsLightbot for Message {
  fn mentions_lightbot(&self) -> bool {
    self.content.starts_with("<@1263328534030454875>")
  }
}
