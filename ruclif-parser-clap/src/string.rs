use crate::ClapArgData;

mod impls;

#[derive(Default, Debug)]
pub struct StringClapArgData<'a> {
    common: ClapArgData<'a>, // 64 bytes
    default_value: Option<&'a str>, // 16 bytes
    value_parser: Option<fn(&str) -> Result<&str, &str>>, // 8 bytes
} // 88 bytes

pub struct StringClapArg<'a> {
    data: StringClapArgData<'a>
} // 88 bytes

pub struct StringClapArgBuilder<'a> {
    name: Option<&'a str>, // 16 bytes
    short: Option<&'a str>, // 16 bytes
    long: Option<&'a str>, // 16 bytes
    description: Option<&'a str>, // 16 bytes
    default_value: Option<&'a str>, // 16 bytes
    value_parser: Option<fn(&str) -> Result<&str, &str>>, // 8 bytes
} // 88 bytes
