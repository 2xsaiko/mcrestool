pub struct TryIter<'a, E, I> {
    iter: I,
    failed: &'a mut Option<E>,
}

impl<T, E, I> Iterator for TryIter<'_, E, I>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed.is_none() {
            match self.iter.next() {
                None => None,
                Some(Ok(v)) => Some(v),
                Some(Err(e)) => {
                    *self.failed = Some(e);
                    None
                }
            }
        } else {
            None
        }
    }
}

pub fn try_iter<T, R, E, I, F>(iter: I, mapper: F) -> Result<R, E>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnOnce(TryIter<E, I>) -> R,
{
    let mut failed = None;
    let iter = TryIter {
        iter,
        failed: &mut failed,
    };
    let result = mapper(iter);
    match failed {
        None => Ok(result),
        Some(e) => Err(e),
    }
}
