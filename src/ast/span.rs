use derive_more::{Deref, DerefMut};

#[derive(Debug)]
pub struct OwnedSpan;

#[derive(Debug, Deref, DerefMut)]
// #[as_ref(forward)]
// #[as_mut(forward)]
pub struct Spanned<T> {
    // #[as_ref]
    // #[as_mut]
    #[deref]
    #[deref_mut]
    inner: T,
    span: OwnedSpan,
}

impl<T> Spanned<T> {
    pub fn new(inner: T, span: OwnedSpan) -> Self {
        Self { inner, span }
    }
}
