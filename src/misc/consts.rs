/*pub const HELP_COMMAND_DESCRIPTION: &str = "\nDisplay this text.\n";
pub const PS_COMMAND_DESCRIPTION: &str = "\nFormat parade state message. [/ps <parade_state_message>]\n";
pub const DUTIES_COMMAND_DESCRIPTION: &str = "\nShow all duties for specified person. [/duties <name> <month>]\n
                                                Replace spaces in name with underscore. Upper or lowercase does not matter.\n
                                                Enter either name or number of month.\n";*/

pub const PARADE_STATE_TITLE: &str = "PARADE STATE";
pub const DUTIES_TITLE: &str = "DUTIES";

pub const DAY_DATE_REGEX_ERROR_MESSAGE: &str = "Hardcoded day-date regex should be valid.";
pub const ABSENCES_DETAILS_REGEX_ERROR_MESSAGE: &str = "Hardcoded list regex should be valid.";
pub const DATE_FORMATTING_ERROR_MESSAGE: &str = "Date formatting has failed.";
pub const INVALID_PARADE_STATE_ERROR_MESSAGE: &str = "Parade state message not valid.";

pub const NO_ABSENTEES_MESSAGE: &str = "No absentees.";
pub const INVALID_MONTH_INT_MESSAGE: &str = "Month not within 1 and 12.";
pub const INVALID_MONTH_STR_MESSAGE: &str = "Cannot determine month from argument.";

pub const PRIVATE_KEY_NOT_FOUND_MESSAGE: &str = "Cannot find private key file.";
pub const AUTHENTICATOR_FAILED_ERROR_MESSAGE: &str = "OAuth2 authenticator failed.";
pub const SPREADSHEET_RETRIEVAL_FAILED_MESSAGE: &str = "Cannot retrieve spreadsheet.";

pub const DUTIES_SPREADSHEET_ID: &str = "1aCHk12Xrw8TqUXdR1e7m6LkS3SBK6LsnzQmRwzKryQU";
pub const CDO_SPREADSHEET_ID: &str = "1iOKqte4heZtyyC-aeQtBaddS1lfQiO_J50ortTmyefo";