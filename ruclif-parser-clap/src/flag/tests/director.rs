use crate::{
    flag::{FlagAction, FlagClapArgData},
    ClapNamedArgData,
};

pub mod defaults {
    use super::*;

    pub const NAME: &str = "AUTHOR";
    pub const SHORT: char = 'a';
    pub const LONG: &str = "author";
    pub const DESCRIPTION: &str = "Print author";
    pub const ACTION: FlagAction = FlagAction::SetFalse;
}

pub struct TestData {
    arg_data: FlagClapArgData,
}

impl TestData {
    pub fn name(&self) -> &'static str {
        self.arg_data.common.name
    }

    pub fn short(&self) -> char {
        self.arg_data.common.short
    }

    pub fn long(&self) -> &'static str {
        self.arg_data.common.long
    }

    pub fn description(&self) -> &'static str {
        self.arg_data.common.description
    }

    pub fn action(&self) -> &FlagAction {
        &self.arg_data.action
    }
}

impl TestData {
    pub fn with_all_fields_set() -> Self {
        Self {
            arg_data: FlagClapArgData {
                common: ClapNamedArgData {
                    name: defaults::NAME,
                    short: defaults::SHORT,
                    long: defaults::LONG,
                    description: defaults::DESCRIPTION,
                },
                action: defaults::ACTION,
            },
        }
    }
}
