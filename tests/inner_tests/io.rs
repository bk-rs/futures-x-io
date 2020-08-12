use std::io::{self, SeekFrom};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_x_io::poll_close_or_shutdown;
use futures_x_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

struct Foo;

impl AsyncBufRead for Foo {
    fn poll_fill_buf(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<&[u8]>> {
        Poll::Ready(Ok(b""))
    }

    fn consume(self: Pin<&mut Self>, _amt: usize) {}
}

impl AsyncRead for Foo {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context,
        _buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(0))
    }
}

impl AsyncSeek for Foo {
    #[cfg(feature = "futures_io")]
    fn poll_seek(self: Pin<&mut Self>, _cx: &mut Context, _pos: SeekFrom) -> Poll<io::Result<u64>> {
        Poll::Ready(Ok(0))
    }

    #[cfg(feature = "tokio_io")]
    fn start_seek(
        self: Pin<&mut Self>,
        _cx: &mut Context,
        _position: SeekFrom,
    ) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    #[cfg(feature = "tokio_io")]
    fn poll_complete(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<u64>> {
        Poll::Ready(Ok(0))
    }
}

impl AsyncWrite for Foo {
    fn poll_write(self: Pin<&mut Self>, _cx: &mut Context, _buf: &[u8]) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(0))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    poll_close_or_shutdown!(Poll::Ready(Ok(())));
}

#[test]
fn foo() -> io::Result<()> {
    Ok(())
}