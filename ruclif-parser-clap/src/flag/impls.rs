use clap::{
    builder::{IntoResettable, Resettable},
    Arg, ArgAction, ArgMatches,
};
use ruclif_core::{
    builder::{Builder, HasBuilder},
    common::IntoFrom,
};

use crate::{
    flag::{builder_state, error_message, FlagAction, FlagClapArg, FlagClapArgBuilder, FlagClapArgData},
    ClapNamedArgData,
};

impl HasBuilder for FlagClapArg {
    type Builder = FlagClapArgBuilder;

    fn builder() -> Self::Builder {
        FlagClapArgBuilder::default()
    }
}

impl FlagClapArgBuilder {
    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self.state |= builder_state::NAME as u8;
        self
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self.state |= builder_state::SHORT as u8;
        self
    }

    pub fn long(mut self, long: &'static str) -> Self {
        self.long = Some(long);
        self.state |= builder_state::LONG as u8;
        self
    }

    pub fn description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self.state |= builder_state::DESCRIPTION as u8;
        self
    }

    pub fn action(mut self, action: FlagAction) -> Self {
        self.action = Some(action);
        self
    }
}

impl FlagClapArgBuilder {
    fn is_name_set(&self) -> bool {
        self.state & builder_state::NAME as u8 != 0
    }

    fn is_short_set(&self) -> bool {
        self.state & builder_state::SHORT as u8 != 0
    }

    fn is_long_set(&self) -> bool {
        self.state & builder_state::LONG as u8 != 0
    }

    fn is_description_set(&self) -> bool {
        self.state & builder_state::DESCRIPTION as u8 != 0
    }

    fn build_error_message(&self) -> String {
        let map = [
            ("name", !self.is_name_set()),
            ("short", !self.is_short_set()),
            ("long", !self.is_long_set()),
            ("description", !self.is_description_set()),
        ];

        let missing_fields: Vec<&str> = map.iter().filter_map(|entry| entry.1.then_some(entry.0)).collect();
    
        error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", &missing_fields.join(", "))
    }
}

impl Builder for FlagClapArgBuilder {
    type Result = FlagClapArg;

    fn build(self) -> Result<FlagClapArg, String> {
        if self.state != builder_state::EXPECTED {
            Err(self.build_error_message())
        } else {
            let data = FlagClapArgData {
                common: ClapNamedArgData {
                    name: self.name.unwrap(),
                    short: self.short.unwrap(),
                    long: self.long.unwrap(),
                    description: self.description.unwrap(),
                },
                action: self.action.unwrap_or(FlagAction::SetTrue),
            };

            Ok(FlagClapArg { data })
        }
    }
}

impl FlagClapArg {
    fn name(&self) -> &'static str {
        self.data.common.name
    }

    fn short(&self) -> char {
        self.data.common.short
    }

    fn long(&self) -> &'static str {
        self.data.common.long
    }

    fn description(&self) -> &'static str {
        self.data.common.description
    }

    fn action(&self) -> &FlagAction {
        &self.data.action
    }
}

impl IntoResettable<ArgAction> for &FlagAction {
    fn into_resettable(self) -> Resettable<ArgAction> {
        match self {
            FlagAction::SetTrue => Resettable::Value(ArgAction::SetTrue),
            FlagAction::SetFalse => Resettable::Value(ArgAction::SetFalse),
        }
    }
}

impl From<&FlagClapArg> for Arg {
    fn from(arg: &FlagClapArg) -> Self {
        Self::new(arg.name())
            .short(arg.short())
            .long(arg.long())
            .help(arg.description())
            .action(arg.action())
    }
}

impl IntoFrom<&ArgMatches, bool> for FlagClapArg {
    fn into_from(self, parsing_result: &ArgMatches) -> bool {
        parsing_result.get_flag(self.name())
    }
}
