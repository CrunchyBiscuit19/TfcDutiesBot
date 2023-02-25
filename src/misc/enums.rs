use std::fmt;
use strum_macros::EnumString;

// TODO remember to use discriminants in the enums
pub enum Months {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    Current,
}

// All ranks prefixed with 'R' ton prevent starting with digit.
#[derive(Copy, Clone, Debug, EnumString)]
pub enum Ranks {
    RLTC,
    RMAJ,
    RCPT,
    RLTA,
    R2LT,
    R3WO,
    RMSG,
    RSSG,
    R3SG,
    RCFC,
    RCPL,
    RLCP,
    RPTE,
    RUNKNOWN,
}

impl fmt::Display for Ranks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}