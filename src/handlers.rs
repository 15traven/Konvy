use std::error::Error;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, InlineKeyboardButton}
};
use regex::Regex;
use crate::consts;

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn handle_start_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "👋 Hello!\nThis bot was created to help you quickly and easily convert different units between each other.").await?;
    handle_help_command(bot, msg).await?;
    
    Ok(())
}

pub async fn handle_help_command(bot: Bot, msg: Message) -> HandlerResult {
    let url = url::Url::parse(consts::HELP_DOC_LINK)?;
    let keyboard = InlineKeyboardMarkup::new([[
        InlineKeyboardButton::url("Click here", url)
    ]]);
    
    bot.send_message(msg.chat.id, "🤔 How to use this bot?\nSimply send your request.\nFor example\n\n34kg in lb\n\nTo check all supported units and their shortcodes, click the button below.")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

pub async fn handle_convert_request(bot: Bot, msg: Message) -> HandlerResult {
    let re = Regex::new(r"^(\d+(?:\.\d+)?)\s*(\w+)\s+in\s+(\w+)$").unwrap();

    if let Some(caps) = &re.captures(msg.text().unwrap()) {
        let (from_category, from_factor) = match consts::FACTORS.get(&caps[2]) {
            Some((c, f)) => (c, f),
            None => {
                bot.send_message(msg.chat.id, format!("❌ Unsupported unit: {}", &caps[2])).await?;
                return Ok(())
            }
        };
        let (to_category, to_factor) = match consts::FACTORS.get(&caps[3]) {
            Some((c, f)) => (c, f),
            None => {
                bot.send_message(msg.chat.id, format!("❌ Unsupported unit: {}", &caps[3])).await?;
                return Ok(())
            }
        };
        
        if from_category != to_category {
            bot.send_message(msg.chat.id, format!("❌ Cannot convert {} to {} (different categories)", &caps[2], &caps[3])).await?;
            return Ok(())
        }

        let value = caps[1].parse::<f64>().unwrap();
        
        let result = value * (from_factor / to_factor);
        bot.send_message(msg.chat.id, result.to_string()).await?;
    };

    Ok(())
}
