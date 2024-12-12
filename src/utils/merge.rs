pub trait Merge<T> {
    fn merge(self) -> Vec<T>;
}

impl<T> Merge<T> for (&[T], &[T])
where
    T: Clone,
{
    fn merge(self) -> Vec<T> {
        let (mut left, right) = (self.0.to_owned(), self.1.to_owned());
        left.extend(right);
        left
    }
}
