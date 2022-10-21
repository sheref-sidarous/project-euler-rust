
pub struct IndexedIter<T> 
where T : Iterator
{
    wrapped_iter: T,
    index : usize,
}

impl<T> IndexedIter<T> 
where T : Iterator
{
    pub fn new(wrapped_iter : T) -> IndexedIter<T> {
        IndexedIter {
            wrapped_iter : wrapped_iter,
            index : 0,
        }
    }
}

impl<T> Iterator for IndexedIter<T> 
where T : Iterator
{
    type Item = (<T as Iterator>::Item, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let output = match self.wrapped_iter.next() {
            Some(value) => {
                Some((value, self.index))
            },
            None => None,
        };

        self.index += 1;

        output
    }
}
