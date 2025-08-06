use rstest::*;
use ruclif_core::builder::{Builder, HasBuilder};
use speculoos::{assert_that, result::ContainingResultAssertions};

use crate::{
    string::{StringClapArg, StringClapArgData},
    ClapArgData,
};

mod builder {
    use super::*;

    mod build {
        use super::*;

        mod director {
            use super::*;

            pub mod defaults {
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

            pub fn without_default_value() -> Result<StringClapArg, String> {
                StringClapArg::builder()
                    .name(defaults::NAME)
                    .short(defaults::SHORT)
                    .long(defaults::LONG)
                    .description(defaults::DESCRIPTION)
                    .value_parser(defaults::VALUE_PARSER)
                    .build()
            }

            pub fn without_value_parser() -> Result<StringClapArg, String> {
                StringClapArg::builder()
                    .name(defaults::NAME)
                    .short(defaults::SHORT)
                    .long(defaults::LONG)
                    .description(defaults::DESCRIPTION)
                    .default_value(defaults::DEFAULT_VALUE)
                    .build()
            }

            pub fn without_value_parser_default_value() -> Result<StringClapArg, String> {
                StringClapArg::builder()
                    .name(defaults::NAME)
                    .short(defaults::SHORT)
                    .long(defaults::LONG)
                    .description(defaults::DESCRIPTION)
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

            mod with_optional_fields_missing {
                use super::*;

                struct TestCaseData {
                    name: &'static str,
                    actual: fn() -> Result<StringClapArg, String>,
                    expected: StringClapArg,
                }

                impl TestCaseData {
                    fn without_default_value() -> Self {
                        Self {
                            name: "without_default_value",
                            actual: || director::without_default_value(),
                            expected: StringClapArg {
                                data: StringClapArgData {
                                    common: ClapArgData {
                                        name: director::defaults::NAME,
                                        short: director::defaults::SHORT,
                                        long: director::defaults::LONG,
                                        description: director::defaults::DESCRIPTION,
                                    },
                                    default_value: None,
                                    value_parser: Some(director::defaults::VALUE_PARSER),
                                },
                            },
                        }
                    }

                    fn without_value_parser() -> Self {
                        Self {
                            name: "without_value_parser",
                            actual: || director::without_value_parser(),
                            expected: StringClapArg {
                                data: StringClapArgData {
                                    common: ClapArgData {
                                        name: director::defaults::NAME,
                                        short: director::defaults::SHORT,
                                        long: director::defaults::LONG,
                                        description: director::defaults::DESCRIPTION,
                                    },
                                    default_value: Some(director::defaults::DEFAULT_VALUE),
                                    value_parser: None,
                                },
                            },
                        }
                    }

                    fn without_value_parser_default_value() -> Self {
                        Self {
                            name: "without_value_parser_default_value",
                            actual: || director::without_value_parser_default_value(),
                            expected: StringClapArg {
                                data: StringClapArgData {
                                    common: ClapArgData {
                                        name: director::defaults::NAME,
                                        short: director::defaults::SHORT,
                                        long: director::defaults::LONG,
                                        description: director::defaults::DESCRIPTION,
                                    },
                                    default_value: None,
                                    value_parser: None,
                                },
                            },
                        }
                    }
                }

                #[fixture]
                fn test_cases_data() -> Vec<TestCaseData> {
                    vec![
                        TestCaseData::without_default_value(),
                        TestCaseData::without_value_parser(),
                        TestCaseData::without_value_parser_default_value(),
                    ]
                }

                #[rstest]
                fn it_returns_correct_string_clap_arg_instance(test_cases_data: Vec<TestCaseData>) {
                    for test_case_data in test_cases_data {
                        println!("Running test case '{}'", test_case_data.name);

                        let expected = test_case_data.expected;
                        let actual = (test_case_data.actual)();

                        assert_that(&actual).is_ok_containing(expected);
                    }
                }
            }
        }

        mod unhappy {
            use super::*;

            mod with_mandatory_fields_missing {
                use super::*;

                struct TestCaseData {
                    name: &'static str,
                    actual: fn() -> Result<StringClapArg, String>,
                    expected: &'static str,
                }

                #[fixture]
                fn test_cases_data() -> Vec<TestCaseData> {
                    vec![
                        TestCaseData {
                            name: "without_name",
                            actual: || director::without_name(),
                            expected: "name",
                        },
                        TestCaseData {
                            name: "without_short",
                            actual: || director::without_short(),
                            expected: "short",
                        },
                        TestCaseData {
                            name: "without_long",
                            actual: || director::without_long(),
                            expected: "long",
                        },
                        TestCaseData {
                            name: "without_description",
                            actual: || director::without_description(),
                            expected: "description",
                        },
                        TestCaseData {
                            name: "without_name_short",
                            actual: || director::without_name_short(),
                            expected: "name, short",
                        },
                        TestCaseData {
                            name: "without_name_long",
                            actual: || director::without_name_long(),
                            expected: "name, long",
                        },
                        TestCaseData {
                            name: "without_name_description",
                            actual: || director::without_name_description(),
                            expected: "name, description",
                        },
                        TestCaseData {
                            name: "without_short_long",
                            actual: || director::without_short_long(),
                            expected: "short, long",
                        },
                        TestCaseData {
                            name: "without_short_description",
                            actual: || director::without_short_description(),
                            expected: "short, description",
                        },
                        TestCaseData {
                            name: "without_long_description",
                            actual: || director::without_long_description(),
                            expected: "long, description",
                        },
                        TestCaseData {
                            name: "without_name_short_long",
                            actual: || director::without_name_short_long(),
                            expected: "name, short, long",
                        },
                        TestCaseData {
                            name: "without_name_short_description",
                            actual: || director::without_name_short_description(),
                            expected: "name, short, description",
                        },
                        TestCaseData {
                            name: "without_name_long_description",
                            actual: || director::without_name_long_description(),
                            expected: "name, long, description",
                        },
                        TestCaseData {
                            name: "without_short_long_description",
                            actual: || director::without_short_long_description(),
                            expected: "short, long, description",
                        },
                        TestCaseData {
                            name: "without_any_mandatory_fields",
                            actual: || director::without_any_mandatory_fields(),
                            expected: "name, short, long, description",
                        },
                    ]
                }

                #[rstest]
                fn it_returns_correct_error(test_cases_data: Vec<TestCaseData>) {
                    for test_case_data in test_cases_data {
                        println!("Running test case '{}'", test_case_data.name);

                        let expected = format!("Following mandatory fields are missing: {}", test_case_data.expected);
                        let actual = (test_case_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }
                }
            }
        }
    }
}
