pub mod string;

#[derive(Default, Debug)]
struct ClapArgData<'a> {
    name: Option<&'a str>, // 16 bytes
    short: Option<&'a str>, // 16 bytes
    long: Option<&'a str>, // 16 bytes
    description: Option<&'a str>, // 16 bytes
} // 64 bytes
