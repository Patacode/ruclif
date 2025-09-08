use std::ffi::OsString;

use clap::{Arg, ArgAction, Command};
use rstest::*;
use ruclif_core::{
    builder::{Builder, HasBuilder},
    common::IntoFrom,
};
use speculoos::{assert_that, prelude::BooleanAssertions};

use crate::flag::{tests::director::TestData, FlagAction, FlagClapArg, FlagClapArgBuilder};

mod builder {
    use super::*;

    mod happy {
        use super::*;

        #[test]
        fn it_should_return_default_flag_clap_arg_builder_instance() {
            let expected = FlagClapArgBuilder::default();
            let actual = FlagClapArg::builder();

            assert_that(&actual).is_equal_to(expected);
        }
    }
}

mod into {
    use super::*;

    #[fixture]
    pub fn test_data() -> TestData {
        TestData::with_all_fields_set()
    }

    mod happy {
        use super::*;

        mod it_convert_into_arg {
            use super::*;

            #[rstest]
            fn with_all_fields_set(test_data: TestData) {
                let arg = FlagClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .action(test_data.action().clone())
                    .build()
                    .unwrap();

                let expected = Arg::new(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .help(test_data.description())
                    .action(test_data.action());
                let actual: Arg = (&arg).into();

                assert_that(&actual.get_id()).named("id").is_equal_to(expected.get_id());
                assert_that(&actual.get_short())
                    .named("short")
                    .is_equal_to(expected.get_short());
                assert_that(&actual.get_long())
                    .named("long")
                    .is_equal_to(expected.get_long());
                assert_that(&actual.get_help())
                    .named("help")
                    .is_equal_to(expected.get_help());
                assert_that(&matches!(actual.get_action(), ArgAction::SetFalse)).named("action").is_true();
            }

            #[rstest]
            fn with_no_action(test_data: TestData) {
                let arg = FlagClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .build()
                    .unwrap();

                let expected = Arg::new(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .help(test_data.description());
                let actual: Arg = (&arg).into();

                let is_action_set_true = matches!(actual.get_action(), ArgAction::SetTrue);

                assert_that(&actual.get_id()).named("id").is_equal_to(expected.get_id());
                assert_that(&actual.get_short())
                    .named("short")
                    .is_equal_to(expected.get_short());
                assert_that(&actual.get_long())
                    .named("long")
                    .is_equal_to(expected.get_long());
                assert_that(&actual.get_help())
                    .named("help")
                    .is_equal_to(expected.get_help());
                assert_that(&is_action_set_true).named("action").is_true();
            }
        }
    }
}

mod into_from {
    use super::*;

    #[fixture]
    pub fn test_data() -> TestData {
        TestData::with_all_fields_set()
    }

    mod happy {
        use super::*;

        mod it_convert_into_bool_from_arg_matches {
            use super::*;

            mod with_set_true_action {
                use super::*;

                #[rstest]
                fn when_flag_given(test_data: TestData) {
                    let arg = FlagClapArgBuilder::default()
                        .name(test_data.name())
                        .short(test_data.short())
                        .long(test_data.long())
                        .description(test_data.description())
                        .action(FlagAction::SetTrue)
                        .build()
                        .unwrap();

                    let cli_parser = Command::new("test").no_binary_name(true).arg(&arg);
                    let arg_matches = cli_parser.get_matches_from(vec!["--author"]);

                    let actual = arg.into_from(&arg_matches);

                    assert_that(&actual).is_true();
                }

                #[rstest]
                fn when_no_flag_given(test_data: TestData) {
                    let arg = FlagClapArgBuilder::default()
                        .name(test_data.name())
                        .short(test_data.short())
                        .long(test_data.long())
                        .description(test_data.description())
                        .action(FlagAction::SetTrue)
                        .build()
                        .unwrap();

                    let cli_parser = Command::new("test").no_binary_name(true).arg(&arg);
                    let arg_matches = cli_parser.get_matches_from::<Vec<OsString>, OsString>(vec![]);

                    let actual = arg.into_from(&arg_matches);

                    assert_that(&actual).is_false();
                }
            }

            mod with_set_false_action {
                use super::*;

                #[rstest]
                fn when_flag_given(test_data: TestData) {
                    let arg = FlagClapArgBuilder::default()
                        .name(test_data.name())
                        .short(test_data.short())
                        .long(test_data.long())
                        .description(test_data.description())
                        .action(FlagAction::SetFalse)
                        .build()
                        .unwrap();

                    let cli_parser = Command::new("test").no_binary_name(true).arg(&arg);
                    let arg_matches = cli_parser.get_matches_from(vec!["--author"]);

                    let actual = arg.into_from(&arg_matches);

                    assert_that(&actual).is_false();
                }

                #[rstest]
                fn when_no_flag_given(test_data: TestData) {
                    let arg = FlagClapArgBuilder::default()
                        .name(test_data.name())
                        .short(test_data.short())
                        .long(test_data.long())
                        .description(test_data.description())
                        .action(FlagAction::SetFalse)
                        .build()
                        .unwrap();

                    let cli_parser = Command::new("test").no_binary_name(true).arg(&arg);
                    let arg_matches = cli_parser.get_matches_from::<Vec<OsString>, OsString>(vec![]);

                    let actual = arg.into_from(&arg_matches);

                    assert_that(&actual).is_true();
                }
            }
        }
    }
}
