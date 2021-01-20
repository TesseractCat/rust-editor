use crate::cursor::*;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub struct Buffer {
    pub path: Option<String>,
    
    pub viewport: usize,
    pub height: usize,
    
    pub lines: Vec<String>,
    pub file_as_string: String,
    pub dirty: bool,
    
    pub cursors: Vec<Box<Cursor>>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            path: None,
            viewport:0,
            height:50,
            lines: vec!["Scratch buffer".to_string(), "".to_string(), "Text".to_string()],
            file_as_string: vec!["Scratch buffer".to_string(), "".to_string(), "Text".to_string()].join("\n"),
            dirty: false,
            cursors: vec![Box::new(Cursor {
                line:0,
                index:0,
                line_range:0,
                index_range:0,
                range:false,
            })],
        }
    }
    
    pub fn load_path(&mut self, path: &str) {
        let file = File::open(path);
        let file = match file {
            Ok(f) => f,
            Err(_e) => {
                tinyfiledialogs::message_box_ok("Error", "Error opening file", tinyfiledialogs::MessageBoxIcon::Error);
                return;
            },
        };
        self.lines = io::BufReader::new(file)
            .lines()
            .map(|l| l.expect("<Error parsing line>"))
            .collect();
        self.path = Some(path.to_string());
        self.dirty = true;
    }
    
    pub fn write(&mut self) {
        if self.path == None {
            tinyfiledialogs::message_box_ok("Error", "Can't write scratch buffer", tinyfiledialogs::MessageBoxIcon::Error);
            return;
        }
        
        let file = File::create(self.path.as_ref().unwrap());
        let mut file = match file {
            Ok(f) => f,
            Err(_e) => {
                tinyfiledialogs::message_box_ok("Error", "Error writing file", tinyfiledialogs::MessageBoxIcon::Error);
                return;
            },
        };
        file.write_all(self.lines.join("\n").as_bytes()).ok();
    }
    
    pub fn add_before_cursor(&mut self, cursor: &Box<Cursor>, new_content: &str) {
        let lower = cursor.get_lower();
        
        if lower.1 >= self.lines[lower.0].len() {
            self.lines[lower.0].push_str(new_content);
        } else {
            self.lines[lower.0].insert_str(lower.1, new_content);
        }
        self.dirty = true;
    }
    
    pub fn sub_cursor(&mut self, cursor: &Box<Cursor>, new_content: &str) {
        let new_content_vec: Vec<&str> = new_content.split('\n').collect();
        
        //Find lower and higher indices of the cursor, and constrain their column to line length - 1.
        let mut lower = cursor.get_lower();
        let mut higher = cursor.get_higher();
        lower.1 = cmp::min(lower.1, self.lines[lower.0].len().checked_sub(1).unwrap_or(0));
        higher.1 = cmp::min(higher.1, self.lines[higher.0].len().checked_sub(1).unwrap_or(0));
        
        if lower.0 == higher.0 {
            if self.lines[lower.0].len() == 0 {
                self.lines[lower.0].push_str(new_content);
            } else {
                self.lines[lower.0].replace_range(lower.1..=higher.1, new_content);
            }
            let newline_split: Vec<String> =
                self.lines[lower.0].split('\n').collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect();
            if newline_split.len() > 1 {
                //Handle newlines
                self.lines.splice(lower.0..=lower.0, newline_split.iter().cloned());
            }
        } else {
            for i in ((lower.0 + 1)..=(higher.0 - 1)).rev() {
                self.lines.remove(i);
            }
            self.lines[lower.0].replace_range(lower.1.., "");
            self.lines[lower.0].push_str(new_content_vec[0]);
            
            let higher_line = self.lines[lower.0 + 1].clone();
            self.lines.remove(lower.0 + 1);
            
            for i in (1..new_content_vec.len()).rev() {
                self.lines.insert(lower.0 + 1, new_content_vec[i].to_string());
            }
            self.lines[lower.0 + new_content_vec.len() - 1].push_str(&higher_line.get((higher.1 + 1)..).unwrap_or(&""));
        }
        self.dirty = true;
    }
    
    pub fn get_in_cursor(&mut self, cursor: &Box<Cursor>) -> String {
        let mut out: String = "".to_string();
        
        let mut lower = cursor.get_lower();
        let mut higher = cursor.get_higher();
        lower.1 = cmp::min(lower.1, self.lines[lower.0].len().checked_sub(1).unwrap_or(0));
        higher.1 = cmp::min(higher.1, self.lines[higher.0].len().checked_sub(1).unwrap_or(0));
        
        if lower.0 == higher.0 {
            if self.lines[lower.0].len() != 0 {
                out.push_str(&self.lines[lower.0].get(lower.1..=higher.1).unwrap_or(&""));
            }
        } else {
            out.push_str(self.lines[lower.0].get(lower.1..).unwrap_or(&""));
            out.push('\n');
            for i in (lower.0 + 1)..=(higher.0 - 1) {
                out.push_str(&self.lines[i]);
                out.push('\n');
            }
            out.push_str(self.lines[higher.0].get(0..(higher.1+1)).unwrap_or(&""));
        }
        
        return out;
    }
    
    pub fn get_as_string(&mut self) -> &str {
        if self.dirty {
            self.file_as_string = self.lines.join("\n");
        }
        return &self.file_as_string;
    }
    
    pub fn string_idx_to_cursor_idx(&mut self, idx: usize) -> (usize, usize) {
        let self_as_string: &str = self.get_as_string();
        let line = self_as_string[..=idx].matches('\n').count();
        if line == 0 {
            return (0, idx);
        }
        for (i, si) in (0..=idx).rev().enumerate() {
            if self_as_string.as_bytes()[si] as char == '\n' {
                return (line, i - 1);
            }
        }
        return (0,0);
    }
}
