use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use bytes::{Buf, BytesMut};
use dashmap::DashMap;
use hyper::body::HttpBody;
use hyper::client::connect::HttpConnector;
use hyper::{Client, Uri};
use lazy_static::lazy_static;

use crate::core::tx::Tx;
use crate::utils::default;
use crate::{Error, Result};

lazy_static! {
    pub static ref TX_FETCHER: TxFetcher = TxFetcher::new();
}

#[derive(Debug)]
pub struct TxFetcher {
    cache: DashMap<String, Tx>,
    client: Client<HttpConnector>,
}

impl TxFetcher {
    fn new() -> Self {
        Self {
            cache: default(),
            client: default(),
        }
    }

    const fn get_url(testnet: bool) -> &'static str {
        if testnet {
            "http://testnet.programmingbitcoin.com"
        } else {
            "http://mainnet.programmingbitcoin.com"
        }
    }

    pub async fn fetch(&self, tx_id: &str, testnet: bool, fresh: bool) -> Result<Tx> {
        if fresh || !self.cache.contains_key(tx_id) {
            let url = format!("{}/tx/{}.hex", Self::get_url(testnet), hex::encode(tx_id));
            let uri: Uri = url.parse().unwrap();

            let mut response = self.client.get(uri).await?;
            let mut bytes = BytesMut::with_capacity(response.size_hint().lower() as usize);

            while let Some(chunk) = response.data().await {
                bytes.extend_from_slice(&chunk?);
            }

            let tx = if bytes[4] == 0x0 {
                let chain = bytes[..4].chain(&bytes[6..]);
                let mut tx = Tx::deserialize(chain, testnet)?;
                let mut last_four = Cursor::new(&bytes[(bytes.len() - 4)..]);
                tx.locktime = last_four.read_u64::<LittleEndian>()?;

                tx
            } else {
                Tx::deserialize(bytes, testnet)?
            };

            if tx.id()? != tx_id {
                return Err(Error::FetchedInvalidTransaction);
            }

            self.cache.insert(tx_id.to_string(), tx);
        }

        self.cache.get_mut(tx_id).unwrap().testnet = testnet;
        return Ok(self.cache.get(tx_id).unwrap().value().clone());
    }
}
