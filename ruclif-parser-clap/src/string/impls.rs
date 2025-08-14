use clap::{Arg, ArgMatches};
use ruclif_core::{
    builder::{Builder, HasBuilder},
    common::IntoFrom,
};

use crate::{
    string::{builder_state, error_message, StringClapArg, StringClapArgBuilder, StringClapArgData, ValueParserFunc},
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
        self.state |= builder_state::DESCRIPTION as u8;
        self
    }

    pub fn default_value(mut self, default_value: &'static str) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn value_parser(mut self, value_parser: ValueParserFunc) -> Self {
        self.value_parser = Some(value_parser);
        self
    }
}

impl StringClapArgBuilder {
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

impl Builder for StringClapArgBuilder {
    type Result = StringClapArg;

    fn build(self) -> Result<StringClapArg, String> {
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

    fn default_value(&self) -> Option<&'static str> {
        self.data.default_value
    }

    fn value_parser(&self) -> Option<fn(&str) -> Result<String, String>> {
        self.data.value_parser
    }
}

impl From<&StringClapArg> for Arg {
    fn from(arg: &StringClapArg) -> Self {
        let mut result = Self::new(arg.name())
            .short(arg.short())
            .long(arg.long())
            .help(arg.description());

        if let Some(default_value) = arg.default_value() {
            result = result.default_value(default_value);
        }
        if let Some(value_parser) = arg.value_parser() {
            result = result.value_parser(value_parser);
        }

        result
    }
}

impl IntoFrom<&ArgMatches, String> for StringClapArg {
    fn into_from(self, parsing_result: &ArgMatches) -> String {
        parsing_result.get_one::<String>(self.name()).unwrap().to_owned()
    }
}
