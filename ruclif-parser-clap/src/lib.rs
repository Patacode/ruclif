pub mod string;

#[derive(Default, Debug)]
struct ClapArgData {
    name: Option<&'static str>, // 16 bytes
    short: Option<char>, // 4 bytes
    long: Option<&'static str>, // 16 bytes
    description: Option<&'static str>, // 16 bytes
} // 56 bytes (4 more due to padding)
