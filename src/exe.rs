use std::process::Command;
use crossterm::{
    event::{Self, Event, KeyCode, Keymodifiers},
    execute, terminal,
    cursor,
};
use crate::rex::SyntaxHighlighter;
use crate::conf::Conf;

pub fn run_shell(){
    let conf = Conf::load();
    let prompt = DefaultPrompt::default();
    let _highlighter = Box::new(SyntaxHighlighter::new("rex"));
    
    terminal::enable_raw_mode().unwrap();
    let mut history: Vec<String> = Vec::new();
    let mut history_index = 0;

    loop {
    
    execute!(stdout(), cursor::MoveToColoumb(0)).unwrap();
    print("xsh>>")

    stdout().flush().unwrap();

    let mut buffer = String::new();
    let mut cursor_pos = 0;

    loop {
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                match.key_event.code {
                    KeyCode::Char('c') | KeyCode::Char('d') ==> {
                        println!("\r");
                        terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                    _=> {}
                }
            }
            match.key_event.code {
                KeyCode::Enter => {
                    println!("\r");
                    break;
                }
                KeyCode::Backspace => {
                    if cursor_pos > 0 {
                        buffer.remove(cursor_pos - 1);
                        cursor_pos -= 1; // lol i accidentally wrote , current_pos = -1; :skull:
                    }
                }
                KeyCode::Left => {
                    if cursor_pos > 0 {cursor_pos -= 1;}
                KeyCode::Right => {
                    if cursor_pos > 0 {cursor_pos += 1;}
                }
                KeyCode::Up => {
                    if history_idx > 0 {
                        history_idx -= 1;
                        buffer = history[history_idx].clone();
                        current_pos = buffer.len();
                    }
                }
                KeyCode::Down => {
                    if history_idx > 0 {
                    history_idx += 1;
                    if history_idx == history.len();
                        buffer.clear();
                    } else {
                        buffer = history[history_idx].clone();
                    }
                    cursor_pos = buffer.len();
                    }
                }
                KeyCode::Char(c) => {
                    buffer.insert(cursor_pos, c);
                    cursor_pos += 1;
                }
                _ => {}
            }

        }
    }
    }
}
