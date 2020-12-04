use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Cursor {
    pub line: usize,
    pub index: usize,
    pub line_range: usize,
    pub index_range: usize,
    pub range: bool,
}

impl Cursor {
    pub fn get_lower(&self) -> (usize, usize) {
        if !self.range {
            return (self.line, self.index);
        }
        
        if self.line == self.line_range {
            if self.index < self.index_range { return (self.line, self.index); }
            else { return (self.line_range, self.index_range); }
        } else if self.line < self.line_range {
            return (self.line, self.index);
        } else {
            return (self.line_range, self.index_range);
        }
    }
    pub fn get_higher(&self) -> (usize, usize) {
        if !self.range {
            return (self.line, self.index);
        }
        
        if self.line == self.line_range {
            if self.index < self.index_range { return (self.line_range, self.index_range); }
            else { return (self.line, self.index); }
        } else if self.line < self.line_range {
            return (self.line_range, self.index_range);
        } else {
            return (self.line, self.index);
        }
    }
    pub fn is_lower(&self) -> bool {
        if !self.range {
            return true;
        }
        
        if self.line == self.line_range {
            if self.index < self.index_range { return true; }
            else { return false; }
        } else if self.line < self.line_range {
            return true;
        } else {
            return false;
        }
    }
}
