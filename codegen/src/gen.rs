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
}

impl Codegen {
    pub fn lf(&mut self) {
        writeln!(self.writer).unwrap()
    }

    pub fn ln(&mut self, line: impl AsRef<str>) {
        writeln!(self.writer, "{}", line.as_ref()).unwrap()
    }
}
