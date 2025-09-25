use clap::builder::IntoResettable;
use clap::builder::Resettable;
use clap::ArgAction;
use clap::ArgMatches;
use ruclif_core::builder::Builder;
use ruclif_core::builder::HasBuilder;
use ruclif_core::common::FromInto;

use crate::flag::arg_builder_state;
use crate::flag::Action;
use crate::flag::Arg;
use crate::flag::ArgBuilder;
use crate::flag::ArgData;
use crate::helper::build_missing_mandatory_fields_error_message;
use crate::NamedArgData;

impl HasBuilder for Arg {
    type Builder = ArgBuilder;

    fn builder() -> Self::Builder {
        ArgBuilder::default()
    }
}

impl ArgBuilder {
    pub fn name(mut self, name: &'static str) -> Self {
        self.common.name = Some(name);
        self.state |= arg_builder_state::NAME as u8;
        self
    }

    pub fn short(mut self, short: char) -> Self {
        self.common.short = Some(short);
        self.state |= arg_builder_state::SHORT as u8;
        self
    }

    pub fn long(mut self, long: &'static str) -> Self {
        self.common.long = Some(long);
        self.state |= arg_builder_state::LONG as u8;
        self
    }

    pub fn description(mut self, description: &'static str) -> Self {
        self.common.description = Some(description);
        self.state |= arg_builder_state::DESCRIPTION as u8;
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }
}

impl ArgBuilder {
    fn is_name_set(&self) -> bool {
        self.state & arg_builder_state::NAME as u8 != 0
    }

    fn is_short_set(&self) -> bool {
        self.state & arg_builder_state::SHORT as u8 != 0
    }

    fn is_long_set(&self) -> bool {
        self.state & arg_builder_state::LONG as u8 != 0
    }

    fn is_description_set(&self) -> bool {
        self.state & arg_builder_state::DESCRIPTION as u8 != 0
    }

    fn build_error_message(&self) -> String {
        build_missing_mandatory_fields_error_message(&[
            ("name", !self.is_name_set()),
            ("short", !self.is_short_set()),
            ("long", !self.is_long_set()),
            ("description", !self.is_description_set()),
        ])
    }
}

impl From<ArgBuilder> for ArgData {
    fn from(value: ArgBuilder) -> Self {
        ArgData {
            common: NamedArgData {
                name: value.common.name.unwrap(),
                short: value.common.short.unwrap(),
                long: value.common.long.unwrap(),
                description: value.common.description.unwrap(),
            },
            action: value.action.unwrap_or(Action::SetTrue),
        }
    }
}

impl Builder for ArgBuilder {
    type Result = Arg;

    fn build(self) -> Result<Arg, String> {
        if self.state != arg_builder_state::EXPECTED {
            Err(self.build_error_message())
        } else {
            Ok(Arg { data: self.into() })
        }
    }
}

impl Arg {
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

    fn action(&self) -> &Action {
        &self.data.action
    }
}

impl IntoResettable<ArgAction> for &Action {
    fn into_resettable(self) -> Resettable<ArgAction> {
        match self {
            Action::SetTrue => Resettable::Value(ArgAction::SetTrue),
            Action::SetFalse => Resettable::Value(ArgAction::SetFalse),
        }
    }
}

impl From<&Arg> for clap::Arg {
    fn from(arg: &Arg) -> Self {
        Self::new(arg.name())
            .short(arg.short())
            .long(arg.long())
            .help(arg.description())
            .action(arg.action())
    }
}

impl FromInto<&ArgMatches, bool> for Arg {
    fn from_into(self, parsing_result: &ArgMatches) -> bool {
        parsing_result.get_flag(self.name())
    }
}
