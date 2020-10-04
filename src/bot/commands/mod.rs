use telegram_bot::{Api, Error, Message, MessageKind};
mod fallback_cmd;
mod info_command;
mod signals;

pub async fn match_command(api: Api, message: Message) -> Result<(), Error> {
    match message.kind {
        MessageKind::Text { ref data, .. } => {
            let data_as_str = data.as_str();
            if !data_as_str.contains("/") {
                return Ok(());
            }
            let may_be_cmd: Vec<&str> = data_as_str.split_whitespace().collect();
            let cmd_to_parse: &str = if may_be_cmd[0].is_empty() {
                data_as_str
            } else {
                may_be_cmd[0]
            };
            /* Maping of commands to related functions
               In case of unmatched command send fallback message
            */
            match cmd_to_parse {
                signals::COUNT_PHOTOS => info_command::count_profile_photos(api, message).await?,
                signals::COUNT_MSG_LETTERS => info_command::count_letters(api, message).await?,
                _ => fallback_cmd::unmatched(api, message).await?,
            }
        }
        _ => (),
    };

    Ok(())
}
