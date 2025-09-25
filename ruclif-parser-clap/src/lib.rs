pub mod flag;
pub mod helper;
pub mod list;
pub mod string;

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Clone)]
struct NamedArgData {
    name:        &'static str,
    short:       char,
    long:        &'static str,
    description: &'static str,
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Clone)]
struct PosArgData {
    name:        &'static str,
    description: &'static str,
}

#[rustfmt::skip]
#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct ArgBuilderData {
    name:        Option<&'static str>,
    short:       Option<char>,
    long:        Option<&'static str>,
    description: Option<&'static str>,
}

#[rustfmt::skip]
mod common_arg_builder_state {
    pub const NAME:        usize = 0b0001;
    pub const SHORT:       usize = 0b0010;
    pub const LONG:        usize = 0b0100;
    pub const DESCRIPTION: usize = 0b1000;
    pub const EXPECTED:    u8    = 0b1111;
}

#[rustfmt::skip]
mod common_error_message {
    pub const MANDATORY_FIELDS_MISSING: &str = "Following mandatory fields are missing: {fields}";
}
