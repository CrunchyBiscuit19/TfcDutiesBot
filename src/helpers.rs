use dunce;
use std::fs::OpenOptions;
use std::io::Write;

use build_html::{Html, HtmlContainer, HtmlPage, Table};
use chrono::NaiveDate;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;

use crate::misc::consts;
use crate::misc::structs;

pub fn construct_absences_details_table(
    absences_details: &mut Vec<structs::AbsenceInfo>,
    day_date: &NaiveDate,
) {
    //absences_details.sort_by(|a, b| (a.rank as u32).cmp(&(b.rank as u32)));
    absences_details.sort_by_key(|absences_detail| {
        (
            absences_detail.rank as u32,
            absences_detail.name.to_lowercase(),
        )
    });

    let mut absences_details_arranged = vec![];
    for absence_details in absences_details {
        let rank = &absence_details.rank.to_string()[1..];
        absences_details_arranged.push(vec![
            String::from(rank),
            absence_details.name.to_owned(),
            absence_details.details.to_owned(),
        ]);
    }

    let absences_details_table_html = HtmlPage::new()
        .with_stylesheet(consts::TABLE_STYLESHEET_FILENAME)
        .with_header(3, day_date.format("%A %d-%m-%Y"))
        .with_table(
            Table::from(absences_details_arranged)
                .with_header_row(structs::AbsenceInfo::field_names())
                .with_attributes([("id", consts::HTML_SCREENSHOT_ID)]),
        )
        .to_html_string();

    let mut table_html_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(consts::TABLE_HTML_FILEPATH)
        .expect(consts::OPEN_TABLE_HTML_ERROR);

    table_html_file
        .write_all(absences_details_table_html.as_bytes())
        .expect(consts::WRITE_TABLE_HTML_ERROR);
}

pub fn render_html_table() {
    let browser: Browser = Browser::default().expect(consts::OPEN_HEADLESS_BROWSER_ERROR);
    let browser_tab = browser.new_tab().expect(consts::OPEN_HEADLESS_TAB_ERROR);
    browser_tab
        .navigate_to(
            format!(
                "{}",
                dunce::canonicalize(consts::TABLE_HTML_FILEPATH)
                    .unwrap()
                    .as_path()
                    .display()
            )
            .as_str(),
        )
        .expect(consts::NAVIGATE_TABLE_HTML_FILEPATH);

    let viewport = browser_tab
        .wait_for_element(format!("#{}", consts::HTML_SCREENSHOT_ID).as_str())
        .expect(consts::TABLE_HTML_ELEMENT_NOT_FOUND)
        .get_box_model()
        .unwrap()
        .margin_viewport();

    let screenshot_data = browser_tab
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png, Some(1), Some(viewport), true)
        .expect(consts::TABLE_HTML_SCREENSHOT_ERROR);

    let mut table_png_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(consts::TABLE_IMAGE_FILEPATH)
        .expect(consts::OPEN_TABLE_IMAGE_ERROR);

    table_png_file
        .write_all(screenshot_data.as_slice())
        .expect(consts::WRITE_TABLE_IMAGE_ERROR);
}
