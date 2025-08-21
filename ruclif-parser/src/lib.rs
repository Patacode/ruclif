use ruclif_core::builder::HasBuilder;
use ruclif_parser_clap::flag::{FlagClapArg, FlagClapArgBuilder};

pub fn create_flag_arg() -> FlagClapArgBuilder {
    FlagClapArg::builder()
}
