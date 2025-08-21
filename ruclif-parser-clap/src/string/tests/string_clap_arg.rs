use clap::{Arg, Command};
use rstest::*;
use ruclif_core::{
    builder::{Builder, HasBuilder},
    common::IntoFrom,
};
use speculoos::assert_that;

use crate::string::{tests::director::TestData, StringClapArg, StringClapArgBuilder};

mod builder {
    use super::*;

    mod happy {
        use super::*;

        #[test]
        fn it_should_return_default_string_clap_arg_builder_instance() {
            let expected = StringClapArgBuilder::default();
            let actual = StringClapArg::builder();

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
                let arg = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build()
                    .unwrap();

                let expected = Arg::new(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .help(test_data.description())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser());
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
                assert_that(&actual.get_default_values())
                    .named("default value")
                    .is_equal_to(expected.get_default_values());
                assert_that(&format!("{:?}", &actual.get_value_parser()))
                    .named("value parser")
                    .is_equal_to(format!("{:?}", &expected.get_value_parser()));
            }

            #[rstest]
            fn with_no_default_value(test_data: TestData) {
                let arg = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .value_parser(test_data.value_parser())
                    .build()
                    .unwrap();

                let expected = Arg::new(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .help(test_data.description())
                    .value_parser(test_data.value_parser());
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
                assert_that(&actual.get_default_values())
                    .named("default value")
                    .is_equal_to(expected.get_default_values());
                assert_that(&format!("{:?}", &actual.get_value_parser()))
                    .named("value parser")
                    .is_equal_to(format!("{:?}", &expected.get_value_parser()));
            }

            #[rstest]
            fn with_no_value_parser(test_data: TestData) {
                let arg = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .build()
                    .unwrap();

                let expected = Arg::new(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .help(test_data.description())
                    .default_value(test_data.default_value());
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
                assert_that(&actual.get_default_values())
                    .named("default value")
                    .is_equal_to(expected.get_default_values());
                assert_that(&format!("{:?}", &actual.get_value_parser()))
                    .named("value parser")
                    .is_equal_to(format!("{:?}", &expected.get_value_parser()));
            }

            #[rstest]
            fn with_no_default_value_neither_value_parser(test_data: TestData) {
                let arg = StringClapArgBuilder::default()
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
                assert_that(&actual.get_default_values())
                    .named("default value")
                    .is_equal_to(expected.get_default_values());
                assert_that(&format!("{:?}", &actual.get_value_parser()))
                    .named("value parser")
                    .is_equal_to(format!("{:?}", &expected.get_value_parser()));
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

        mod it_convert_into_string_from_arg_matches {
            use super::*;

            #[rstest]
            fn when_arg_value_given(test_data: TestData) {
                let arg = StringClapArgBuilder::default()
                    .name(test_data.name())
                    .short(test_data.short())
                    .long(test_data.long())
                    .description(test_data.description())
                    .default_value(test_data.default_value())
                    .value_parser(test_data.value_parser())
                    .build()
                    .unwrap();

                let cli_parser = Command::new("test").no_binary_name(true).arg(&arg);
                let arg_matches = cli_parser.get_matches_from(vec!["-c", "hello"]);

                let expected = String::from("hello");
                let actual = arg.into_from(&arg_matches);

                assert_that(&actual).is_equal_to(expected);
            }
        }
    }
}
