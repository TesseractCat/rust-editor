//#![windows_subsystem = "windows"]
use web_view::*;
use std::{fs, str};
use serde_json::{Value, json};
use regex::{Captures, Regex};
use rust_embed::RustEmbed;

mod cursor;
mod buffer;
mod actions;
mod console;
use buffer::*;

#[derive(RustEmbed)]
#[folder = "src/assets"]
struct Asset;

fn get_asset_str(asset: &str) -> String {
    str::from_utf8(Asset::get(asset).unwrap().to_mut()).unwrap().to_string()
}

fn main() {
    //HIDPI Scaling fix
    #[cfg(target_os = "windows")]
    unsafe {
        winapi::um::shellscalingapi::SetProcessDpiAwareness(2);
    }
    
    //Load html
    let mut html_content: String = include_str!("assets/index.html").to_string();
    html_content = html_content.replace("{inline-assets}",
        &vec![
        inline_style(include_str!("assets/katex/katex.css")),
        inline_script(include_str!("assets/katex/katex.min.js"))
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
    
    let main_buffer = Buffer::new();
    
    let mut webview = web_view::builder()
        .title("Frog")
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
        redraw(webview);
    } else if response["type"] == "debug" {
        println!("{}", response["message"]);
    } else if response["type"] == "edit" {
        actions::execute_action(
            state, response["action"].as_str().unwrap(), &response["motion"], response["key"].as_str());
        redraw(webview);
    } else if response["type"] == "console" {
        let console_response = console::execute_command(state, response["command"].as_str().unwrap_or_default());
        webview.eval(&console_response);
        redraw(webview);
    }
}

fn redraw(webview: &mut WebView<Buffer>) {
    let state: &mut Buffer = webview.user_data_mut();
    
    state.cursors.sort_by(|c, d| {
        d.line.cmp(&c.line).then_with(|| d.index.cmp(&c.index))
    });
    let buffer_data = json!({
        "path": state.path.as_ref().unwrap_or(&"".to_string()),
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
