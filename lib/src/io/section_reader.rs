use std::io::{self, Read, Seek, SeekFrom};

// 与えられたReaderのバイト窓(limited section)のみ読み込む
// バイナリファイルの部分読み込みに便利

// Seeker: func Seek(offset int64, whence int) (int64, error)メソッドを持ちます。読み書き位置を移動します。

#[derive(Debug)]
pub struct SectionReader<R: Read + Seek> {
    seeked_reader: R,
    reads_upto: usize,
}

impl<R> SectionReader<R>
where
    R: Read + Seek,
{
    pub fn new(mut reader: R, offset_byte: u64, n_byte: usize) -> io::Result<Self> {
        reader.seek(SeekFrom::Start(offset_byte))?;
        Ok(Self {
            seeked_reader: reader,
            reads_upto: n_byte,
        })
    }
}

impl<R> Read for SectionReader<R>
where
    R: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.reads_upto == 0 {
            return Ok(0); // pseudo EOF
        }
        let read_bytes = self.seeked_reader.read(buf)?;
        if read_bytes == 0 {
            return Ok(0); // got EOF
        }
        if read_bytes <= self.reads_upto {
            self.reads_upto -= read_bytes;
            return Ok(read_bytes);
        }
        let ret = self.reads_upto;
        self.reads_upto = 0;
        Ok(ret)
    }
}
