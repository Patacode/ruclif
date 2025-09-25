use crate::ArgBuilderData;
use crate::NamedArgData;

mod impls;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action {
    SetTrue,
    SetFalse,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArgData {
    common: NamedArgData,
    action: Action,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Arg {
    data: ArgData,
}

#[rustfmt::skip]
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ArgBuilder {
    common: ArgBuilderData,
    action: Option<Action>,
    state:  u8,
}

mod arg_builder_state {
    pub use crate::common_arg_builder_state::*;
}
