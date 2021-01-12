#[cfg(all(
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use futures_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

#[cfg(all(
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
pub use tokio02::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

#[cfg(all(
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub use tokio::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

//
//
//
#[cfg(all(
    feature = "futures_util_io",
    not(feature = "futures_lite_io"),
    not(feature = "tokio02_io_util"),
    not(feature = "tokio_io_util"),
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use futures_util::io::Cursor;
#[cfg(all(
    feature = "futures_util_io",
    not(feature = "futures_lite_io"),
    not(feature = "tokio02_io_util"),
    not(feature = "tokio_io_util"),
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use futures_util::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};

#[cfg(all(
    not(feature = "futures_util_io"),
    feature = "futures_lite_io",
    not(feature = "tokio02_io_util"),
    not(feature = "tokio_io_util"),
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use futures_lite::io::Cursor;
#[cfg(all(
    not(feature = "futures_util_io"),
    feature = "futures_lite_io",
    not(feature = "tokio02_io_util"),
    not(feature = "tokio_io_util"),
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use futures_lite::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};

#[cfg(all(
    not(feature = "futures_util_io"),
    not(feature = "futures_lite_io"),
    feature = "tokio02_io_util",
    not(feature = "tokio_io_util"),
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
pub use std::io::Cursor;
#[cfg(all(
    not(feature = "futures_util_io"),
    not(feature = "futures_lite_io"),
    feature = "tokio02_io_util",
    not(feature = "tokio_io_util"),
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
pub use tokio02::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};

#[cfg(all(
    not(feature = "futures_util_io"),
    not(feature = "futures_lite_io"),
    not(feature = "tokio02_io_util"),
    feature = "tokio_io_util",
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub use std::io::Cursor;
#[cfg(all(
    not(feature = "futures_util_io"),
    not(feature = "futures_lite_io"),
    not(feature = "tokio02_io_util"),
    feature = "tokio_io_util",
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};
