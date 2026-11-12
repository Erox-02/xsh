use std::process::Command;
use crossterm::{
    event::{self , Event, KeyCode, KeyModifiers},
    execute, terminal,
    cursor,
    style
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
    
    let(prompt_str, prompt_len) = get_prompt(last_exit_code);
    execute!(stdout(), cursor::MoveToColumn(0)).unwrap();
    print!("{prompt_str}");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    let mut cursor_pos = 0;

    loop {
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
            match key_event.code {  //bruh wtf i added a dot after match , am i drunk or something
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
                    if history_idx < history.len() {
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

            // let (start_col, start_row) = cursor::position().unwrap_or((0,0));
            
            // let old_coloured = _highlighter.format_line(&buffer);
            // let old_vis_bytes = strip_ansi_escapes::strip(&old_coloured);
            // let old_vis_len = String::from_utf8_lossy(&old_vis_bytes).len() as u16;

            execute!(
                stdout(),
                cursor::MoveToColumn(0),
                terminal::Clear(terminal::ClearType::CurrentLine)  // this unitnewline is the cause of the prob , definately
            ).unwrap();
            
            print!("{prompt_str}");
            
            let colored_line = _highlighter.format_line(&buffer);
            let mut visible_len = 0;
            let mut cursor_col =  prompt_len;

            for (colour, text) in colored_line {    // its a mess with my normal british colour and the crates color american english lol
                if let Some(x) = colour {
                    execute!(stdout(), crossterm::style::SetForegroundColor(x)).unwrap();
                } else {
                    execute!(stdout(), crossterm::style::ResetColor).unwrap();
                }
                print!("{text}");

                if visible_len + text.len() <= cursor_pos {
                    cursor_col += text.len() as u16;
                } else if visible_len < cursor_pos {
                    let offset = cursor_pos - visible_len; 
                    cursor_col += offset as u16;
                }
                visible_len += text.len();
            }

            execute!(stdout(), crossterm::style::ResetColor).unwrap();

            execute!(
                stdout(),
                cursor::MoveToColumn(cursor_col)
            ).unwrap();
            //  ======================================= legacy code ========================================================================================================================================
            // let buffer_before_cursor = &buffer[..cursor_pos];
            // let styled_before_cursor = _highlighter.format_line(buffer_before_cursor);
            // let visible_before_cursor =  strip_ansi_escapes::strip(&styled_before_cursor);
            // let visible_before_len = String::from_utf8_lossy(&visible_before_cursor).len() as u16;  

            // execute!(
                // stdout(),
                // cursor::MoveToColumn(prompt_len + visible_before_len)
            // ).unwrap();
            // stdout().flush().unwrap();

            // let target_col = prompt_len + visible_before_len;
            // let (term_wid, _) = terminal::size().unwrap_or((80,25)); // size Returns the terminal size (columns, rows) => frm `https://docs.rs/crossterm/0.29.0/crossterm/terminal/index.html` 
            //  i didnt pass the default 80x24 because it is too basic instead i used 80x25 lol its legacy too but i js didnt wanna use the old unix way here
            
            // if target_col >= term_wid {
            //     let rows_down = target_col / term_wid;
            //     let final_col = target_col % term_wid;
           
                // execute!(
                //     stdout(),
                //     cursor::MoveUp(rows_down),
                //     cursor::MoveToColumn(final_col)
                // ).unwrap_or_default();
                // } else {
                // execute!(
                //     stdout(),
                //     cursor::MoveToColumn(target_col),
                // ).unwrap_or_default();
                // }
            // ======================================= legacy code ========================================================================================================================================
                stdout().flush().unwrap();
            }
        }
        let input = buffer.trim();
        if input.is_empty() {continue;}
        if input == "exit" {break;}
        
        history.push(input.to_string());
        history_idx = history.len();

        let parts = match shlex::split(input) {
            Some(p) => p,
            None => {
                eprintln!("Unfinished quote baka");
                last_exit_code = 1;
                continue;
            }
        };
        if parts.is_empty() {
            continue;
        }

        let command = &parts[0];
        let mut args: Vec<String> = Vec::new(); 

        for arg in &parts[1..] {
            if arg.contains("*") || arg.contains("?") {
                match glob::glob(arg) {
                    Ok(paths) => {
                        let mut found = false ;
                        for entry in paths.flatten() {
                            args.push(entry.to_string_lossy().to_string());
                            found = true;
                        }
                        if !found {
                            args.push(arg.clone());  // if it doesnt match , keep th literal 
                        }
                    }
                    Err(_) => args.push(arg.clone()),
                }
            } else {
                args.push(arg.clone()); // dont use wildcard , push the clone directly
            }
        }

        let mut final_command = command.to_string();
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        if let Some(expanded) = conf.resolv_alias(command.as_str(), &args_refs) {
            if let Some(alias_parts) = shlex::split(&expanded) {
                if !alias_parts.is_empty() {
                    final_command = alias_parts[0].clone();
                
            // final_command = expanded_parts.next().unwrap().to_string();

            // let mut alias_args: Vec<String> = expanded_parts.map(|s| s.to_string()).collect();
            let mut alias_args: Vec<String> = alias_parts[1..].to_vec();
            alias_args.extend(args); 
           args = alias_args;
        // TODO ==> FIX alias + multi word quoted text fix , Swapiness = 50
        
        }
    }
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
    