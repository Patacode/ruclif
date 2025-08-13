use ruclif_core::builder::HasBuilder;
use speculoos::assert_that;

use crate::string::{StringClapArg, StringClapArgBuilder};

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
