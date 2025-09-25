use ruclif_core::builder::HasBuilder;
use ruclif_parser_clap::flag::Arg;
use ruclif_parser_clap::flag::ArgBuilder;

pub fn create_flag_arg() -> ArgBuilder {
    Arg::builder()
}
