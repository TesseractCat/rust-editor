use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::buffer::*;
use crate::cursor::*;
use regex::Regex;

use std::process;

type ConsoleCommand = fn(&mut Buffer, Vec<&str>) -> Option<String>;

lazy_static! {
    static ref COMMANDMAP: HashMap<&'static str, ConsoleCommand> = {
        let mut m = HashMap::<&'static str, ConsoleCommand>::new();
        m.insert("quit", quit);
        m.insert("q", quit);
        
        m.insert("edit", edit);
        m.insert("e", edit);
        
        m.insert("write", write);
        m.insert("w", write);
        
        m.insert("x", extract);
        
        m.insert("cc", clear_cursors);
        m
    };
}

pub fn execute_command(buffer: &mut Buffer, command_string: &str) -> String {
    if command_string.chars().next() != Some(':') || command_string.len() < 2 {
        return "".to_owned();
    }
    
    let split_command_string: Vec<&str> = command_string[1..].split(' ').collect();
    let command_slice: &str = split_command_string[0];
    let args_slice: Vec<&str> = split_command_string.get(1..).unwrap_or_default().to_vec();

    match COMMANDMAP.get(command_slice) {
        Some(x) => {
            return x(buffer, args_slice).unwrap_or("".to_owned());
        },
        None => ()
    }
    
    "".to_owned()
}

fn quit(_buffer: &mut Buffer, _args: Vec<&str>) -> Option<String> {
    process::exit(0);
}

fn edit(buffer: &mut Buffer, args: Vec<&str>) -> Option<String> {
    let path = args.join(" ");
    buffer.load_path(&path);
    None
}

fn write(buffer: &mut Buffer, _args: Vec<&str>) -> Option<String> {
    buffer.write();
    None
}

fn clear_cursors(buffer: &mut Buffer, _args: Vec<&str>) -> Option<String> {
    buffer.cursors.truncate(1);
    None
}

fn extract(buffer: &mut Buffer, args: Vec<&str>) -> Option<String> {
    let search: String = args.join("");
    let re: Regex = Regex::new(&search).unwrap();
    let mut new_cursors: Vec<Cursor> = vec![];
    
    let mut capture_ranges: Vec<(usize, usize)> = vec![];

    for cap in re.captures_iter(buffer.get_as_string()) {
        capture_ranges.push(
            (cap.iter().next().unwrap().unwrap().end() - 1, cap.iter().next().unwrap().unwrap().start()));
    }
    for r in capture_ranges {
        let lower = buffer.string_idx_to_cursor_idx(r.0);
        let higher = buffer.string_idx_to_cursor_idx(r.1);
        new_cursors.push(Cursor {
            line:lower.0,
            index:lower.1,
            line_range:higher.0,
            index_range:higher.1,
            range:true,
        });
   
    }
    if new_cursors.len() > 0 {
        buffer.cursors.truncate(0);
        buffer.cursors.extend(new_cursors.iter().map(|x| Box::new(x.clone())));
        return Some("changeMode(\"selection\")".to_owned());
    }
    None
}
