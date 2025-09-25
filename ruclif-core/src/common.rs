pub trait FromInto<I, O>: Sized {
    fn into_from(self, arg: I) -> O;
}
