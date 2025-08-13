pub mod string;

#[derive(Debug, PartialEq, Eq)]
struct ClapArgData {
    name: &'static str,        // 16 bytes
    short: char,               // 4 bytes
    long: &'static str,        // 16 bytes
    description: &'static str, // 16 bytes
} // 56 bytes (4 more due to padding)
