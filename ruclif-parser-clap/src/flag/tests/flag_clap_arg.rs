use std::ffi::OsString;

use clap::ArgAction;
use clap::Command;
use rstest::*;
use ruclif_core::builder::Builder;
use ruclif_core::builder::HasBuilder;
use ruclif_core::common::FromInto;
use speculoos::assert_that;
use speculoos::prelude::BooleanAssertions;

use crate::flag::tests::director::TestData;
use crate::flag::Action;
use crate::flag::Arg;
use crate::flag::ArgBuilder;

mod builder {
    use super::*;

    mod happy {
        use super::*;

        #[test]
        fn it_should_return_default_flag_clap_arg_builder_instance() {
            let expected = ArgBuilder::default();
            let actual = Arg::builder();

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
                let arg = ArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .action(test_data.action().clone())
                    .build()
                    .unwrap();

                let expected = clap::Arg::new(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .help(test_data.description())
                    .action(test_data.action());
                let actual: clap::Arg = (&arg).into();

                assert_that(&actual.get_id())
                    .named("id")
                    .is_equal_to(expected.get_id());
                assert_that(&actual.get_short())
                    .named("short")
                    .is_equal_to(expected.get_short());
                assert_that(&actual.get_long())
                    .named("long")
                    .is_equal_to(expected.get_long());
                assert_that(&actual.get_help())
                    .named("help")
                    .is_equal_to(expected.get_help());
                assert_that(&matches!(
                    actual.get_action(),
                    ArgAction::SetFalse
                ))
                .named("action")
                .is_true();
            }

            #[rstest]
            fn with_no_action(test_data: TestData) {
                let arg = ArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .build()
                    .unwrap();

                let expected = clap::Arg::new(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .help(test_data.description());
                let actual: clap::Arg = (&arg).into();

                let is_action_set_true =
                    matches!(actual.get_action(), ArgAction::SetTrue);

                assert_that(&actual.get_id())
                    .named("id")
                    .is_equal_to(expected.get_id());
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

mod from_into {
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
                    let arg = ArgBuilder::default()
                        .name(test_data.name())
                        .short(test_data.short())
                        .long(test_data.long())
                        .description(test_data.description())
                        .action(Action::SetTrue)
                        .build()
                        .unwrap();

                    let cli_parser =
                        Command::new("test").no_binary_name(true).arg(&arg);
                    let arg_matches =
                        cli_parser.get_matches_from(vec!["--author"]);

                    let actual = arg.from_into(&arg_matches);

                    assert_that(&actual).is_true();
                }

                #[rstest]
                fn when_no_flag_given(test_data: TestData) {
                    let arg = ArgBuilder::default()
                        .name(test_data.name())
                        .short(test_data.short())
                        .long(test_data.long())
                        .description(test_data.description())
                        .action(Action::SetTrue)
                        .build()
                        .unwrap();

                    let cli_parser =
                        Command::new("test").no_binary_name(true).arg(&arg);
                    let arg_matches = cli_parser
                        .get_matches_from::<Vec<OsString>, OsString>(vec![]);

                    let actual = arg.from_into(&arg_matches);

                    assert_that(&actual).is_false();
                }
            }

            mod with_set_false_action {
                use super::*;

                #[rstest]
                fn when_flag_given(test_data: TestData) {
                    let arg = ArgBuilder::default()
                        .name(test_data.name())
                        .short(test_data.short())
                        .long(test_data.long())
                        .description(test_data.description())
                        .action(Action::SetFalse)
                        .build()
                        .unwrap();

                    let cli_parser =
                        Command::new("test").no_binary_name(true).arg(&arg);
                    let arg_matches =
                        cli_parser.get_matches_from(vec!["--author"]);

                    let actual = arg.from_into(&arg_matches);

                    assert_that(&actual).is_false();
                }

                #[rstest]
                fn when_no_flag_given(test_data: TestData) {
                    let arg = ArgBuilder::default()
                        .name(test_data.name())
                        .short(test_data.short())
                        .long(test_data.long())
                        .description(test_data.description())
                        .action(Action::SetFalse)
                        .build()
                        .unwrap();

                    let cli_parser =
                        Command::new("test").no_binary_name(true).arg(&arg);
                    let arg_matches = cli_parser
                        .get_matches_from::<Vec<OsString>, OsString>(vec![]);

                    let actual = arg.from_into(&arg_matches);

                    assert_that(&actual).is_true();
                }
            }
        }
    }
}
