use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::buffer::*;
use crate::cursor::*;
use serde_json::Value;
use regex::*;

type EditAction = fn(&mut Buffer, usize, Option<&str>) -> ();

/*lazy_static! {
    pub static ref ACTIONMAP: HashMap<&'static str, EditAction> = {
        let mut m = HashMap::<&'static str, EditAction>::new();
        m.insert("move_up", move_up);
        m.insert("move_down", move_down);
        m.insert("move_left", move_left);
        m.insert("move_right", move_right);
        
        m.insert("viewport_up", viewport_up);
        m.insert("viewport_down", viewport_down);
        
        m.insert("move_beginning", move_beginning);
        m.insert("move_end", move_end);
        
        m.insert("move_front", move_front);
        m.insert("move_back", move_back);
        m.insert("move_word", move_word);
        m.insert("move_word_reverse", move_word_reverse);
        
        m.insert("move_find", move_find);
        m.insert("move_find_reverse", move_find_reverse);
        m.insert("move_till", move_till);
        m.insert("move_till_reverse", move_till_reverse);
        
        m.insert("new_cursor_up", new_cursor_up);
        m.insert("new_cursor_down", new_cursor_down);
        
        m.insert("delete_char", delete_char);
        m.insert("delete_back", delete_back);
        m.insert("replace_char", replace);
        m.insert("toggle_char_case", toggle_char_case);
        m.insert("insert_char", insert_char);
        m.insert("delete_line", delete_line);
        m.insert("delete", delete);
        
        m.insert("split_line", split_line);
        m.insert("join_line", join_line);
        
        m.insert("open_up", open_up);
        m.insert("open_down", open_down);
        
        m.insert("toggle_selection", toggle_selection);
        m.insert("swap_selection", swap_selection);
        
        m.insert("open_file", open_file);
        m
    };
}

pub fn execute_action(buffer: &mut Buffer, command: &str, motion: &Value, key: Option<&str>) -> Vec<(usize, String)> {
    let mut line_changes: Vec<(usize, String)> = vec![];
    
    if motion != &Value::Null {
        execute_action(buffer, "toggleSelection", &Value::Null, None);
        execute_action(buffer, motion["action"].as_str().unwrap(), &Value::Null, motion["key"].as_str());
        //This action is the only one to modify the file
        line_changes = execute_action(buffer, command, &Value::Null, None);
        execute_action(buffer, "toggleSelection", &Value::Null, None);
    } else {
        buffer.cursors.sort_by(|c, d| {
            d.line.cmp(&c.line).then_with(|| d.index.cmp(&c.index))
        });
        //Iterates from end of file to beginning
        for c in 0..buffer.cursors.len() {
            let mut lines_diff: isize = buffer.lines.len() as isize;
            let mut columns_diff: isize = buffer.lines[buffer.cursors[c].line].len() as isize;
            let line_prev: String = buffer.lines[buffer.cursors[c].line].to_owned();
            
            match ACTIONMAP.get(command) {
                Some(x) => x(buffer, c, key),
                None => ()
            }
            lines_diff = (buffer.lines.len() as isize) - lines_diff;
            columns_diff = (buffer.lines[buffer.cursors[c].line].len() as isize) - columns_diff;
            
            //Push line change
            if line_prev != buffer.lines[buffer.cursors[c].line] {
                line_changes.push((buffer.cursors[c].line, buffer.lines[buffer.cursors[c].line].to_owned()));
            }
            
            //Shift previous cursors
            for d in 0..c {
                buffer.cursors[d].line =
                    (buffer.cursors[d].line as isize + lines_diff) as usize;
                if buffer.cursors[d].line == buffer.cursors[c].line {
                    buffer.cursors[d].index =
                        (buffer.cursors[d].index as isize + columns_diff) as usize;
                }
            }
        }
    }
    return line_changes;
}*/

pub fn execute_action(action: EditAction, buffer: &mut Buffer, key: Option<&str>) {
    for c in 0..buffer.cursors.len() {
        action(buffer, c, key);
    }
}

pub fn constrain_cursor(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.lines[buffer.cursors[c].line].chars().count() == 0 {
        buffer.cursors[c].index = 0;
    } else if buffer.cursors[c].index > buffer.lines[buffer.cursors[c].line].chars().count() {
        buffer.cursors[c].index = buffer.lines[buffer.cursors[c].line].chars().count();
    }
}

//Movement
pub fn move_up(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.cursors[c].line > 0 {
        buffer.cursors[c].line -= 1;
    }
}
pub fn move_down(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.cursors[c].line < buffer.lines.len() - 1 {
        buffer.cursors[c].line += 1;
    }
}
pub fn move_left(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    constrain_cursor(buffer, c, None);
    
    if buffer.cursors[c].index > 0 {
        buffer.cursors[c].index -= 1;
    }
}
pub fn move_right(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.cursors[c].index += 1;
    constrain_cursor(buffer, c, None);
}

pub fn move_front(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.cursors[c].index = 0;
}
pub fn move_back(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.lines[buffer.cursors[c].line].len() != 0 {
        buffer.cursors[c].index = buffer.lines[buffer.cursors[c].line].chars().count()-1;
    }
}
pub fn move_word(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    lazy_static! {
        //static ref WORDRE: Regex = Regex::new("(([A-z]|[0-9]|_)+|[^a-zA-Z0-9\\s]+)").unwrap();
        static ref WORDRE: Regex = Regex::new("([A-z]|[0-9]|_)+").unwrap();
    }
    for cap in WORDRE.captures_iter(&buffer.lines[buffer.cursors[c].line]) {
        if cap.get(1).unwrap().start() > buffer.cursors[c].index {
            buffer.cursors[c].index = cap.get(1).unwrap().start();
            return;
        }
    }
    //Finally, move to the back
    move_back(buffer, c, _key);
}
pub fn move_word_reverse(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    lazy_static! {
        //static ref WORDRE: Regex = Regex::new("(([A-z]|[0-9]|_)+|[^a-zA-Z0-9\\s]+)").unwrap();
        static ref WORDRE: Regex = Regex::new("([A-z]|[0-9]|_)+").unwrap();
    }
    let mut matches: Vec<Match> =
        WORDRE.captures_iter(&buffer.lines[buffer.cursors[c].line]).map(|x| x.get(0).unwrap()).collect::<Vec<Match>>();
    matches.reverse();
    
    for cap in matches {
        if cap.start() < buffer.cursors[c].index {
            buffer.cursors[c].index = cap.start();
            return;
        }
    }
    //Finally, move to the front
    move_front(buffer, c, _key);
}
pub fn move_find(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    let mut line_iter = buffer.lines[buffer.cursors[c].line].chars();
    let key_as_char: char = _key.unwrap().chars().next().unwrap();
    let mut i = buffer.cursors[c].index + 1;
    
    line_iter.nth(buffer.cursors[c].index);
    while let Some(next_char) = line_iter.next() {
        if next_char == key_as_char {
            buffer.cursors[c].index = i;
            return;
        }
        i += 1;
    }
}
pub fn move_find_reverse(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    let mut line_iter = buffer.lines[buffer.cursors[c].line].chars().rev();
    let key_as_char: char = _key.unwrap().chars().next().unwrap();
    let mut i = buffer.cursors[c].index;
    
    line_iter.nth((buffer.lines[buffer.cursors[c].line].len() - 1) - buffer.cursors[c].index);
    while let Some(next_char) = line_iter.next() {
        if next_char == key_as_char {
            buffer.cursors[c].index = i - 1;
            return;
        }
        i -= 1;
    }
}
pub fn move_till(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    move_find(buffer, c, _key);
    match buffer.lines[buffer.cursors[c].line].chars().nth(buffer.cursors[c].index) {
        Some(x) => {
            if x.to_string() == _key.unwrap() {
                move_left(buffer, c, _key);
            }
        },
        None => ()
    }
}
pub fn move_till_reverse(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    move_find_reverse(buffer, c, _key);
    match buffer.lines[buffer.cursors[c].line].chars().nth(buffer.cursors[c].index) {
        Some(x) => {
            if x.to_string() == _key.unwrap() {
                move_right(buffer, c, _key);
            }
        },
        None => ()
    }
}

pub fn move_beginning(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if c != 0 { return; }
    
    buffer.cursors[c].line = 0;
    buffer.cursors[c].index = 0;
}
pub fn move_end(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if c != 0 { return; }
    
    buffer.cursors[c].line = buffer.lines.len().checked_sub(1).unwrap_or(0);
    buffer.cursors[c].index =
        buffer.lines.get(buffer.cursors[c].line)
        .unwrap_or(&("".to_owned())).len()
        .checked_sub(1).unwrap_or(0);
}

//Viewport mutation TODO
pub fn viewport_up(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if c != 0 { return; }
}
pub fn viewport_down(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if c != 0 { return; }
}

//Cursor mutation
pub fn new_cursor_up(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.cursors.sort_by(|c, d| {
        d.line.cmp(&c.line).then_with(|| d.index.cmp(&c.index))
    });
    
    if buffer.cursors[c].line == 0 || c != buffer.cursors.len() - 1 {
        return;
    }
    buffer.cursors.push(Box::new(Cursor {
        line:buffer.cursors[c].line - 1,
        index:buffer.cursors[c].index,
        line_range:0,
        index_range:0,
        range:false,
    }));
}
pub fn new_cursor_down(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.cursors.sort_by(|c, d| {
        d.line.cmp(&c.line).then_with(|| d.index.cmp(&c.index))
    });
    
    if buffer.cursors[c].line >= buffer.lines.len() - 1 || c != 0 {
        return;
    }
    buffer.cursors.push(Box::new(Cursor {
        line:buffer.cursors[c].line + 1,
        index:buffer.cursors[c].index,
        line_range:0,
        index_range:0,
        range:false,
    }));
}

//Selection
pub fn toggle_selection(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.cursors[c].range = !buffer.cursors[c].range;
    buffer.cursors[c].line_range = buffer.cursors[c].line;
    buffer.cursors[c].index_range = buffer.cursors[c].index;
}
pub fn swap_selection(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    let temp_line = buffer.cursors[c].line;
    buffer.cursors[c].line = buffer.cursors[c].line_range;
    buffer.cursors[c].line_range = temp_line;
    
    let temp_index = buffer.cursors[c].index;
    buffer.cursors[c].index = buffer.cursors[c].index_range;
    buffer.cursors[c].index_range = temp_index;
}

//Text mutation
pub fn delete(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.cursors[c].range {
        buffer.sub_cursor(&buffer.cursors[c].clone(), "");
        if !buffer.cursors[c].is_lower() {
            swap_selection(buffer, c, _key);
        }
    }
}
pub fn delete_char(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    constrain_cursor(buffer, c, None);
    if buffer.lines[buffer.cursors[c].line].chars().count() > 0
        && buffer.cursors[c].index < buffer.lines[buffer.cursors[c].line].chars().count() {
        buffer.lines[buffer.cursors[c].line].replace_range(
            buffer.cursors[c].index..=buffer.cursors[c].index, "");
    } else {
        move_left(buffer, c, _key);
    }
    buffer.dirty = true;
}
pub fn delete_back(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.cursors[c].index > 0 {
        move_left(buffer, c, _key);
        delete_char(buffer, c, _key);
    } else if buffer.cursors[c].line > 0 {
        move_up(buffer, c, _key);
        join_line(buffer, c, _key);
    }
    buffer.dirty = true;
}
pub fn replace(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    let mut to_sub_text = buffer.get_in_cursor(&buffer.cursors[c].clone());
    
    let key = _key.unwrap().chars().next().unwrap();
    to_sub_text = to_sub_text.chars().map(|c| {
        if c == '\n' {return '\n';}
        return key;
    }).collect();
    buffer.sub_cursor(&buffer.cursors[c].clone(), &to_sub_text);
    if !buffer.cursors[c].is_lower() {
        swap_selection(buffer, c, _key);
    }
}
pub fn toggle_char_case(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    let mut to_sub_text = buffer.get_in_cursor(&buffer.cursors[c].clone());
    
    to_sub_text = to_sub_text.chars().map(|c| {
        if c == '\n' {return '\n';}
        if c.is_ascii_uppercase() {
            return c.to_ascii_lowercase();
        } else {
            return c.to_ascii_uppercase();
        }
    }).collect();
    buffer.sub_cursor(&buffer.cursors[c].clone(), &to_sub_text);
    if !buffer.cursors[c].is_lower() {
        swap_selection(buffer, c, _key);
    }
}
pub fn insert_char(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.add_before_cursor(&buffer.cursors[c].clone(), _key.unwrap());
}

//Line mutation
pub fn delete_line(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.lines.remove(buffer.cursors[c].line);
    if buffer.cursors[c].line >= buffer.lines.len() {
        move_up(buffer, c, _key);
    }
    if buffer.lines.len() == 0 {
        buffer.lines.push("".to_string());
    }
    buffer.dirty = true;
}
pub fn split_line(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.lines[buffer.cursors[c].line].len() > 0 {
        let front_split = buffer.lines[buffer.cursors[c].line][..buffer.cursors[c].index].to_string();
        let back_split = buffer.lines[buffer.cursors[c].line][buffer.cursors[c].index..].to_string();
        buffer.lines[buffer.cursors[c].line] = front_split;
        buffer.lines.insert(buffer.cursors[c].line + 1, back_split);
    } else {
        open_down(buffer, c, _key);
    }
    buffer.dirty = true;
}
pub fn join_line(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.cursors[c].line < buffer.lines.len() - 1 {
        let next_line = buffer.lines[buffer.cursors[c].line+1].clone();
        let line_len = buffer.lines[buffer.cursors[c].line].len();
        buffer.lines[buffer.cursors[c].line].push_str(&next_line);
        buffer.lines.remove(buffer.cursors[c].line+1);
        buffer.cursors[c].index = line_len;
    }
    constrain_cursor(buffer, c, None);
    buffer.dirty = true;
}
pub fn open_up(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.lines.insert(buffer.cursors[c].line, "".to_string());
    buffer.dirty = true;
}
pub fn open_down(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.lines.insert(buffer.cursors[c].line + 1, "".to_string());
    buffer.dirty = true;
}

pub fn open_file(_buffer: &mut Buffer, _c: usize, _key: Option<&str>) {
    println!("File path chosen: {}", tinyfiledialogs::open_file_dialog("Open", "./", None).unwrap());
}
