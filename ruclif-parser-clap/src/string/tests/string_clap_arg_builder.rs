use rstest::*;
use ruclif_core::builder::Builder;
use speculoos::{assert_that, result::ContainingResultAssertions};

use crate::{
    string::{
        tests::string_clap_arg_builder::director::{defaults, TestData},
        StringClapArg, StringClapArgBuilder, StringClapArgData,
    },
    ClapArgData,
};

mod director;

mod build {
    use super::*;

    #[fixture]
    pub fn test_data() -> TestData {
        TestData::with_all_fields_set()
    }

    mod happy {
        use super::*;

        mod it_returns_string_clap_arg {
            use super::*;

            #[rstest]
            fn when_all_fields_set(test_data: TestData) {
                let expected = StringClapArg {
                    data: StringClapArgData {
                        common: ClapArgData {
                            name: defaults::NAME,
                            short: defaults::SHORT,
                            long: defaults::LONG,
                            description: defaults::DESCRIPTION,
                        },
                        default_value: Some(defaults::DEFAULT_VALUE),
                        value_parser: Some(defaults::VALUE_PARSER),
                    },
                };
                let actual = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_ok_containing(expected);
            }

            #[rstest]
            fn when_no_default_value(test_data: TestData) {
                let expected = StringClapArg {
                    data: StringClapArgData {
                        common: ClapArgData {
                            name: defaults::NAME,
                            short: defaults::SHORT,
                            long: defaults::LONG,
                            description: defaults::DESCRIPTION,
                        },
                        default_value: None,
                        value_parser: Some(defaults::VALUE_PARSER),
                    },
                };
                let actual = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_ok_containing(expected);
            }

            #[rstest]
            fn when_no_value_parser(test_data: TestData) {
                let expected = StringClapArg {
                    data: StringClapArgData {
                        common: ClapArgData {
                            name: defaults::NAME,
                            short: defaults::SHORT,
                            long: defaults::LONG,
                            description: defaults::DESCRIPTION,
                        },
                        default_value: Some(defaults::DEFAULT_VALUE),
                        value_parser: None,
                    },
                };
                let actual = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .build();

                assert_that(&actual).is_ok_containing(expected);
            }

            #[rstest]
            fn when_no_default_value_neither_value_parser(test_data: TestData) {
                let expected = StringClapArg {
                    data: StringClapArgData {
                        common: ClapArgData {
                            name: defaults::NAME,
                            short: defaults::SHORT,
                            long: defaults::LONG,
                            description: defaults::DESCRIPTION,
                        },
                        default_value: None,
                        value_parser: None,
                    },
                };
                let actual = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .build();

                assert_that(&actual).is_ok_containing(expected);
            }
        }
    }

    mod unhappy {
        use super::*;

        mod it_returns_error_message {
            use super::*;

            #[rstest]
            fn when_no_name(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name");
                let actual = StringClapArgBuilder::default()
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_short(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: short");
                let actual = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .long(test_data.long())
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_long(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: long");
                let actual = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: description");
                let actual = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_short(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, short");
                let actual = StringClapArgBuilder::default()
                    .long(test_data.long())
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_long(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, long");
                let actual = StringClapArgBuilder::default()
                    .short(test_data.short())
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, description");
                let actual = StringClapArgBuilder::default()
                    .short(test_data.short())
                    .long(test_data.long())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_short_neither_long(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: short, long");
                let actual = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_short_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: short, description");
                let actual = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .long(test_data.long())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_long_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: long, description");
                let actual = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_short_neither_long(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, short, long");
                let actual = StringClapArgBuilder::default()
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_short_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, short, description");
                let actual = StringClapArgBuilder::default()
                    .long(test_data.long())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_long_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, long, description");
                let actual = StringClapArgBuilder::default()
                    .short(test_data.short())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_short_neither_long_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: short, long, description");
                let actual = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_all_mandatory_fields_unset(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, short, long, description");
                let actual = StringClapArgBuilder::default()
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }
        }
    }
}
