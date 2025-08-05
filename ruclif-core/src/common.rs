pub trait IntoFrom<U, T>: Sized {
    fn into_from(self, arg: U) -> T;
}
