use crate::ClapArgData;

mod impls;

#[cfg(test)]
mod tests;

pub type ValueParserFunc = fn(&str) -> Result<String, String>;

#[derive(Debug, PartialEq, Eq)]
pub struct StringClapArgData {
    common: ClapArgData,                   // 56 bytes
    default_value: Option<&'static str>,   // 16 bytes
    value_parser: Option<ValueParserFunc>, // 8 bytes
} // 80 bytes

#[derive(Debug, PartialEq, Eq)]
pub struct StringClapArg {
    data: StringClapArgData, // 80 bytes
} // 80 bytes

#[derive(Default, Debug, PartialEq, Eq)]
pub struct StringClapArgBuilder {
    name: Option<&'static str>,            // 16 bytes
    short: Option<char>,                   // 4 bytes
    long: Option<&'static str>,            // 16 bytes
    description: Option<&'static str>,     // 16 bytes
    default_value: Option<&'static str>,   // 16 bytes
    value_parser: Option<ValueParserFunc>, // 8 bytes
    state: u8,
} // 80 bytes

mod builder_state {
    pub const NAME: usize = 0b1;
    pub const SHORT: usize = 0b10;
    pub const LONG: usize = 0b100;
    pub const DESCRIPTION: usize = 0b1000;
    pub const EXPECTED: u8 = 0b1111;
}

mod error_message {
    pub const MANDATORY_FIELDS_MISSING: &str = "Following mandatory fields are missing: {fields}";
}
