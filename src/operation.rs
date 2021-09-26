use std::path::PathBuf;

#[derive(Debug)]
pub enum Operation<'a> {
    Index(PathBuf),
    PlayList(Option<&'a str>, bool),
    Rate(i64, i64),
}
