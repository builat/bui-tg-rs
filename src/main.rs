use futures::StreamExt;
use getopts::Options;
use telegram_bot::*;
mod bot;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "set config file name", "");
    let match_result = match opts.parse(&args[1..]) {
        Ok(results) => results.opt_str("c"),
        Err(_) => None,
    };

    let config = bot::helpers::read_init_config(match_result);
    let api = Api::new(config.unwrap().token);
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            bot::commands::match_command(api.clone(), message).await?;
        }
    }
    return Ok(());
}
