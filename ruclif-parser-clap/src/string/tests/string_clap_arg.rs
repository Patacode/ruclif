mod builder {
    use rstest::*;
    use ruclif_core::builder::{Builder, HasBuilder};
    use speculoos::{assert_that, result::ContainingResultAssertions};

    use crate::{
        string::{StringClapArg, StringClapArgBuilder, StringClapArgData},
        ClapArgData,
    };

    #[test]
    fn test_that_it_should_return_correct_builder_instance() {
        let expected = StringClapArgBuilder::default();
        let actual = StringClapArg::builder();

        assert_that(&actual).is_equal_to(expected);
    }

    mod director {
        use super::*;

        mod defaults {
            pub const NAME: &str = "GENERATOR_URI";
            pub const SHORT: char = 'c';
            pub const LONG: &str = "generator-uri";
            pub const DESCRIPTION: &str = "The template generator uri";
            pub const DEFAULT_VALUE: &str = "/developers/gitignore/api";
            pub const VALUE_PARSER: fn(&str) -> Result<String, String> =
                |_value: &str| -> Result<String, String> { Ok(String::from("hello")) };
        }

        pub fn without_name() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .short(defaults::SHORT)
                .long(defaults::LONG)
                .description(defaults::DESCRIPTION)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_short() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .name(defaults::NAME)
                .long(defaults::LONG)
                .description(defaults::DESCRIPTION)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_long() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .name(defaults::NAME)
                .short(defaults::SHORT)
                .description(defaults::DESCRIPTION)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_description() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .name(defaults::NAME)
                .short(defaults::SHORT)
                .long(defaults::LONG)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_name_short() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .long(defaults::LONG)
                .description(defaults::DESCRIPTION)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_name_long() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .short(defaults::SHORT)
                .description(defaults::DESCRIPTION)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_name_description() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .long(defaults::LONG)
                .short(defaults::SHORT)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_short_long() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .name(defaults::NAME)
                .description(defaults::DESCRIPTION)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_short_description() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .name(defaults::NAME)
                .long(defaults::LONG)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_long_description() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .name(defaults::NAME)
                .short(defaults::SHORT)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_name_short_long() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .description(defaults::DESCRIPTION)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_name_short_description() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .long(defaults::LONG)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_name_long_description() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .short(defaults::SHORT)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_short_long_description() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .name(defaults::NAME)
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }

        pub fn without_any_mandatory_fields() -> Result<StringClapArg, String> {
            StringClapArg::builder()
                .default_value(defaults::DEFAULT_VALUE)
                .value_parser(defaults::VALUE_PARSER)
                .build()
        }
    }

    mod happy {
        use super::*;

        #[test]
        fn test_that_it_should_build_string_clap_arg_with_all_fields_set() {
            let value_parser = |_value: &str| -> Result<String, String> { Ok(String::from("hello")) };

            let expected = StringClapArg {
                data: StringClapArgData {
                    common: ClapArgData {
                        name: "GENERATOR_URI",
                        short: 'c',
                        long: "generator-uri",
                        description: "The template generator uri",
                    },
                    default_value: Some("/developers/gitignore/api"),
                    value_parser: Some(value_parser),
                },
            };
            let actual = StringClapArg::builder()
                .name("GENERATOR_URI")
                .short('c')
                .long("generator-uri")
                .description("The template generator uri")
                .default_value("/developers/gitignore/api")
                .value_parser(value_parser)
                .build();

            assert_that(&actual).is_ok_containing(expected);
        }

        #[test]
        fn test_that_it_should_build_string_clap_arg_with_optional_default_value_field_unset() {
            let value_parser = |_value: &str| -> Result<String, String> { Ok(String::from("hello")) };

            let expected = StringClapArg {
                data: StringClapArgData {
                    common: ClapArgData {
                        name: "GENERATOR_URI",
                        short: 'c',
                        long: "generator-uri",
                        description: "The template generator uri",
                    },
                    default_value: None,
                    value_parser: Some(value_parser),
                },
            };
            let actual = StringClapArg::builder()
                .name("GENERATOR_URI")
                .short('c')
                .long("generator-uri")
                .description("The template generator uri")
                .value_parser(value_parser)
                .build();

            assert_that(&actual).is_ok_containing(expected);
        }

        #[test]
        fn test_that_it_should_build_string_clap_arg_with_optional_value_parser_field_unset() {
            let expected = StringClapArg {
                data: StringClapArgData {
                    common: ClapArgData {
                        name: "GENERATOR_URI",
                        short: 'c',
                        long: "generator-uri",
                        description: "The template generator uri",
                    },
                    default_value: Some("/developers/gitignore/api"),
                    value_parser: None,
                },
            };
            let actual = StringClapArg::builder()
                .name("GENERATOR_URI")
                .short('c')
                .long("generator-uri")
                .description("The template generator uri")
                .default_value("/developers/gitignore/api")
                .build();

            assert_that(&actual).is_ok_containing(expected);
        }

        #[test]
        fn test_that_it_should_build_string_clap_arg_with_all_optional_fields_unset() {
            let expected = StringClapArg {
                data: StringClapArgData {
                    common: ClapArgData {
                        name: "GENERATOR_URI",
                        short: 'c',
                        long: "generator-uri",
                        description: "The template generator uri",
                    },
                    default_value: None,
                    value_parser: None,
                },
            };
            let actual = StringClapArg::builder()
                .name("GENERATOR_URI")
                .short('c')
                .long("generator-uri")
                .description("The template generator uri")
                .build();

            assert_that(&actual).is_ok_containing(expected);
        }
    }

    mod unhappy {
        use super::*;

        #[fixture]
        fn test_cases_missing_mandatory_fields_data(
        ) -> Vec<(&'static str, fn() -> Result<StringClapArg, String>, &'static str)> {
            vec![
                ("without_name", || director::without_name(), "name"),
                ("without_short", || director::without_short(), "short"),
                ("without_long", || director::without_long(), "long"),
                ("without_description", || director::without_description(), "description"),
                ("without_name_short", || director::without_name_short(), "name, short"),
                ("without_name_long", || director::without_name_long(), "name, long"),
                (
                    "without_name_description",
                    || director::without_name_description(),
                    "name, description",
                ),
                ("without_short_long", || director::without_short_long(), "short, long"),
                (
                    "without_short_description",
                    || director::without_short_description(),
                    "short, description",
                ),
                (
                    "without_long_description",
                    || director::without_long_description(),
                    "long, description",
                ),
                (
                    "without_name_short_long",
                    || director::without_name_short_long(),
                    "name, short, long",
                ),
                (
                    "without_name_short_description",
                    || director::without_name_short_description(),
                    "name, short, description",
                ),
                (
                    "without_name_long_description",
                    || director::without_name_long_description(),
                    "name, long, description",
                ),
                (
                    "without_short_long_description",
                    || director::without_short_long_description(),
                    "short, long, description",
                ),
                (
                    "without_any_mandatory_fields",
                    || director::without_any_mandatory_fields(),
                    "name, short, long, description",
                ),
            ]
        }

        #[rstest]
        fn test_that_it_should_fail_building_string_clap_arg_when_one_or_more_mandatory_fields_missing(
            test_cases_missing_mandatory_fields_data: Vec<(&str, fn() -> Result<StringClapArg, String>, &str)>,
        ) {
            for test_case_data in test_cases_missing_mandatory_fields_data {
                println!("Running test case '{}'", test_case_data.0);

                let expected = format!("Following mandatory fields are missing: {}", test_case_data.2);
                let actual = test_case_data.1();

                assert_that(&actual).is_err_containing(expected);
            }
        }
    }
}
