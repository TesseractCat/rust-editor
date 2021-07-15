//#![windows_subsystem = "windows"]
use web_view::*;
use std::{cmp, str, env, fs};
use serde_json::{Value, json};
use regex::{Captures, Regex};
use rust_embed::RustEmbed;

mod cursor;
mod buffer;
mod actions;
mod console;
mod keymap;

use buffer::*;
use keymap::*;

#[derive(RustEmbed)]
#[folder = "src/assets"]
struct Asset;

fn get_asset_str(asset: &str) -> String {
    str::from_utf8(Asset::get(asset).unwrap().to_mut()).unwrap().to_string()
}

struct State {
    pub buffer: Buffer,
    pub key_executor: KeyExecutor,
}

fn main() {
    //Command line arguments
    let args: Vec<String> = env::args().collect();
    
    //HIDPI Scaling fix
    #[cfg(target_os = "windows")]
    unsafe {
        winapi::um::shellscalingapi::SetProcessDpiAwareness(2);
    }
    
    //Load html
    let mut html_content: String = include_str!("assets/index.html").to_string();
    html_content = html_content.replace("{inline-assets}",
        &vec![
        inline_style(include_str!("assets/index.css")),
        inline_script(include_str!("assets/index.js")),
        
        inline_style(include_str!("assets/katex/katex.css")),
        inline_script(include_str!("assets/katex/katex.min.js")),
        ].join(""));
    
    //Load base64 fonts
    //TODO: Do this as a preprocess step
    let font_re = Regex::new("fonts/(.+?\\.woff2)\\)").unwrap();
    html_content = font_re.replace_all(&html_content, |caps: &Captures| {
        let mut new_path = "./fonts/".to_string();
        new_path.push_str(caps.get(1).map_or("", |m| m.as_str()));
        new_path.push_str(".b64");
        return "'data:application/font-woff2;charset=utf-8;base64,".to_string() +
            get_asset_str(&new_path).replace("\n","").as_str() + "')";
    }).to_string();
    
    //Setup program state
    let mut main_state: State = State {
        buffer: Buffer::new(),
        key_executor: KeyExecutor::new()
    };
    
    if args.len() >= 2 {
        main_state.buffer.load_path(&args[1]);
    }
    
    let webview = web_view::builder()
        .title("Frog")
        .content(Content::Html(html_content))
        .size(800, 500)
        .resizable(true)
        .visible(false)
        .debug(true)
        .user_data(main_state)
        .invoke_handler(|_webview, _arg| {
            handler(_webview, _arg);
            Ok(())
        })
        .build()
        .unwrap();
    
    webview.run().ok();
}

fn handler(webview: &mut WebView<State>, arg: &str) {
    //println!("{}", arg);
    
    let state: &mut State = webview.user_data_mut();
    let buffer: &mut Buffer = &mut state.buffer;
    let response: Value = serde_json::from_str(arg).unwrap();
    
    if response["type"] == "init" {
        //Webview loaded
        webview.set_visible(true);
        redraw(webview, None);
    } else if response["type"] == "debug" {
        //Debug message from webview
        println!("{}", response["message"]);
    } else if response["type"] == "keyevent" {
        //Debug message from webview
        println!("Keyevent! Event: {} | Key: {}", response["event"], response["key"]);
        state.key_executor.execute_key(response["key"].as_str().unwrap_or_default(), buffer);
        redraw(webview, None);
    } else if response["type"] == "console" {
        //Console command
        let console_response = console::execute_command(buffer, response["command"].as_str().unwrap_or_default());
        webview.eval(&console_response).ok();
        redraw(webview, None);
    }
}

fn redraw(webview: &mut WebView<State>, line_changes: Option<Vec<(usize, String)>>) {
    let state: &mut State = webview.user_data_mut();
    let buffer: &mut Buffer = &mut state.buffer;
    
    match line_changes {
        Some(changes) => {
            //Sort cursors from beginning of file to end
            //buffer.cursors.sort_by(|c, d| {
            //    d.line.cmp(&c.line).then_with(|| d.index.cmp(&c.index))
            //});
            //let buffer_data = json!({
            //    "changed": changes
            //});
            buffer.cursors.sort_by(|c, d| {
                d.line.cmp(&c.line).then_with(|| d.index.cmp(&c.index))
            });
            let buffer_data = json!({
                "path": buffer.path.as_ref().unwrap_or(&"".to_string()),
                "viewport":0,
                "lines": buffer.lines,
                "cursors": buffer.cursors
            });
            webview.eval(&format!("populateBuffer({})", buffer_data.to_string())).ok();
        },
        None => {
            //Sort cursors from beginning of file to end
            buffer.cursors.sort_by(|c, d| {
                d.line.cmp(&c.line).then_with(|| d.index.cmp(&c.index))
            });
            let buffer_data = json!({
                "path": buffer.path.as_ref().unwrap_or(&"".to_string()),
                "viewport":0,
                "lines": buffer.lines,
                "cursors": buffer.cursors
            });
            webview.eval(&format!("populateBuffer({})", buffer_data.to_string())).ok();
        }
    }
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
