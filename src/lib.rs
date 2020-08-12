cfg_if::cfg_if! {
    if #[cfg(all(feature = "futures_io", not(feature = "tokio_io")))] {
        pub use futures_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

        #[macro_export]
        macro_rules! poll_close_or_shutdown {
            ( $e:expr ) => {
                fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
                    $e
                }
            }
        }
    } else if #[cfg(all(not(feature = "futures_io"), feature = "tokio_io"))] {
        pub use tokio::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

        #[macro_export]
        macro_rules! poll_close_or_shutdown {
            ( $e:expr ) => {
                fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
                    $e
                }
            }
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "futures_util_io", not(feature = "futures_lite_io"), not(feature = "tokio_io_util")))] {
        pub use futures_util::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};
        pub use futures_util::io::Cursor;
    } else if #[cfg(all(not(feature = "futures_util_io"), feature = "futures_lite_io", not(feature = "tokio_io_util")))] {
        pub use futures_lite::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};
        pub use futures_lite::io::Cursor;
    } else if #[cfg(all(not(feature = "futures_util_io"), not(feature = "futures_lite_io"), feature = "tokio_io_util"))] {
        pub use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};
        pub use std::io::Cursor;
    }
}
