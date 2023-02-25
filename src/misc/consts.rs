/*pub const HELP_COMMAND_DESCRIPTION: &str = "\nDisplay this text.\n";
pub const PS_COMMAND_DESCRIPTION: &str = "\nFormat parade state message. [/ps <parade_state_message>]\n";
pub const DUTIES_COMMAND_DESCRIPTION: &str = "\nShow all duties for specified person. [/duties <name> <month>]\n
                                                Replace spaces in name with underscore. Upper or lowercase does not matter.\n
                                                Enter either name or number of month.\n";*/

pub const DAY_DATE_REGEX_ERROR_MESSAGE: &str = "Hardcoded day-date regex should be valid.";
pub const ABSENCES_DETAILS_REGEX_ERROR_MESSAGE: &str = "Hardcoded list regex should be valid.";
pub const DATE_FORMATTING_ERROR_MESSAGE: &str = "Date formatting has failed.";
pub const INVALID_PARADE_STATE_ERROR_MESSAGE: &str = "Parade state message not valid.";

pub const OPEN_TABLE_HTML_ERROR: &str = "Cannot open table HTML file for writing.";
pub const WRITE_TABLE_HTML_ERROR: &str = "Unable to write into table HTML file.";
pub const OPEN_TABLE_IMAGE_ERROR: &str = "Cannot open table PNG file for writing.";
pub const WRITE_TABLE_IMAGE_ERROR: &str = "Unable to write into table PNG file.";

pub const OPEN_HEADLESS_BROWSER_ERROR: &str = "Cannot open headless browser."; 
pub const OPEN_HEADLESS_TAB_ERROR: &str = "Cannot open headless tab.";
pub const NAVIGATE_TABLE_HTML_FILEPATH: &str = "Cannot navigate to table HTMl file in headless browser.";
pub const TABLE_HTML_ELEMENT_NOT_FOUND: &str = "Cannot find table HTML element by ID in headless browser.";
pub const TABLE_HTML_SCREENSHOT_ERROR: &str = "Cannot screenshot table HTML element in headless browser.";

pub const TABLE_IMAGE_FILEPATH: &str = "./resources/table.png";
pub const TABLE_HTML_FILEPATH: &str = "./resources/table.html";
pub const TABLE_STYLESHEET_FILENAME: &str = "table.css";
pub const HTML_SCREENSHOT_ID: &str = "screenshot_this";