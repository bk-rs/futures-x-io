use std::io::{self, SeekFrom};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_executor::block_on;

use futures_x_io::AsyncRead;
use futures_x_io::Cursor;
use futures_x_io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};

#[test]
fn buf_read() -> io::Result<()> {
    block_on(async {
        let mut cursor = BufReader::new(Cursor::new(b"foo-bar"));
        let mut buf = vec![];

        let n = cursor.read_until(b'-', &mut buf).await?;
        assert_eq!(n, 4);
        assert_eq!(buf, b"foo-");
        buf.clear();

        let n = cursor.read_until(b'-', &mut buf).await?;
        assert_eq!(n, 3);
        assert_eq!(buf, b"bar");
        buf.clear();

        let n = cursor.read_until(b'-', &mut buf).await?;
        assert_eq!(n, 0);
        assert_eq!(buf, b"");

        Ok(())
    })
}

#[test]
fn read() -> io::Result<()> {
    block_on(async {
        let mut cursor = Cursor::new(b"foo");

        let mut buf = vec![0u8; 5];
        let n = cursor.read(&mut buf).await?;
        assert_eq!(n, 3);
        assert_eq!(buf, b"foo\0\0");

        Ok(())
    })
}

#[test]
fn seek() -> io::Result<()> {
    block_on(async {
        let mut cursor = Cursor::new(b"foo");

        cursor.seek(SeekFrom::Start(1)).await?;

        let mut buf = vec![0u8; 3];
        let n = cursor.read(&mut buf).await?;
        assert_eq!(n, 2);
        assert_eq!(buf, b"oo\0");

        Ok(())
    })
}

#[test]
fn write() -> io::Result<()> {
    block_on(async {
        let mut cursor = Cursor::new(b"".to_vec());

        let n = cursor.write(b"foo").await?;
        assert_eq!(n, 3);
        assert_eq!(cursor.into_inner(), b"foo");

        Ok(())
    })
}

//
//
//
struct Foo;

impl AsyncRead for Foo {
    #[cfg(any(feature = "futures_io", feature = "tokio02_io"))]
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context,
        _buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(0))
    }

    #[cfg(feature = "tokio_io")]
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context,
        _buf: &mut tokio::io::ReadBuf,
    ) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

#[test]
fn foo() -> io::Result<()> {
    block_on(async {
        let mut foo = Foo {};

        let mut buf = vec![0u8; 3];
        let n = foo.read(&mut buf).await?;
        assert_eq!(n, 0);
        assert_eq!(buf, b"\0\0\0");

        Ok(())
    })
}
