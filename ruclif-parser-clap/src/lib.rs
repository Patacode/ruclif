pub mod string;

#[derive(Default, Debug, PartialEq)]
struct ClapArgData {
    name: &'static str,        // 16 bytes
    short: char,               // 4 bytes
    long: &'static str,        // 16 bytes
    description: &'static str, // 16 bytes
} // 56 bytes (4 more due to padding)
