#[derive(Default, Debug)]
struct ClapArgData<'a> {
    name: Option<&'a str>, // 16 bytes (pointer + len)
    short: Option<&'a str>, // 16 bytes (pointer + len)
    long: Option<&'a str>, // 16 bytes (pointer + len)
    description: Option<&'a str>, // 16 bytes (pointer + len)
} // 64 bytes
