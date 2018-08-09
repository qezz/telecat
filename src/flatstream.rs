use futures::{Async, Poll, Stream};
use std::collections::VecDeque;

pub struct FlatStreamIter<S, T> {
    buffer: VecDeque<T>,
    stream: S,
}

impl<S, T> From<S> for FlatStreamIter<S, T>
where
    S: Stream,
    S::Item: IntoIterator<Item = T>,
{
    fn from(stream: S) -> Self {
        FlatStreamIter {
            buffer: VecDeque::new(),
            stream,
        }
    }
}

impl<S, T> Stream for FlatStreamIter<S, T>
where
    S: Stream,
    S::Item: IntoIterator<Item = T>,
{
    type Item = T;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            if let Some(buffered) = self.buffer.pop_front() {
                return Ok(Async::Ready(Some(buffered)));
            } else {
                match try_ready!(self.stream.poll()) {
                    None => return Ok(Async::Ready(None)),
                    Some(collection) => {
                        self.buffer.extend(collection);
                    }
                }
            }
        }
    }
}

pub trait StreamFlatExt: Stream {
    fn flat_iter<T>(self) -> FlatStreamIter<Self, T>
    where
        Self::Item: IntoIterator<Item = T>,
        Self: Sized,
    {
        self.into()
    }
}

impl<S: Stream> StreamFlatExt for S {}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream::iter_ok;

    #[test]
    fn it_works() {
        let stream = iter_ok::<_, ()>(vec![vec![1, 2], vec![3, 4, 5], vec![], vec![6]]);

        assert_eq!(
            Ok(vec!(1, 2, 3, 4, 5, 6)),
            stream.flat_iter().wait().collect()
        );
    }

    // #[test]
    // fn it_wor2ks() {
    //     let stream = iter_ok::<_, ()>(vec![vec![1, 2], vec![3, 4, 5], vec![], vec![6]]);
    //     panic!();
    //     assert_eq!(
    //         Ok(vec!(1, 2, 3, 4, 5, 6)),
    //         stream.flat_iter().wait().collect()// .for_each(|x| {
    //         //     println!("hello {}", x);
    //         // }
    //         // )
    //     );
    // }
}
