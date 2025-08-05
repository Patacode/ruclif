pub trait HasBuilder {
    type Builder;

    fn builder() -> Self::Builder;
}

pub trait Builder {
    type Result;

    fn build(self) -> Self::Result;
}
