use clap::{Arg, ArgMatches};
use ruclif_core::{
    builder::{Builder, HasBuilder},
    common::IntoFrom,
};

use crate::{
    string::{builder_state, error_message, StringClapArg, StringClapArgBuilder, StringClapArgData},
    ClapArgData,
};

impl HasBuilder for StringClapArg {
    type Builder = StringClapArgBuilder;

    fn builder() -> Self::Builder {
        StringClapArgBuilder::default()
    }
}

impl StringClapArgBuilder {
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
        self.state = self.state | builder_state::DESCRIPTION as u8;
        self
    }

    pub fn default_value(mut self, default_value: &'static str) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn value_parser(mut self, value_parser: fn(&str) -> Result<String, String>) -> Self {
        self.value_parser = Some(value_parser);
        self
    }
}

impl Builder for StringClapArgBuilder {
    type Result = StringClapArg;

    fn build(self) -> Result<StringClapArg, String> {
        if self.state != builder_state::EXPECTED {
            let missing_states = self.state & builder_state::EXPECTED;
            let mut missing_fields: Vec<&str> = Vec::new();

            if missing_states & builder_state::NAME as u8 == 0 {
                missing_fields.push("name");
            }

            if missing_states & builder_state::SHORT as u8 == 0 {
                missing_fields.push("short");
            }

            if missing_states & builder_state::LONG as u8 == 0 {
                missing_fields.push("long");
            }

            if missing_states & builder_state::DESCRIPTION as u8 == 0 {
                missing_fields.push("description");
            }

            Err(error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", &missing_fields.join(", ")))
        } else {
            let data = StringClapArgData {
                common: ClapArgData {
                    name: self.name.unwrap(),
                    short: self.short.unwrap(),
                    long: self.long.unwrap(),
                    description: self.description.unwrap(),
                },
                default_value: self.default_value,
                value_parser: self.value_parser,
            };

            Ok(StringClapArg { data })
        }
    }
}

impl StringClapArg {
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

    fn default_value(&self) -> &'static str {
        self.data.default_value.unwrap()
    }

    fn value_parser(&self) -> fn(&str) -> Result<String, String> {
        self.data.value_parser.unwrap()
    }
}

impl From<&StringClapArg> for Arg {
    fn from(arg: &StringClapArg) -> Self {
        Self::new(arg.name())
            .short(arg.short())
            .long(arg.long())
            .help(arg.description())
            .default_value(arg.default_value())
            .value_parser(arg.value_parser())
    }
}

impl IntoFrom<&ArgMatches, String> for StringClapArg {
    fn into_from(self, parsing_result: &ArgMatches) -> String {
        parsing_result.get_one::<String>(self.name()).unwrap().to_owned()
    }
}
