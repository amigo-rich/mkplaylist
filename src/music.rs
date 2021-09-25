use std::fmt;

#[derive(Debug)]
pub struct Music {
    pub path: String,
}

impl fmt::Display for Music {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}
