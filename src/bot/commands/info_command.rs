use super::signals;
use telegram_bot::{
    Api, CanGetUserProfilePhotos, CanReplySendMessage, Error, Message, MessageText,
};

pub async fn count_profile_photos(api: Api, message: Message) -> Result<(), Error> {
    let photos = api.send(message.from.get_user_profile_photos()).await?;
    api.send(message.text_reply(format!("Found photos: {}", photos.total_count)))
        .await?;
    Ok(())
}

pub async fn count_letters(api: Api, message: Message) -> Result<(), Error> {
    let signal_basis = signals::COUNT_MSG_LETTERS.chars().count() + 1;
    api.send(message.text_reply(format!(
        "Length of your message: {}",
        message.text().unwrap().chars().count() - signal_basis
    )))
    .await?;
    Ok(())
}
