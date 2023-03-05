use std::str::FromStr;

use chrono::prelude::*;
use num_traits::FromPrimitive;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateFilterExt, UpdateHandler},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

mod misc;
use crate::misc::{bot_dialogues::State, consts};
mod helpers;

type CurrentDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Start the dialogue with the bot.")]
    Start,
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![State::Start].branch(case![Command::Start].endpoint(start)));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::HandleParadeState].endpoint(handle_parade_state))
        .branch(case![State::GetName].endpoint(get_name))
        .branch(case![State::GetMonth { name }].endpoint(get_month));

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![State::PerformAction].endpoint(perform_action));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn start(bot: Bot, dialogue: CurrentDialogue) -> HandlerResult {
    let choices = ["PS", "Duties"].map(|choice| InlineKeyboardButton::callback(choice, choice));

    bot.send_message(
        dialogue.chat_id(),
        concat!(
            "START",
            "\n\n",
            "- Select PS to format parade state messages.",
            "\n\n",
            "- Select Duties to retrieve duty information for specified GA."
        ),
    )
    .reply_markup(InlineKeyboardMarkup::new([choices]))
    .await?;

    dialogue.update(State::PerformAction).await?;

    Ok(())
}

async fn perform_action(bot: Bot, dialogue: CurrentDialogue, q: CallbackQuery) -> HandlerResult {
    if let Some(choice) = &q.data {
        match choice.as_str() {
            "PS" => {
                bot.send_message(
                    dialogue.chat_id(),
                    "Copy and paste the parade state message here.",
                )
                .await?;
                dialogue.update(State::HandleParadeState).await?;
            }
            "Duties" => {
                bot.send_message(dialogue.chat_id(), "Type your name.")
                    .await?;
                dialogue.update(State::GetName).await?;
            }
            _ => dialogue.update(State::Start).await?,
        }
    }

    Ok(())
}

async fn handle_parade_state(bot: Bot, dialogue: CurrentDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(parade_state) => {
            let formatted_parade_state = helpers::format_parade_state(parade_state);
            bot.send_message(dialogue.chat_id(), formatted_parade_state)
                .await?;
            dialogue.exit().await?;
        }
        None => {}
    }
    Ok(())
}

async fn get_name(bot: Bot, dialogue: CurrentDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(name) => {
            bot.send_message(
                dialogue.chat_id(),
                "Type the month. (1-12, Jan-Dec, January-December)",
            )
            .await?;
            dialogue.update(State::GetMonth { name }).await?;
        }
        None => {}
    }
    Ok(())
}

async fn get_month(
    bot: Bot,
    dialogue: CurrentDialogue,
    name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(month) => {
            let month_parser_int = month.parse::<u32>();
            let mut month_detected: Option<Month> = None;
            match month_parser_int {
                Ok(month_int) => match Month::from_u32(month_int) {
                    Some(month_object) => {
                        month_detected = Some(month_object);
                    }
                    None => {
                        bot.send_message(dialogue.chat_id(), consts::INVALID_MONTH_INT_MESSAGE).await?;
                        dialogue.exit().await?
                    }
                },
                Err(_) => match Month::from_str(month.as_str()) {
                    Ok(month_object) => {
                        month_detected = Some(month_object);
                    }
                    Err(_) => {
                        bot.send_message(dialogue.chat_id(), consts::INVALID_MONTH_STR_MESSAGE).await?;
                        dialogue.exit().await?
                    }
                },
            }

            match month_detected {
                Some(month_object) => {
                    bot.send_message(dialogue.chat_id(), helpers::find_duties(name, month_object).await).await?;
                    dialogue.exit().await?
                }
                None => {}
            }
        }
        None => {}
    }
    Ok(())
}
