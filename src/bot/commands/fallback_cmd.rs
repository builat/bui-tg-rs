use telegram_bot::{Api, CanReplySendMessage, Error, Message};

pub async fn unmatched(api: Api, message: Message) -> Result<(), Error> {
    api.send(message.text_reply("No matched cmd found")).await?;
    Ok(())
}
