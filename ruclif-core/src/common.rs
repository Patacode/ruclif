pub trait FromInto<I, O>: Sized {
    fn from_into(self, arg: I) -> O;
}
