use std::path::PathBuf;

#[derive(Debug)]
pub enum PlayList<'a> {
    Standard,
    Filtered(&'a str),
    Shuffled,
    ShuffledFiltered(&'a str),
}

#[derive(Debug)]
pub enum Operation<'a> {
    Create(PlayList<'a>),
    Index(PathBuf),
    Rate(i64, i64),
}
