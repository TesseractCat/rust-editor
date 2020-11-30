use crate::cursor::*;

pub struct Buffer {
    pub path: Option<String>,
    pub lines: Vec<String>,
    pub cursors: Vec<Box<Cursor>>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            path: None,
            lines: vec!["Scratch buffer".to_string(), "".to_string(), "Text".to_string()],
            cursors: vec![Box::new(Cursor {
                line:0,
                index:0,
                pair:None
            })],
        }
    }
}
