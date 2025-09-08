pub mod flag;
pub mod list;
pub mod string;

#[derive(Debug, PartialEq, Eq, Clone)]
struct ClapNamedArgData {
    name: &'static str,
    short: char,
    long: &'static str,
    description: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ClapPosArgData {
    name: &'static str,
    description: &'static str,
}
