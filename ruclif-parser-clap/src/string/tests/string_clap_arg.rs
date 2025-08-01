mod builder {
    use speculoos::assert_that;

    use crate::string::{StringClapArg, StringClapArgBuilder};

    #[test]
    fn test_that_it_should_return_correct_builder_instance() {
        let expected = StringClapArgBuilder::default();
        let actual = StringClapArg::builder();

        assert_that(&actual).is_equal_to(expected);
    }
}
