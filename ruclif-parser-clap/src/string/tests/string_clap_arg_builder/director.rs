use crate::{
    string::{StringClapArgData, ValueParserFunc},
    ClapArgData,
};

pub mod defaults {
    use crate::string::ValueParserFunc;

    pub const NAME: &str = "GENERATOR_URI";
    pub const SHORT: char = 'c';
    pub const LONG: &str = "generator-uri";
    pub const DESCRIPTION: &str = "The template generator uri";
    pub const DEFAULT_VALUE: &str = "/developers/gitignore/api";
    pub const VALUE_PARSER: ValueParserFunc = |value: &str| -> Result<String, String> { Ok(value.to_string()) };
}

pub struct TestData {
    arg_data: StringClapArgData,
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

    pub fn default_value(&self) -> &'static str {
        self.arg_data.default_value.unwrap()
    }

    pub fn value_parser(&self) -> ValueParserFunc {
        self.arg_data.value_parser.unwrap()
    }
}

impl TestData {
    pub fn with_all_fields_set() -> Self {
        Self {
            arg_data: StringClapArgData {
                common: ClapArgData {
                    name: defaults::NAME,
                    short: defaults::SHORT,
                    long: defaults::LONG,
                    description: defaults::DESCRIPTION,
                },
                default_value: Some(defaults::DEFAULT_VALUE),
                value_parser: Some(defaults::VALUE_PARSER),
            },
        }
    }
}
