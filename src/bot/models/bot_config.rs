use serde::Deserialize;
use telegram_bot::User;

#[derive(Deserialize, Debug)]
pub struct BotConfig {
    pub token: String,
    pub users: Vec<User>,
}
