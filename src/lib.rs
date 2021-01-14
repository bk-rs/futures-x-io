#[cfg(feature = "futures_io")]
pub mod futures_io {
    pub use ::futures_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};
}

#[cfg(feature = "tokio02_io")]
pub mod tokio02_io {
    pub use ::tokio02::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};
}

#[cfg(feature = "tokio_io")]
pub mod tokio_io {
    pub use tokio::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

    pub mod ext {
        pub use tokio::io::ReadBuf;
    }
}

#[cfg(all(
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use self::futures_io::*;

#[cfg(all(
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
pub use self::tokio02_io::*;

#[cfg(all(
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub use self::tokio_io::*;

//
//
//
#[cfg(feature = "futures_util_io")]
pub mod futures_util_io {
    pub use futures_util::io::Cursor;
    pub use futures_util::io::{
        AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader,
    };
}

#[cfg(feature = "futures_lite_io")]
pub mod futures_lite_io {
    pub use futures_lite::io::Cursor;
    pub use futures_lite::io::{
        AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader,
    };
}

#[cfg(feature = "tokio02_io_util")]
pub mod tokio02_io_util {
    pub use std::io::Cursor;
    pub use tokio02::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};
}

#[cfg(feature = "tokio_io_util")]
pub mod tokio_io_util {
    pub use std::io::Cursor;
    pub use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};
}

#[cfg(all(
    feature = "futures_util_io",
    not(feature = "futures_lite_io"),
    not(feature = "tokio02_io_util"),
    not(feature = "tokio_io_util"),
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use self::futures_util_io::*;

#[cfg(all(
    not(feature = "futures_util_io"),
    feature = "futures_lite_io",
    not(feature = "tokio02_io_util"),
    not(feature = "tokio_io_util"),
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use self::futures_lite_io::*;

#[cfg(all(
    not(feature = "futures_util_io"),
    not(feature = "futures_lite_io"),
    feature = "tokio02_io_util",
    not(feature = "tokio_io_util"),
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
pub use self::tokio02_io_util::*;

#[cfg(all(
    not(feature = "futures_util_io"),
    not(feature = "futures_lite_io"),
    not(feature = "tokio02_io_util"),
    feature = "tokio_io_util",
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub use self::tokio_io_util::*;
