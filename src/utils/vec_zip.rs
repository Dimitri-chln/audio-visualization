#[derive(Debug)]
pub struct VecZip<T> {
    iterators: Vec<std::vec::IntoIter<T>>,
}

impl<T> VecZip<T> {
    pub fn new(iterators: Vec<Vec<T>>) -> Self {
        Self {
            iterators: iterators
                .into_iter()
                .map(IntoIterator::into_iter)
                .collect::<Vec<_>>(),
        }
    }
}

impl<T> Iterator for VecZip<T> {
    type Item = Vec<Option<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut not_finished = false;

        let values = self
            .iterators
            .iter_mut()
            .map(|iterator| iterator.next().inspect(|_| not_finished = true))
            .collect();

        not_finished.then_some(values)
    }
}
