use std::cmp::Ordering;

use hmac::{Hmac, Mac};
use ripemd160::Ripemd160;
use sha2::{Digest, Sha256};

use crate::{Error, Result};

pub(crate) fn prepend_padding<A, T>(vec: A, size: usize, with: T) -> Result<Vec<T>>
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

pub(crate) fn strip_start<T>(arr: &[T], elem: T) -> &[T]
where
    T: Eq,
{
    let mut new_start = 0;

    while arr[new_start] == elem {
        new_start += 1;

        if new_start == arr.len() {
            break;
        }
    }

    &arr[new_start..]
}

pub fn hash160<B>(data: B) -> Vec<u8>
where
    B: AsRef<[u8]>,
{
    let hasher = Sha256::new();
    let digest = hasher.chain(data.as_ref()).finalize();

    let hasher = Ripemd160::new();
    let digest = hasher.chain(digest).finalize();

    digest.as_slice().to_vec()
}

pub fn hash256<B>(data: B) -> Vec<u8>
where
    B: AsRef<[u8]>,
{
    let mut hasher = Sha256::new();

    hasher.update(data.as_ref());
    let digest = hasher.finalize_reset();

    hasher.update(digest);
    let digest = hasher.finalize();

    digest.as_slice().to_vec()
}

pub(crate) trait Chain {
    fn chain(self, data: &[u8]) -> Self;
}

impl Chain for Hmac<Sha256> {
    fn chain(mut self, data: &[u8]) -> Self {
        self.update(data);
        self
    }
}
