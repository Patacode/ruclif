use crate::ClapArgData;

mod impls;

#[derive(Default, Debug)]
pub struct StringClapArgData {
    common: ClapArgData, // 56 bytes
    default_value: Option<&'static str>, // 16 bytes
    value_parser: Option<fn(&str) -> Result<String, String>>, // 8 bytes
} // 80 bytes

#[derive(Default, Debug)]
pub struct StringClapArg {
    data: StringClapArgData // 80 bytes
} // 80 bytes

#[derive(Default, Debug)]
pub struct StringClapArgBuilder {
    name: Option<&'static str>, // 16 bytes
    short: Option<char>, // 4 bytes
    long: Option<&'static str>, // 16 bytes
    description: Option<&'static str>, // 16 bytes
    default_value: Option<&'static str>, // 16 bytes
    value_parser: Option<fn(&str) -> Result<String, String>>, // 8 bytes
} // 80 bytes
