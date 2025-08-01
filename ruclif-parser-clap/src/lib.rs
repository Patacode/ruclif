pub mod string;

#[derive(Default, Debug)]
struct ClapArgData {
    name: Option<&'static str>, // 16 bytes
    short: Option<char>, // 16 bytes
    long: Option<&'static str>, // 16 bytes
    description: Option<&'static str>, // 16 bytes
} // 64 bytes
