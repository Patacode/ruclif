use clap::Command;
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
        fn it_should_return_correct_builder_instance() {
            let expected = StringClapArgBuilder::default();
            let actual = StringClapArg::builder();

            assert_that(&actual).is_equal_to(expected);
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

        #[rstest]
        fn it_should_convert_into_string_from_arg_matches(test_data: TestData) {
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
