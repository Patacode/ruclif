use clap::{
    builder::{IntoResettable, Resettable},
    Arg, ArgAction, ArgMatches,
};
use ruclif_core::{
    builder::{Builder, HasBuilder},
    common::IntoFrom,
};

use crate::{
    flag::{builder_state, error_message, BoolArgAction, BoolClapArg, BoolClapArgBuilder, BoolClapArgData},
    ClapArgData,
};

impl HasBuilder for BoolClapArgData {
    type Builder = BoolClapArgBuilder;

    fn builder() -> Self::Builder {
        BoolClapArgBuilder::default()
    }
}

impl BoolClapArgBuilder {
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

    pub fn action(mut self, action: BoolArgAction) -> Self {
        self.action = Some(action);
        self
    }
}

impl BoolClapArgBuilder {
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
}

impl Builder for BoolClapArgBuilder {
    type Result = BoolClapArg;

    fn build(self) -> Result<BoolClapArg, String> {
        if self.state != builder_state::EXPECTED {
            let map = vec![
                ("name", !self.is_name_set()),
                ("short", !self.is_short_set()),
                ("long", !self.is_long_set()),
                ("description", !self.is_description_set()),
            ];

            let missing_fields: Vec<&str> = map.iter().filter_map(|entry| entry.1.then_some(entry.0)).collect();

            Err(error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", &missing_fields.join(", ")))
        } else {
            let data = BoolClapArgData {
                common: ClapArgData {
                    name: self.name.unwrap(),
                    short: self.short.unwrap(),
                    long: self.long.unwrap(),
                    description: self.description.unwrap(),
                },
                action: self.action,
            };

            Ok(BoolClapArg { data })
        }
    }
}

impl BoolClapArg {
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

    fn action(&self) -> &Option<BoolArgAction> {
        &self.data.action
    }
}

impl IntoResettable<ArgAction> for &BoolArgAction {
    fn into_resettable(self) -> Resettable<ArgAction> {
        match self {
            BoolArgAction::SetTrue => Resettable::Value(ArgAction::SetTrue),
            BoolArgAction::SetFalse => Resettable::Value(ArgAction::SetFalse),
        }
    }
}

impl From<&BoolClapArg> for Arg {
    fn from(arg: &BoolClapArg) -> Self {
        let mut result = Self::new(arg.name())
            .short(arg.short())
            .long(arg.long())
            .help(arg.description());

        if let Some(action) = arg.action() {
            result = result.action(action);
        } else {
            result  = result.action(&BoolArgAction::SetTrue)
        }

        result
    }
}

impl IntoFrom<&ArgMatches, bool> for BoolClapArg {
    fn into_from(self, parsing_result: &ArgMatches) -> bool {
        parsing_result.get_flag(self.name())
    }
}
