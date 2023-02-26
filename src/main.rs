use std::path::Path;
use std::str::FromStr;

use chrono::NaiveDate;
use regex::Regex;
use teloxide::types::InputFile;
use teloxide::{prelude::*, utils::command::BotCommands};

mod misc;
use crate::misc::consts;
use crate::misc::enums;
use crate::misc::structs;

mod helpers;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "\nDisplay this text.\n")]
    Help,
    #[command(description = "\nFormat parade state message. 
                            \nUsage: [/ps <parade_state_message>]\n")]
    PS(String),
    #[command(
        description = "\nShow all duties for specified person.
                        \nUsage: [/duties <name> <month>]
                        \n- Replace spaces in name with underscore. Upper or lowercase does not matter.
                        \n- Enter either name (January-December) or number (1-12) of month.\n",
        parse_with = "split"
    )]
    Duties(String, String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        
        Command::PS(parade_state) => {
            let day_date_regex: Regex = Regex::new(r"\*(?P<day>[a-zA-Z]+)\*\s(?P<date>\d{6})")
                .expect(consts::DAY_DATE_REGEX_ERROR_MESSAGE);
            let absences_details_regex =
                Regex::new(r"\d\.\s(?P<rank>[a-zA-Z\d]{3,4})\s(?P<name>.+)\s\((?P<details>.+)\)")
                    .expect(consts::ABSENCES_DETAILS_REGEX_ERROR_MESSAGE);

            let day_date_capture = day_date_regex.captures(&parade_state);
            let absences_details_captures = absences_details_regex.captures_iter(&parade_state);

            // Absence details gather and sort by rank.
            let mut absences_details: Vec<structs::AbsenceInfo> = vec![];
            for absence_details in absences_details_captures {
                absences_details.push(structs::AbsenceInfo {
                    rank: enums::Ranks::from_str(format!("R{}", &absence_details["rank"]).as_str())
                        .unwrap_or(enums::Ranks::RUNKNOWN),
                    name: String::from(&absence_details["name"]),
                    details: String::from(&absence_details["details"]),
                });
            }

            // If message is valid parade state message.
            if day_date_capture.is_some() || !absences_details.is_empty() {
                let day_date_values = day_date_capture.unwrap();
                let day_date_formatted =
                    format!("{} {}", &day_date_values["day"], &day_date_values["date"]);
                let day_date = NaiveDate::parse_from_str(&day_date_formatted, "%A %d%m%y")
                    .expect(consts::DATE_FORMATTING_ERROR_MESSAGE);

                // Construct and render the table of parade state info.
                absences_details.swap_remove(0); // Remove 3WO Martin's example.
                helpers::construct_absences_details_table(&mut absences_details, &day_date);
                helpers::render_html_table();

                bot.send_photo(
                    msg.chat.id,
                    InputFile::file(Path::new(consts::TABLE_IMAGE_FILEPATH).as_os_str()),
                )
                .await?
            } else {
                bot.send_message(msg.chat.id, consts::INVALID_PARADE_STATE_ERROR_MESSAGE)
                    .await?
            }
        }
        
        Command::Duties(_name, _month) => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await? //TODO
        }
    };
    Ok(())
}
