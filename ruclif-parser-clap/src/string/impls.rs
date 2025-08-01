use clap::{Arg, ArgMatches};
use ruclif_core::{common::HasBuilder, parser::CliArg};

use crate::{string::{StringClapArg, StringClapArgBuilder, StringClapArgData}, ClapArgData};

impl HasBuilder for StringClapArg {
    type Builder = StringClapArgBuilder;

    fn builder() -> Self::Builder {
        StringClapArgBuilder::default()
    }
}

impl StringClapArgBuilder {
    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    pub fn long(mut self, long: &'static str) -> Self {
        self.long = Some(long);
        self
    }

    pub fn description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
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

    pub fn build(self) -> StringClapArg {
        let data = StringClapArgData {
            common: ClapArgData {
                name: self.name,
                short: self.short,
                long: self.long,
                description: self.description,
            },
            default_value: self.default_value,
            value_parser: self.value_parser,
        };

        StringClapArg { data }
    }
}

impl StringClapArg {
    fn name(&self) -> &'static str {
        self.data.common.name.unwrap()
    }

    fn short(&self) -> char {
       self.data.common.short.unwrap()
    }

    fn long(&self) -> &'static str {
        self.data.common.long.unwrap()
    }

    fn description(&self) -> &'static str {
        self.data.common.description.unwrap()
    }

    fn default_value(&self) -> &'static str {
        self.data.default_value.unwrap()
    }

    fn value_parser(&self) -> fn(&str) -> Result<String, String> {
        self.data.value_parser.unwrap()
    }
}

impl CliArg<Arg, ArgMatches, String> for StringClapArg {
    fn build(&self) -> Arg {
        Arg::new(self.name())
            .short(self.short())
            .long(self.long())
            .help(self.description())
            .default_value(self.default_value())
            .value_parser(self.value_parser())
    }

    fn convert(&self, parsing_result: &ArgMatches) -> String {
        parsing_result
            .get_one::<String>(self.name())
            .unwrap()
            .to_owned()
    }
}
