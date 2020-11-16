use std::fs::File;
use std::io::{Cursor, ErrorKind, IoSlice, IoSliceMut, Read, Seek, SeekFrom, Write};
use std::io;

#[derive(Debug)]
pub enum ResFile {
    File(File),
    ZipEntry(Cursor<Vec<u8>>),
}

impl Read for ResFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            ResFile::File(inner) => inner.read(buf),
            ResFile::ZipEntry(inner) => inner.read(buf),
        }
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        match self {
            ResFile::File(inner) => inner.read_vectored(bufs),
            ResFile::ZipEntry(inner) => inner.read_vectored(bufs),
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        match self {
            ResFile::File(inner) => inner.read_exact(buf),
            ResFile::ZipEntry(inner) => inner.read_exact(buf),
        }
    }
}

impl Write for ResFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            ResFile::File(inner) => inner.write(buf),
            ResFile::ZipEntry(_) => Err(io::Error::new(ErrorKind::Other, "unsupported write")),
        }
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        match self {
            ResFile::File(inner) => inner.write_vectored(bufs),
            ResFile::ZipEntry(_) => Err(io::Error::new(ErrorKind::Other, "unsupported write")),
        }
    }


    fn flush(&mut self) -> io::Result<()> {
        match self {
            ResFile::File(inner) => inner.flush(),
            ResFile::ZipEntry(_) => Err(io::Error::new(ErrorKind::Other, "unsupported write")),
        }
    }
}

impl Seek for ResFile {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match self {
            ResFile::File(inner) => inner.seek(pos),
            ResFile::ZipEntry(inner) => inner.seek(pos),
        }
    }

    #[cfg(feature = "seek_convenience")]
    fn stream_len(&mut self) -> io::Result<u64> {
        match self {
            ResFile::File(inner) => inner.stream_len(),
            ResFile::ZipEntry(inner) => inner.stream_len(),
        }
    }

    #[cfg(feature = "seek_convenience")]
    fn stream_position(&mut self) -> io::Result<u64> {
        match self {
            ResFile::File(inner) => inner.stream_position(),
            ResFile::ZipEntry(inner) => inner.stream_position(),
        }
    }
}