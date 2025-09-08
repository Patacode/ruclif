use crate::ClapNamedArgData;

mod impls;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FlagAction {
    SetTrue,
    SetFalse,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FlagClapArgData {
    common: ClapNamedArgData,
    action: FlagAction,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FlagClapArg {
    data: FlagClapArgData,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct FlagClapArgBuilder {
    name: Option<&'static str>,
    short: Option<char>,
    long: Option<&'static str>,
    description: Option<&'static str>,
    action: Option<FlagAction>,
    state: u8,
}

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
