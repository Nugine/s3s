use std::fmt;

pub type Result<T = (), E = Failed> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Failed {
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl<E> From<E> for Failed
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(source: E) -> Self {
        Self {
            source: Some(Box::new(source)),
        }
    }
}

impl fmt::Display for Failed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = &self.source {
            write!(f, "Failed: {source}")
        } else {
            write!(f, "Failed")
        }
    }
}
