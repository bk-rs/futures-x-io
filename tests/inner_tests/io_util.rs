use std::io::{self, SeekFrom};

use futures_executor::block_on;

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
fn seed() -> io::Result<()> {
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
