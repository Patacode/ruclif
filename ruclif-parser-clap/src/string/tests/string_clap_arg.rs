mod builder {
    use ruclif_core::builder::{Builder, HasBuilder};
    use speculoos::assert_that;

    use crate::{
        string::{StringClapArg, StringClapArgBuilder, StringClapArgData},
        ClapArgData,
    };

    #[test]
    fn test_that_it_should_return_correct_builder_instance() {
        let expected = StringClapArgBuilder::default();
        let actual = StringClapArg::builder();

        assert_that(&actual).is_equal_to(expected);
    }

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

        assert_that(&actual).is_equal_to(expected);
    }
}
