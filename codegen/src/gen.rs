use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;

pub struct Codegen {
    writer: Box<dyn Write>,
}

impl Codegen {
    pub fn new(writer: impl Write + 'static) -> Self {
        Self {
            writer: Box::new(writer),
        }
    }

    pub fn create_file(path: &str) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self::new(BufWriter::with_capacity(1024 * 1024, file)))
    }
}

impl Codegen {
    pub fn lf(&mut self) {
        writeln!(self.writer).unwrap();
    }

    pub fn ln(&mut self, line: impl AsRef<str>) {
        writeln!(self.writer, "{}", line.as_ref()).unwrap();
    }
}
