pub(crate) mod bytes {
    use std::fmt::{self, Formatter};

    use bytes::Bytes;

    pub(crate) fn fmt(bytes: &Bytes, fmt: &mut Formatter) -> fmt::Result {
        let hex = hex::encode(bytes);
        write!(fmt, "{}", hex)
    }
}
