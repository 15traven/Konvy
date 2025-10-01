use std::error::Error;
use std::collections::HashMap;
use teloxide::prelude::*;
use regex::Regex;
use crate::consts::FACTORS;

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn handle_start_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "👋 Hello!\n\nThis bot was created to help you quickly and easily convert different units between each other.\n\nTo use, simply send your request.\nFor example\n\n34kg in pounds").await?;

    Ok(())
}

pub async fn handle_convert_request(bot: Bot, msg: Message) -> HandlerResult {
    let re = Regex::new(r"^(\d+(?:\.\d+)?)\s*(\w+)\s+in\s+(\w+)$").unwrap();
    if let Some(caps) = &re.captures(msg.text().unwrap()) {
        let from_factor = FACTORS.get(&caps[2]).cloned().unwrap();
        let to_factor = FACTORS.get(&caps[3]).cloned().unwrap();
        let value = caps[1].parse::<f64>().unwrap();
        
        let result = value * (from_factor / to_factor);
        bot.send_message(msg.chat.id, result.to_string()).await?;
    };

    Ok(())
}
