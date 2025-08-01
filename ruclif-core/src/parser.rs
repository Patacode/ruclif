pub trait CliArg<A, P, T> {
    fn build(&self) -> A;
    fn convert(&self, parsing_result: &P) -> T;
}
