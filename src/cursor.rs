use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Cursor {
    pub line: usize,
    pub index: usize,
    pub pair: Option<Box<Cursor>>,
}
