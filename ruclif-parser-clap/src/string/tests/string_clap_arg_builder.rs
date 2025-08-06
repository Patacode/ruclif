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

            pub mod error_message {
                pub const MANDATORY_FIELDS_MISSING: &str = "Following mandatory fields are missing: {fields}";
            }

            pub fn with_all() -> Result<StringClapArg, String> {
                StringClapArg::builder()
                    .name(defaults::NAME)
                    .short(defaults::SHORT)
                    .long(defaults::LONG)
                    .description(defaults::DESCRIPTION)
                    .default_value(defaults::DEFAULT_VALUE)
                    .value_parser(defaults::VALUE_PARSER)
                    .build()
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

            mod valid_built_instance {
                use super::*;

                struct TestCaseData {
                    actual: fn() -> Result<StringClapArg, String>,
                    expected: StringClapArg,
                }

                #[fixture]
                fn string_clap_arg_with_all_fields_set() -> TestCaseData {
                    TestCaseData {
                        actual: || director::with_all(),
                        expected: StringClapArg {
                            data: StringClapArgData {
                                common: ClapArgData {
                                    name: director::defaults::NAME,
                                    short: director::defaults::SHORT,
                                    long: director::defaults::LONG,
                                    description: director::defaults::DESCRIPTION,
                                },
                                default_value: Some(director::defaults::DEFAULT_VALUE),
                                value_parser: Some(director::defaults::VALUE_PARSER),
                            },
                        },
                    }
                }

                #[fixture]
                fn string_clap_arg_without_default_value() -> TestCaseData {
                    TestCaseData {
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

                #[fixture]
                fn string_clap_arg_without_value_parser() -> TestCaseData {
                    TestCaseData {
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

                #[fixture]
                fn string_clap_arg_without_value_parser_and_default_value() -> TestCaseData {
                    TestCaseData {
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

                #[rstest]
                fn with_all_fields_set(string_clap_arg_with_all_fields_set: TestCaseData) {
                    let expected = string_clap_arg_with_all_fields_set.expected;
                    let actual = (string_clap_arg_with_all_fields_set.actual)();

                    assert_that(&actual).is_ok_containing(expected);
                }

                #[rstest]
                fn without_default_value(string_clap_arg_without_default_value: TestCaseData) {
                    let expected = string_clap_arg_without_default_value.expected;
                    let actual = (string_clap_arg_without_default_value.actual)();

                    assert_that(&actual).is_ok_containing(expected);
                }

                #[rstest]
                fn without_value_parser(string_clap_arg_without_value_parser: TestCaseData) {
                    let expected = string_clap_arg_without_value_parser.expected;
                    let actual = (string_clap_arg_without_value_parser.actual)();

                    assert_that(&actual).is_ok_containing(expected);
                }

                #[rstest]
                fn without_value_parser_default_value(
                    string_clap_arg_without_value_parser_and_default_value: TestCaseData,
                ) {
                    let expected = string_clap_arg_without_value_parser_and_default_value.expected;
                    let actual = (string_clap_arg_without_value_parser_and_default_value.actual)();

                    assert_that(&actual).is_ok_containing(expected);
                }
            }
        }

        mod unhappy {
            use super::*;

            mod invalid_built_instance {
                use super::*;

                struct TestCaseData {
                    actual: fn() -> Result<StringClapArg, String>,
                    expected: &'static str,
                }

                impl TestCaseData {
                    fn without_name() -> Self {
                        Self {
                            actual: || director::without_name(),
                            expected: "name",
                        }
                    }

                    fn without_short() -> Self {
                        Self {
                            actual: || director::without_short(),
                            expected: "short",
                        }
                    }

                    fn without_long() -> Self {
                        Self {
                            actual: || director::without_long(),
                            expected: "long",
                        }
                    }

                    fn without_description() -> Self {
                        Self {
                            actual: || director::without_description(),
                            expected: "description",
                        }
                    }

                    fn without_name_short() -> Self {
                        Self {
                            actual: || director::without_name_short(),
                            expected: "name, short",
                        }
                    }

                    fn without_name_long() -> Self {
                        Self {
                            actual: || director::without_name_long(),
                            expected: "name, long",
                        }
                    }

                    fn without_name_description() -> Self {
                        Self {
                            actual: || director::without_name_description(),
                            expected: "name, description",
                        }
                    }

                    fn without_short_long() -> Self {
                        Self {
                            actual: || director::without_short_long(),
                            expected: "short, long",
                        }
                    }

                    fn without_short_description() -> Self {
                        Self {
                            actual: || director::without_short_description(),
                            expected: "short, description",
                        }
                    }

                    fn without_long_description() -> Self {
                        Self {
                            actual: || director::without_long_description(),
                            expected: "long, description",
                        }
                    }

                    fn without_name_short_long() -> Self {
                        Self {
                            actual: || director::without_name_short_long(),
                            expected: "name, short, long",
                        }
                    }

                    fn without_name_short_description() -> Self {
                        Self {
                            actual: || director::without_name_short_description(),
                            expected: "name, short, description",
                        }
                    }

                    fn without_name_long_description() -> Self {
                        Self {
                            actual: || director::without_name_long_description(),
                            expected: "name, long, description",
                        }
                    }

                    fn without_short_long_description() -> Self {
                        Self {
                            actual: || director::without_short_long_description(),
                            expected: "short, long, description",
                        }
                    }

                    fn without_any_mandatory_fields() -> Self {
                        Self {
                            actual: || director::without_any_mandatory_fields(),
                            expected: "name, short, long, description",
                        }
                    }
                }

                mod missing_mandatory_fields {
                    use super::*;
                    use crate::string::tests::string_clap_arg_builder::builder::build::director::error_message;

                    #[test]
                    fn without_name() {
                        let test_data = TestCaseData::without_name();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_short() {
                        let test_data = TestCaseData::without_short();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_long() {
                        let test_data = TestCaseData::without_long();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_description() {
                        let test_data = TestCaseData::without_description();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_name_short() {
                        let test_data = TestCaseData::without_name_short();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_name_long() {
                        let test_data = TestCaseData::without_name_long();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_name_description() {
                        let test_data = TestCaseData::without_name_description();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_short_long() {
                        let test_data = TestCaseData::without_short_long();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_short_description() {
                        let test_data = TestCaseData::without_short_description();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_long_description() {
                        let test_data = TestCaseData::without_long_description();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_name_short_long() {
                        let test_data = TestCaseData::without_name_short_long();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_name_short_description() {
                        let test_data = TestCaseData::without_name_short_description();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_name_long_description() {
                        let test_data = TestCaseData::without_name_long_description();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_short_long_description() {
                        let test_data = TestCaseData::without_short_long_description();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }

                    #[test]
                    fn without_any_mandatory_fields() {
                        let test_data = TestCaseData::without_any_mandatory_fields();
                        let expected = error_message::MANDATORY_FIELDS_MISSING.replace("{fields}", test_data.expected);
                        let actual = (test_data.actual)();

                        assert_that(&actual).is_err_containing(expected);
                    }
                }
            }
        }
    }
}
