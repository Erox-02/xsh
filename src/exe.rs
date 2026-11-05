use std::process::Command;
use crossterm::{
    event::{self , Event, KeyCode, KeyModifiers},
    execute, terminal,
    cursor,
};
use crate::rex::SyntaxHighlighter;
use crate::conf::Conf;
use std::io::{stdout, Write};
use crate::prmpt::get_prompt;

pub fn run_shell(){
    let conf = Conf::load();
    let _highlighter = SyntaxHighlighter::new("rex");
    
    terminal::enable_raw_mode().unwrap();
    let mut history: Vec<String> = Vec::new();
    let mut history_idx = 0;
    let mut last_exit_code = 0;

    loop {
    
    execute!(stdout(), cursor::MoveToColumn(0)).unwrap();
    print!("xsh> ");

    stdout().flush().unwrap();

    let mut buffer = String::new();
    let mut cursor_pos = 0;

    loop {
        let(prompt_str, prompt_len) = get_prompt(last_exit_code);
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                match key_event.code {
                    KeyCode::Char('c') | KeyCode::Char('d') => {
                        println!("\r");
                        terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                    _=> {}
                }
            }
            match key_event.code {  //bruh wtf i added a dot after match , am i drunk or something?
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
                }
                KeyCode::Right => {
                    if cursor_pos > 0 {cursor_pos += 1;}
                }
                KeyCode::Up => {
                    if history_idx > 0 {
                        history_idx -= 1;
                        buffer = history[history_idx].clone();
                        cursor_pos = buffer.len();
                    }
                }
                KeyCode::Down => {
                    if history_idx < 0 {
                    history_idx += 1;
                    if history_idx == history.len() {
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

            execute!(
                stdout(),
                cursor::MoveToColumn(prompt_len),
                terminal::Clear(terminal::ClearType::UntilNewLine)
            ).unwrap();

            let colored_line = _highlighter.format_line(&buffer);
            print!("{}", colored_line);


            execute!(
                stdout(),
                cursor::MoveToColumn(prompt_len + cursor_pos as u16)
            ).unwrap();
            stdout().flush().unwrap();
            }
        }
        let input = buffer.trim();
        if input.is_empty() {continue;}
        if input == "exit" {break;}
        
        history.push(input.to_string());
        history_idx = history.len();

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let mut args: Vec<String> = parts.map(|s| s.to_string()).collect();

        let mut final_command = command.to_string();
        if let Some(expanded) = conf.resolv_alias(command, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>()) {
            let mut expanded_parts = expanded.split_whitespace();
            final_command = expanded_parts.next().unwrap().to_string(); 
           args = expanded_parts.map(|s| s.to_string()).collect();
        }

        match final_command.as_str() {
            "cd" => {
                let tar = args.join(" ");
                if let Err(e) = std::env::set_current_dir(&tar) {
            eprintln!("cd => {e}");
            last_exit_code = 1;
                } else {
                    last_exit_code = 0;
                }
            }
            _ => {
                if let Err(e) = Command::new(&final_command).args(&args).status() {
                    eprintln!("xshell => {e}");
                    last_exit_code = 127;
                }
            }
        }
    }
    terminal::disable_raw_mode().unwrap();
}
    