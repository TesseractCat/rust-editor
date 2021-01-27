use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::buffer::*;
use crate::cursor::*;
use serde_json::Value;
use regex::*;

type EditAction = fn(&mut Buffer, usize, Option<&str>) -> ();

lazy_static! {
    static ref ACTIONMAP: HashMap<&'static str, EditAction> = {
        let mut m = HashMap::<&'static str, EditAction>::new();
        m.insert("moveUp", move_up);
        m.insert("moveDown", move_down);
        m.insert("moveLeft", move_left);
        m.insert("moveRight", move_right);
        
        m.insert("viewportUp", viewport_up);
        m.insert("viewportDown", viewport_down);
        
        m.insert("moveBeginning", move_beginning);
        m.insert("moveEnd", move_end);
        
        m.insert("moveFront", move_front);
        m.insert("moveBack", move_back);
        m.insert("moveWord", move_word);
        m.insert("moveWordReverse", move_word_reverse);
        
        m.insert("moveFind", move_find);
        m.insert("moveFindReverse", move_find_reverse);
        m.insert("moveTill", move_till);
        m.insert("moveTillReverse", move_till_reverse);
        
        m.insert("newCursorUp", new_cursor_up);
        m.insert("newCursorDown", new_cursor_down);
        
        m.insert("deleteChar", delete_char);
        m.insert("deleteBack", delete_back);
        m.insert("replaceChar", replace);
        m.insert("toggleCharCase", toggle_char_case);
        m.insert("insertChar", insert_char);
        m.insert("deleteLine", delete_line);
        m.insert("delete", delete);
        
        m.insert("splitLine", split_line);
        m.insert("joinLine", join_line);
        
        m.insert("openUp", open_up);
        m.insert("openDown", open_down);
        
        m.insert("toggleSelection", toggle_selection);
        m.insert("swapSelection", swap_selection);
        
        m.insert("openFile", open_file);
        m
    };
}

pub fn execute_action(buffer: &mut Buffer, command: &str, motion: &Value, key: Option<&str>) {
    if motion != &Value::Null {
        execute_action(buffer, "toggleSelection", &Value::Null, None);
        execute_action(buffer, motion["action"].as_str().unwrap(), &Value::Null, motion["key"].as_str());
        execute_action(buffer, command, &Value::Null, None);
        execute_action(buffer, "toggleSelection", &Value::Null, None);
    } else {
        buffer.cursors.sort_by(|c, d| {
            d.line.cmp(&c.line).then_with(|| d.index.cmp(&c.index))
        });
        //Iterates from end of file to beginning
        for c in 0..buffer.cursors.len() {
            let mut lines_diff: isize = buffer.lines.len() as isize;
            let mut columns_diff: isize = buffer.lines[buffer.cursors[c].line].len() as isize;
            match ACTIONMAP.get(command) {
                Some(x) => x(buffer, c, key),
                None => ()
            }
            lines_diff = (buffer.lines.len() as isize) - lines_diff;
            columns_diff = (buffer.lines[buffer.cursors[c].line].len() as isize) - columns_diff;
            
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
}

fn constrain_cursor(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.lines[buffer.cursors[c].line].chars().count() == 0 {
        buffer.cursors[c].index = 0;
    } else if buffer.cursors[c].index > buffer.lines[buffer.cursors[c].line].chars().count() {
        buffer.cursors[c].index = buffer.lines[buffer.cursors[c].line].chars().count();
    }
}

//Movement
fn move_up(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.cursors[c].line > 0 {
        buffer.cursors[c].line -= 1;
    }
    if buffer.cursors[c].line < buffer.viewport {
        buffer.viewport = buffer.cursors[c].line;
    }
}
fn move_down(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.cursors[c].line < buffer.lines.len() - 1 {
        buffer.cursors[c].line += 1;
    }
    if buffer.cursors[c].line - buffer.viewport > buffer.height {
        buffer.viewport = buffer.cursors[c].line;
    }
}
fn move_left(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    constrain_cursor(buffer, c, None);
    
    if buffer.cursors[c].index > 0 {
        buffer.cursors[c].index -= 1;
    }
}
fn move_right(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.cursors[c].index += 1;
    constrain_cursor(buffer, c, None);
}

fn move_front(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.cursors[c].index = 0;
}
fn move_back(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.lines[buffer.cursors[c].line].len() != 0 {
        buffer.cursors[c].index = buffer.lines[buffer.cursors[c].line].chars().count()-1;
    }
}
fn move_word(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn move_word_reverse(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn move_find(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn move_find_reverse(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn move_till(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn move_till_reverse(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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

fn move_beginning(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if c != 0 { return; }
    
    buffer.cursors[c].line = 0;
    buffer.cursors[c].index = 0;
}
fn move_end(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if c != 0 { return; }
    
    buffer.cursors[c].line = buffer.lines.len().checked_sub(1).unwrap_or(0);
    buffer.cursors[c].index =
        buffer.lines.get(buffer.cursors[c].line)
        .unwrap_or(&("".to_owned())).len()
        .checked_sub(1).unwrap_or(0);
}

//Viewport mutation
fn viewport_up(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if c != 0 { return; }
    
    match buffer.viewport.checked_sub(1) {
        Some(n) => buffer.viewport = n,
        None => (),
    }
}
fn viewport_down(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if c != 0 { return; }
    
    buffer.viewport += 1;
    
    //Constrain to file length
    if buffer.viewport >= buffer.lines.len() {
        buffer.viewport = buffer.lines.len() - 1;
    }
    
    if buffer.cursors.len() == 1 && buffer.cursors[0].line < buffer.viewport {
        buffer.cursors[0].line = buffer.viewport;
    }
}

//Cursor mutation
fn new_cursor_up(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn new_cursor_down(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn toggle_selection(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.cursors[c].range = !buffer.cursors[c].range;
    buffer.cursors[c].line_range = buffer.cursors[c].line;
    buffer.cursors[c].index_range = buffer.cursors[c].index;
}
fn swap_selection(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    let temp_line = buffer.cursors[c].line;
    buffer.cursors[c].line = buffer.cursors[c].line_range;
    buffer.cursors[c].line_range = temp_line;
    
    let temp_index = buffer.cursors[c].index;
    buffer.cursors[c].index = buffer.cursors[c].index_range;
    buffer.cursors[c].index_range = temp_index;
}

//Text mutation
fn delete(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.cursors[c].range {
        buffer.sub_cursor(&buffer.cursors[c].clone(), "");
        if !buffer.cursors[c].is_lower() {
            swap_selection(buffer, c, _key);
        }
    }
}
fn delete_char(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn delete_back(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    if buffer.cursors[c].index > 0 {
        move_left(buffer, c, _key);
        delete_char(buffer, c, _key);
    } else if buffer.cursors[c].line > 0 {
        move_up(buffer, c, _key);
        join_line(buffer, c, _key);
    }
    buffer.dirty = true;
}
fn replace(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn toggle_char_case(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn insert_char(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.add_before_cursor(&buffer.cursors[c].clone(), _key.unwrap());
}

//Line mutation
fn delete_line(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.lines.remove(buffer.cursors[c].line);
    if buffer.cursors[c].line >= buffer.lines.len() {
        move_up(buffer, c, _key);
    }
    if buffer.lines.len() == 0 {
        buffer.lines.push("".to_string());
    }
    buffer.dirty = true;
}
fn split_line(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn join_line(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
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
fn open_up(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.lines.insert(buffer.cursors[c].line, "".to_string());
    buffer.dirty = true;
}
fn open_down(buffer: &mut Buffer, c: usize, _key: Option<&str>) {
    buffer.lines.insert(buffer.cursors[c].line + 1, "".to_string());
    buffer.dirty = true;
}

fn open_file(_buffer: &mut Buffer, _c: usize, _key: Option<&str>) {
    println!("File path chosen: {}", tinyfiledialogs::open_file_dialog("Open", "./", None).unwrap());
}
