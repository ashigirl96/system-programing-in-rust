use crate::io::SectionReader;
use std::io::{self, Read, Seek};

#[derive(Debug)]
pub struct LimitedReader<R: Read + Seek>(SectionReader<R>);

impl<R> LimitedReader<R>
where
    R: Read + Seek,
{
    pub fn new(reader: R, n_byte: usize) -> io::Result<Self> {
        Ok(Self(SectionReader::new(reader, 0, n_byte)?))
    }
}

impl<R> Read for LimitedReader<R>
where
    R: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}
