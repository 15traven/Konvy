use std::error::Error;
use teloxide::prelude::*;

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn handle_start_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "👋 Hello!\n\nThis bot was created to help you quickly and easily convert different units between each other.\n\nTo use, simply send your request.\nFor example\n\n34kg in pounds").await?;

    Ok(())
}

pub async fn handle_convert_request(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Handled").await?;

    Ok(())
}
