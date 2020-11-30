use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::buffer::*;
use crate::cursor::*;
use serde_json::Value;
use tinyfiledialogs::open_file_dialog;

type NormalCommand = fn(&mut Buffer, Option<&Value>, Option<&str>) -> ();

lazy_static! {
    static ref COMMANDMAP: HashMap<&'static str, NormalCommand> = {
        let mut m = HashMap::<&'static str, NormalCommand>::new();
        m.insert("moveUp", move_up);
        m.insert("moveDown", move_down);
        m.insert("moveLeft", move_left);
        m.insert("moveRight", move_right);
        m.insert("moveFront", move_front);
        m.insert("moveBack", move_back);
        m.insert("moveFind", move_find);
        m.insert("moveFindBackward", move_find_backward);
        
        m.insert("deleteChar", delete_char);
        m.insert("deleteBack", delete_back);
        m.insert("replaceChar", replace_char);
        m.insert("toggleCharCase", toggle_char_case);
        m.insert("insertChar", insert_char);
        m.insert("deleteLine", delete_line);
        
        m.insert("splitLine", split_line);
        m.insert("joinLine", join_line);
        
        m.insert("openUp", open_up);
        m.insert("openDown", open_down);
        
        m.insert("toggleSelection", toggle_selection);
        
        m.insert("openFile", open_file);
        m
    };
}

pub fn execute_command(buffer: &mut Buffer, command: &str, motion: Option<&Value>, key: Option<&str>) {
    match COMMANDMAP.get(command) {
        Some(x) => x(buffer, motion, key),
        None => ()
    }
}

fn constrain_cursor(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    if buffer.lines[buffer.cursors[0].line].chars().count() == 0 {
        buffer.cursors[0].index = 0;
    } else if buffer.cursors[0].index > buffer.lines[buffer.cursors[0].line].chars().count() {
        buffer.cursors[0].index = buffer.lines[buffer.cursors[0].line].chars().count();
    }
}
fn move_up(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    if buffer.cursors[0].line > 0 {
        buffer.cursors[0].line -= 1;
    }
}
fn move_down(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    if buffer.cursors[0].line < buffer.lines.len() - 1 {
        buffer.cursors[0].line += 1;
    }
}
fn move_left(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    constrain_cursor(buffer, None, None);
    
    if buffer.cursors[0].index > 0 {
        buffer.cursors[0].index -= 1;
    }
}
fn move_right(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    buffer.cursors[0].index += 1;
    constrain_cursor(buffer, None, None);
}
fn move_front(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    buffer.cursors[0].index = 0;
}
fn move_back(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    if buffer.lines[buffer.cursors[0].line].len() != 0 {
        buffer.cursors[0].index = buffer.lines[buffer.cursors[0].line].chars().count()-1;
    }
}
fn move_find(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    let mut line_iter = buffer.lines[buffer.cursors[0].line].chars();
    let key_as_char: char = _key.unwrap().chars().next().unwrap();
    let mut i = buffer.cursors[0].index + 1;
    
    line_iter.nth(buffer.cursors[0].index);
    while let Some(next_char) = line_iter.next() {
        if next_char == key_as_char {
            buffer.cursors[0].index = i;
            return;
        }
        i += 1;
    }
}
fn move_find_backward(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    let mut line_iter = buffer.lines[buffer.cursors[0].line].chars();
    let key_as_char: char = _key.unwrap().chars().next().unwrap();
    let mut i = buffer.cursors[0].index + 1;
    
    line_iter.nth(buffer.cursors[0].index);
    while let Some(next_char) = line_iter.next() {
        if next_char == key_as_char {
            buffer.cursors[0].index = i;
            return;
        }
        i += 1;
    }
}

fn toggle_selection(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    buffer.cursors[0].pair = Some(Box::new(Cursor {
        line: buffer.cursors[0].line,
        index: buffer.cursors[0].index,
        pair: None,
    }));
}

fn delete_char(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    constrain_cursor(buffer, None, None);
    if buffer.lines[buffer.cursors[0].line].chars().count() > 0
        && buffer.cursors[0].index < buffer.lines[buffer.cursors[0].line].chars().count() {
        buffer.lines[buffer.cursors[0].line].replace_range(
            buffer.cursors[0].index..=buffer.cursors[0].index, "");
    } else {
        move_left(buffer, _motion, _key);
    }
}
fn delete_back(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    if buffer.cursors[0].index > 0 {
        move_left(buffer, _motion, _key);
        delete_char(buffer, _motion, _key);
    } else if buffer.cursors[0].line > 0 {
        move_up(buffer, _motion, _key);
        join_line(buffer, _motion, _key);
    }
}
fn replace_char(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    constrain_cursor(buffer, None, None);
    if buffer.lines[buffer.cursors[0].line].chars().count() > 0
        && buffer.cursors[0].index < buffer.lines[buffer.cursors[0].line].chars().count() {
        buffer.lines[buffer.cursors[0].line].replace_range(
            buffer.cursors[0].index..=buffer.cursors[0].index, _key.unwrap());
    } else {
        buffer.lines[buffer.cursors[0].line].push_str(_key.unwrap());
    }
}
fn toggle_char_case(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    constrain_cursor(buffer, None, None);
    if buffer.lines[buffer.cursors[0].line].chars().count() > 0
        && buffer.cursors[0].index < buffer.lines[buffer.cursors[0].line].chars().count() {
        let mut line = buffer.lines[buffer.cursors[0].line].clone();
        if line.chars().nth(buffer.cursors[0].index).unwrap().is_ascii_uppercase() {
            line = line.to_ascii_lowercase();
        } else {
            line = line.to_ascii_uppercase();
        }
        buffer.lines[buffer.cursors[0].line].replace_range(
            buffer.cursors[0].index..=buffer.cursors[0].index,
            &line[buffer.cursors[0].index..=buffer.cursors[0].index]);
    }
}
fn insert_char(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    constrain_cursor(buffer, None, None);
    if buffer.lines[buffer.cursors[0].line].chars().count() > 0 {
        if buffer.cursors[0].index >= buffer.lines[buffer.cursors[0].line].chars().count() {
            buffer.lines[buffer.cursors[0].line].push_str(
                _key.unwrap());
        } else {
            buffer.lines[buffer.cursors[0].line].insert_str(
                buffer.cursors[0].index, _key.unwrap());
        }
    } else {
        buffer.lines[buffer.cursors[0].line] = _key.unwrap().to_string();
    }
}

fn delete_line(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    buffer.lines.remove(buffer.cursors[0].line);
    if buffer.cursors[0].line >= buffer.lines.len() {
        move_up(buffer, _motion, _key);
    }
    if buffer.lines.len() == 0 {
        buffer.lines.push("".to_string());
    }
}

fn split_line(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    if buffer.lines[buffer.cursors[0].line].len() > 0 {
        let front_split = buffer.lines[buffer.cursors[0].line][..buffer.cursors[0].index].to_string();
        let back_split = buffer.lines[buffer.cursors[0].line][buffer.cursors[0].index..].to_string();
        buffer.lines[buffer.cursors[0].line] = front_split;
        buffer.lines.insert(buffer.cursors[0].line + 1, back_split);
    } else {
        open_down(buffer, _motion, _key);
    }
}
fn join_line(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    if buffer.cursors[0].line < buffer.lines.len() - 1 {
        let next_line = buffer.lines[buffer.cursors[0].line+1].clone();
        let line_len = buffer.lines[buffer.cursors[0].line].len();
        buffer.lines[buffer.cursors[0].line].push_str(&next_line);
        buffer.lines.remove(buffer.cursors[0].line+1);
        buffer.cursors[0].index = line_len;
    }
    constrain_cursor(buffer, None, None);
}

fn open_up(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    buffer.lines.insert(buffer.cursors[0].line, "".to_string());
}
fn open_down(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    buffer.lines.insert(buffer.cursors[0].line + 1, "".to_string());
}

fn open_file(buffer: &mut Buffer, _motion: Option<&Value>, _key: Option<&str>) {
    println!("File path chosen: {}", tinyfiledialogs::open_file_dialog("Open", "./", None).unwrap());
}
