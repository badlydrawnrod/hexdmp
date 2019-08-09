use std::error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum HexDumpError {
    UnknownSuffix,
    Parse(ParseIntError),
}

impl fmt::Display for HexDumpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            HexDumpError::UnknownSuffix => write!(f, "Unknown suffix"),
            HexDumpError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for HexDumpError {
    fn description(&self) -> &str {
        match *self {
            HexDumpError::UnknownSuffix => &"Unknown suffix",
            HexDumpError::Parse(ref err) => err.description(),
        }
    }
}

impl From<ParseIntError> for HexDumpError {
    fn from(err: ParseIntError) -> HexDumpError {
        HexDumpError::Parse(err)
    }
}
