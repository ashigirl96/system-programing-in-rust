use std::io::{self, Write};

pub struct MultiWriter(Vec<Box<dyn Write>>);

impl MultiWriter {
    pub fn new(writers: Vec<Box<dyn Write>>) -> Self {
        Self(writers)
    }
}

impl Write for MultiWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for w in &mut self.0 {
            w.write_all(buf)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        for w in &mut self.0 {
            w.flush()?;
        }
        Ok(())
    }
}
