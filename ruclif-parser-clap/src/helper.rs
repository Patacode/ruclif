use crate::common_error_message;

pub fn build_missing_mandatory_fields_error_message(
    error_map: &[(&'static str, bool)],
) -> String {
    let missing_fields: Vec<&str> = error_map
        .iter()
        .filter_map(|entry| entry.1.then_some(entry.0))
        .collect();

    common_error_message::MANDATORY_FIELDS_MISSING
        .replace("{fields}", &missing_fields.join(", "))
}
