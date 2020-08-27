#[derive(Debug)]
pub enum StreamedState<I, D> {
    Complete(D),
    Standalone(D),
    Incomplete(I),
}

pub trait StreamedData<D> {
    type Fragment;

    fn update(self, fragment: Self::Fragment) -> StreamedState<Self, D>
    where
        Self: Sized;
}
