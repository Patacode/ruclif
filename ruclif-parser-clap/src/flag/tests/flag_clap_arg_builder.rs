use rstest::*;
use ruclif_core::builder::Builder;
use speculoos::{assert_that, result::ContainingResultAssertions};

use crate::{
    flag::{
        tests::director::{defaults, TestData},
        FlagAction, FlagClapArg, FlagClapArgBuilder, FlagClapArgData,
    },
    ClapNamedArgData,
};

mod build {
    use super::*;

    #[fixture]
    pub fn test_data() -> TestData {
        TestData::with_all_fields_set()
    }

    mod happy {
        use super::*;

        mod it_returns_flag_clap_arg {
            use super::*;

            #[rstest]
            fn when_all_fields_set(test_data: TestData) {
                let expected = FlagClapArg {
                    data: FlagClapArgData {
                        common: ClapNamedArgData {
                            name: defaults::NAME,
                            short: defaults::SHORT,
                            long: defaults::LONG,
                            description: defaults::DESCRIPTION,
                        },
                        action: FlagAction::SetFalse,
                    },
                };
                let actual = FlagClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_ok_containing(expected);
            }

            #[rstest]
            fn when_no_action(test_data: TestData) {
                let expected = FlagClapArg {
                    data: FlagClapArgData {
                        common: ClapNamedArgData {
                            name: defaults::NAME,
                            short: defaults::SHORT,
                            long: defaults::LONG,
                            description: defaults::DESCRIPTION,
                        },
                        action: FlagAction::SetTrue,
                    },
                };
                let actual = FlagClapArgBuilder::default()
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
                let actual = FlagClapArgBuilder::default()
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_short(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: short");
                let actual = FlagClapArgBuilder::default()
                    .name(test_data.name())
                    .long(test_data.long())
                    .description(test_data.description())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_long(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: long");
                let actual = FlagClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .description(test_data.description())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: description");
                let actual = FlagClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_short(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, short");
                let actual = FlagClapArgBuilder::default()
                    .long(test_data.long())
                    .description(test_data.description())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_long(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, long");
                let actual = FlagClapArgBuilder::default()
                    .short(test_data.short())
                    .description(test_data.description())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, description");
                let actual = FlagClapArgBuilder::default()
                    .short(test_data.short())
                    .long(test_data.long())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_short_neither_long(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: short, long");
                let actual = FlagClapArgBuilder::default()
                    .name(test_data.name())
                    .description(test_data.description())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_short_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: short, description");
                let actual = FlagClapArgBuilder::default()
                    .name(test_data.name())
                    .long(test_data.long())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_long_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: long, description");
                let actual = FlagClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_short_neither_long(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, short, long");
                let actual = FlagClapArgBuilder::default()
                    .description(test_data.description())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_short_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, short, description");
                let actual = FlagClapArgBuilder::default()
                    .long(test_data.long())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_name_neither_long_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, long, description");
                let actual = FlagClapArgBuilder::default()
                    .short(test_data.short())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_no_short_neither_long_neither_description(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: short, long, description");
                let actual = FlagClapArgBuilder::default()
                    .name(test_data.name())
                    .action(test_data.action().clone())
                    .build();

                assert_that(&actual).is_err_containing(expected);
            }

            #[rstest]
            fn when_all_mandatory_fields_unset(test_data: TestData) {
                let expected = String::from("Following mandatory fields are missing: name, short, long, description");
                let actual = FlagClapArgBuilder::default().action(test_data.action().clone()).build();

                assert_that(&actual).is_err_containing(expected);
            }
        }
    }
}
