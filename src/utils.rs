use std::cmp::Ordering;

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::Error;

pub(crate) fn prepend_padding<A, T>(vec: A, size: usize, with: T) -> Result<Vec<T>, Error>
where
    T: Clone,
    A: Into<Vec<T>>,
{
    let mut vec: Vec<_> = vec.into();

    match vec.len().cmp(&size) {
        Ordering::Greater => Err(Error::OverflowPadding),
        Ordering::Equal => Ok(vec),
        Ordering::Less => {
            let mut result = vec![with; size - vec.len()];
            result.append(&mut vec);
            Ok(result)
        }
    }
}

pub(crate) trait ChainedMac {
    fn chain(self, data: &[u8]) -> Self;
}

impl ChainedMac for Hmac<Sha256> {
    fn chain(mut self, data: &[u8]) -> Self {
        self.update(data);
        self
    }
}
