#[derive(Debug)]
pub enum Rating {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Rating {
    pub fn as_i64(&self) -> i64 {
        match self {
            Rating::One => 1,
            Rating::Two => 2,
            Rating::Three => 3,
            Rating::Four => 4,
            Rating::Five => 5,
        }
    }
}
