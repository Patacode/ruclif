use crate::{string::{StringClapArg, StringClapArgBuilder, StringClapArgData}, ClapArgData};

impl<'a> StringClapArg<'a> {
    pub fn builder() -> StringClapArgBuilder<'a> {
        StringClapArgBuilder::default()
    }
}

impl<'a> StringClapArgBuilder<'a> {
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn short(mut self, short: &'a str) -> Self {
        self.short = Some(short);
        self
    }

    pub fn long(mut self, long: &'a str) -> Self {
        self.long = Some(long);
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn default_value(mut self, default_value: &'a str) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn value_parser(mut self, value_parser: fn(&str) -> Result<&str, &str>) -> Self {
        self.value_parser = Some(value_parser);
        self
    }

    pub fn build(self) -> StringClapArg<'a> {
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
