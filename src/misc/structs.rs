use super::enums::Ranks;

pub struct AbsenceInfo {
    pub rank: Ranks,
    pub name: String,
    pub details: String,
}

impl AbsenceInfo {
    pub fn field_names() -> [String;3] {
        [String::from("RANK"), String::from("NAME"), String::from("DETAILS")]
    }
}