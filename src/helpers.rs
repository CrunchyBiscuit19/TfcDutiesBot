use crate::misc::structs;

pub fn sort_absences_details (absences_details: &mut Vec<structs::AbsenceInfo>) {
    absences_details.sort_by_key(|absence_details| {
        (
            absence_details.rank as u32,
            absence_details.name.to_lowercase(),
        )
    });
}