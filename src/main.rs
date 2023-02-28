use std::str::FromStr;

use chrono::prelude::*;
use num_traits::FromPrimitive;
use regex::Regex;
use teloxide::{prelude::*, utils::command::BotCommands};
use titlecase::titlecase;

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
    PS { parade_state: String },
    #[command(
        description = "\nShow all duties for specified person.
                        \nUsage: [/duties <name> <month>]
                        \n- Replace spaces in name with underscore. 
                        \n- Enter either name January-December / Jan-Dec or number 1-12 of month.
                        \n- All arguments are case insensitive.\n",
        parse_with = "split"
    )]
    Duties { name: String, month: String },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }

        Command::PS { parade_state } => {
            let mut reply_message =
                String::from(format!("{}\n\n", consts::PARADE_STATE_TITLE).as_str());

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

                absences_details.swap_remove(0); // Remove 3WO Martin's example.
                helpers::sort_absences_details(&mut absences_details);

                reply_message.push_str(day_date.format("%A %d-%m-%Y").to_string().as_str());
                if !absences_details.is_empty() {
                    for absence_details in absences_details {
                        reply_message.push_str("\n\n");
                        reply_message.push_str(
                            format!(
                                "{} | {}\n",
                                absence_details.rank.to_string()[1..].to_owned(),
                                absence_details.name
                            )
                            .as_str(),
                        );
                        reply_message.push_str(absence_details.details.as_str());
                    }
                } else {
                    reply_message
                        .push_str(format!("\n\n{}", consts::NO_ABSENTEES_MESSAGE).as_str());
                }

                bot.send_message(msg.chat.id, reply_message).await?
            } else {
                reply_message.push_str(
                    format!("\n\n{}", consts::INVALID_PARADE_STATE_ERROR_MESSAGE).as_str(),
                );
                bot.send_message(msg.chat.id, reply_message).await?
            }
        }

        Command::Duties { name, month } => {
            let mut reply_message = String::from(format!("{}\n\n", consts::DUTIES_TITLE).as_str());

            let month_parser_int = month.parse::<u32>();
            let mut month_detected: Option<Month> = None;

            match month_parser_int {
                Ok(month_int) => match Month::from_u32(month_int) {
                    Some(month_object) => {
                        month_detected = Some(month_object);
                    }
                    None => {
                        reply_message.push_str(consts::INVALID_MONTH_INT_MESSAGE);
                    }
                },
                Err(_) => match Month::from_str(month.as_str()) {
                    Ok(month_object) => {
                        month_detected = Some(month_object);
                    }
                    Err(_) => {
                        reply_message.push_str(consts::INVALID_MONTH_STR_MESSAGE);
                    }
                },
            }

            match month_detected {
                Some(month_object) => {
                    let month_query = month_object.name();
                    let name_query = name.replace("_", " ").to_lowercase();
                    //TODO search spreadsheet in here
                    reply_message.push_str(
                        format!(
                            "{} has no duties for {}",
                            titlecase(name_query.as_str()),
                            month_query
                        )
                        .as_str(),
                    );
                }
                None => {}
            }

            bot.send_message(msg.chat.id, reply_message).await?
        }
    };
    Ok(())
}
