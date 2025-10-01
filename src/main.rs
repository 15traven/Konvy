use teloxide::{
    prelude::*,
    macros::BotCommands
};
use dptree::case;
use handlers::*;

mod handlers;
mod consts;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command()]
    Start
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    let schema = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .branch(case![Command::Start].endpoint(handle_start_command))
        )
        .branch(
            Update::filter_message()
                .endpoint(handle_convert_request)
        );

    Dispatcher::builder(bot, schema)
        .build()
        .dispatch()
        .await;
}
