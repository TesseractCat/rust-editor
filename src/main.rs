//#![windows_subsystem = "windows"]

use web_view::*;
use std::fs;
use serde_json::{Value, json};
use std::process;

mod cursor;
mod buffer;
mod commands;
use buffer::*;

fn main() {
    let mut html_content = fs::read_to_string("assets/index.html").expect("Unable to read file!");
    html_content = html_content.replace("{inline-assets}",
        &vec![
        inline_style(include_str!("assets/katex/katex.min.css")),
        inline_script(include_str!("assets/katex/katex.min.js"))
        ].join(""));
    
    let main_buffer = Buffer::new();
    
    let mut webview = web_view::builder()
        .title("Rust Editor")
        .content(Content::Html(html_content))
        .size(500, 500)
        .resizable(true)
        .visible(false)
        .user_data(main_buffer)
        .invoke_handler(|_webview, _arg| {
            handler(_webview, _arg);
            Ok(())
        })
        .build()
        .unwrap();
    
    redraw(&mut webview);
    
    webview.run().ok();
}

fn handler(webview: &mut WebView<Buffer>, arg: &str) {
    println!("{}", arg);
    
    let state: &mut Buffer = webview.user_data_mut();
    let response: Value = serde_json::from_str(arg).unwrap();
    
    if response["type"] == "init" {
        webview.set_visible(true);
    } else if response["type"] == "debug" {
        println!("{}", response["message"]);
    } else if response["type"] == "edit" {
        commands::execute_command(
            state, response["command"].as_str().unwrap(), response.pointer("motion"), response["key"].as_str());
    } else if response["type"] == "console" {
        if response["command"] == ":q" {
            process::exit(0);
        }
    }
    
    redraw(webview);
}

fn redraw(webview: &mut WebView<Buffer>) {
    let state: &mut Buffer = webview.user_data_mut();
    
    state.cursors.sort_by(|c, d| {
        d.line.cmp(&c.line).then_with(|| d.index.cmp(&c.index))
    });
    
    let buffer_data = json!({
        "lines": state.lines,
        "cursors": state.cursors
    });
    webview.eval(&format!("populateBuffer({})", buffer_data.to_string())).ok();
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
