pub struct Error;

pub fn parse<'a, T>(input: &'a str, f: impl FnOnce(&mut Parser<'a>) -> Result<T>) -> Result<T> {
    let mut p = Parser::new(input);
    let val = f(&mut p)?;
    Ok(val)
}

pub struct Parser<'a> {
    input: &'a str,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn nom<T>(&mut self, f: impl FnOnce(&'a str) -> nom::IResult<&'a str, T>) -> Result<T> {
        match f(self.input) {
            Ok((input, output)) => {
                self.input = input;
                Ok(output)
            }
            Err(_) => Err(Error),
        }
    }
}
