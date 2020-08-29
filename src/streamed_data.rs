#[derive(Debug)]
pub enum StreamedState<D> {
    Complete(D),
    Standalone(D),
    Incomplete,
}

pub trait StreamedData<D> {
    type Fragment;

    fn reset(&mut self);

    fn update(&mut self, fragment: Self::Fragment) -> StreamedState<D>
    where
        Self: Sized;
}
