use serde::{Serialize, Deserialize};
use super::enums::Ranks;

pub struct AbsenceInfo {
    pub rank: Ranks,
    pub name: String,
    pub details: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SheetFields {
    pub date: String,
    pub od: String,
    pub cffs: String,
    #[serde(rename = "GUARD DUTY")]
    pub guard_duty: String,
    #[serde(default)]
    pub others: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SheetPropertiesL0 {
    pub sheets: Vec<SheetPropertiesL1>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SheetPropertiesL1 {
    pub properties: SheetPropertiesL2
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetPropertiesL2 {
    pub sheet_id: u32,
    pub title: String,
    pub index: u32,
    pub sheet_type: String,
    pub grid_properties: SheetPropertiesL3
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetPropertiesL3 {
    pub row_count: u32,
    pub column_count: u32
}


