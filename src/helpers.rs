use std::{env, str::FromStr};

use chrono::prelude::*;
use regex::Regex;
use serde_json;
use csv;

use crate::misc::{consts, enums, structs};

pub fn sort_absences_details(absences_details: &mut Vec<structs::AbsenceInfo>) {
    absences_details.sort_by_key(|absence_details| {
        (
            absence_details.rank as u32,
            absence_details.name.to_lowercase(),
        )
    });
}

pub fn format_parade_state(parade_state: String) -> String {
    let mut reply_message = String::from(format!("{}\n\n", consts::PARADE_STATE_TITLE).as_str());

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
        sort_absences_details(&mut absences_details);

        reply_message.push_str(day_date.format("%A %d-%m-%Y").to_string().as_str());
        reply_message.push_str(format!("\n\n{} Absentees", absences_details.len()).as_str());
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
            reply_message.push_str(format!("\n\n{}", consts::NO_ABSENTEES_MESSAGE).as_str());
        }
    } else {
        reply_message.push_str(consts::INVALID_PARADE_STATE_ERROR_MESSAGE);
    }

    reply_message
}

pub async fn get_sheet_ids(spreadsheet_id: &str) -> Option<Vec<u32>> {
    let mut sheet_ids = vec![];

    let resp = reqwest::get(format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}?&fields=sheets.properties&key={}",
        spreadsheet_id, env::var("GOOGLE_API_KEY").expect(consts::MISSING_API_KEY)
    ))
    .await;
    match resp {
        Ok(resp) => {
            let resp_data = resp.text().await.unwrap();
            let parsed_data: structs::SheetPropertiesL0 = serde_json::from_str(&resp_data).unwrap();
            for sheet_metadata in parsed_data.sheets {
                sheet_ids.push(sheet_metadata.properties.sheet_id);
            }
            Some(sheet_ids)
        },
        Err(_) => None,
    }
}

pub async fn get_sheet_records(spreadsheet_id: &str, sheet_id: u32) -> Option<Vec<structs::SheetFields>> {
    let mut sheet_records: Vec<structs::SheetFields> = vec![];

    let resp = reqwest::get(format!(
        "https://docs.google.com/spreadsheets/d/{0}/export?format=csv&id={0}&gid={1}",
        spreadsheet_id, sheet_id
    ))
    .await;
    match resp {
        Ok(resp) => {
            let resp_data = resp.text().await.unwrap();
            let mut csv_reader = csv::Reader::from_reader(resp_data.as_bytes());
            for result in csv_reader.deserialize() {
                sheet_records.push(result.unwrap());
            }
            Some(sheet_records)
        },
        Err(_) => None,
    }
}

pub fn get_duties(duties_records: Vec<structs::SheetFields>, name_query: &str) -> String {
    let mut duties_message = String::from("");

    for record in duties_records {
        let mut record_message = String::from("");

        if record.guard_duty.contains(name_query) {
            record_message.push_str("Guard Duty\n");
            let personnel: Vec<&str> = record.guard_duty.split("\n").filter(|p| !p.contains(name_query)).collect();
            record_message.push_str(format!("+ [{}]\n", personnel.join(", ")).as_str());
        }
        if record.cffs.contains(name_query) {
            record_message.push_str("CFFS\n");
            let personnel: Vec<&str> = record.cffs.split("\n").filter(|p| !p.contains(name_query)).collect();
            record_message.push_str(format!("+ [{}]\n", personnel.join(", ")).as_str());
        }
        if record.others.contains(name_query) {
            record_message.push_str("OTHERS\n");
            let personnel: Vec<&str> = record.others.split("\n").filter(|p| !p.contains(name_query)).collect();
            record_message.push_str(format!("+ [{}]\n", personnel.join(", ")).as_str());
        }
        if record.od.contains(name_query) {
            record_message.push_str("OD\n");
        }

        if record_message.len() > 0 {
            record_message.insert_str(0, format!("\n{}\n", record.date).as_str())
        }

        duties_message.push_str(record_message.as_str())
    }

    duties_message
}

pub async fn find_duties(name: String, month_object: Month) -> String {
    let mut reply_message = String::from(format!("{}\n", consts::DUTIES_TITLE).as_str());

    let month_query = month_object.number_from_month();
    let name_query = name.to_uppercase();
    let name_query = name_query.as_str();

    match get_sheet_ids(consts::DUTIES_SPREADSHEET_ID).await {
        Some(sheet_ids) => {
            if (month_query - 1) as usize >= sheet_ids.len() {
                reply_message.push_str(consts::MISSING_DUTIES_MONTH_SHEET);
            } else {
                match get_sheet_records(consts::DUTIES_SPREADSHEET_ID, sheet_ids[(month_query - 1) as usize]).await {
                    Some(duties_records) => {
                        reply_message.push_str(get_duties(duties_records, name_query).as_str());
                    },
                    None => reply_message.push_str(consts::MISSING_DUTIES_MONTH_SHEET)
                }
            }            
        },
        None => reply_message.push_str(consts::MISSING_DUTIES_MONTH_SHEET)
    }

    reply_message
}
