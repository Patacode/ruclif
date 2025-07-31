use crate::ClapArgData;

#[derive(Default, Debug)]
pub struct StringClapArgData<'a> {
    common: Option<ClapArgData<'a>>, // 72 bytes
    default_value: Option<&'a str>, // 16 bytes
    value_parser: Option<fn(&str) -> Result<&str, &str>>, // 8 bytes
} // 96 bytes
