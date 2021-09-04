use std::fmt;

enum Error {
    Io(std::io::Error),
    InvalidInput,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput => f.write_str("Invalid input"),
            Self::Io(e) => write!("{}", e.to_string()),
        }
    }
}

fn blq() -> Connnect4Result<()> {
    let f = std::fs::open("test")?;
}

type Connect4Result<T> = Result<T, Error>;

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
