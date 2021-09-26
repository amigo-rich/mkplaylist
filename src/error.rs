use std::fmt;
use std::path::PathBuf;

pub enum Error {
    IndexerPathNotDir(PathBuf),
    IndexerIO(std::io::Error),
    IndexerSend(std::sync::mpsc::SendError<PathBuf>),
    IndexerThread,
    StoreOpenNoFile(PathBuf),
    StoreRusqlite(rusqlite::Error),
    Usage,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            Error::IndexerPathNotDir(pb) => {
                let maybe_utf8 = pb.to_str().unwrap_or("unknown path");
                format!(
                    "The provided directory ('{}') is not a directory, or is not readable.",
                    maybe_utf8
                )
            }
            Error::IndexerIO(e) => format!("A IO Error '{}' occurred in the indexer thread.", e),
            Error::IndexerSend(e) => format!("A Send Error '{}' occured in the indexer thread.", e),
            Error::IndexerThread => "A panic occurred in the indexer thread.".to_string(),
            Error::StoreOpenNoFile(pb) => {
                let maybe_utf8 = pb.to_str().unwrap_or("unknown path");
                format!(
                    "The provided store path ('{}') is not a file, or is not readable.",
                    maybe_utf8
                )
            }
            Error::StoreRusqlite(e) => format!("A rusqlite Error '{}') occured", e),
            Error::Usage => "Please use mkplaylist --help for example use.".to_string(),
        };
        write!(f, "{}", description)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IndexerIO(e)
    }
}

impl From<std::sync::mpsc::SendError<PathBuf>> for Error {
    fn from(e: std::sync::mpsc::SendError<PathBuf>) -> Self {
        Error::IndexerSend(e)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Error::StoreRusqlite(e)
    }
}
