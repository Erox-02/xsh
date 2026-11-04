use reedline::{DefaultPrompt, Reedline, Signal};
use std::process::Command;
use crate::rex::SyntaxHighlighter;
use crate::conf::Conf;

pub fn run_shell(){
    let conf = Conf::load();
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();
    
    loop{
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                let input = buffer.trim();
                if input.is_empty() {
                    continue;
                };
                if input == "exit" {
                    break;
                }
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
                        }
                    }
                    _ => {
                        if let Err(e) = Command::new(&final_command).args(&args).status() {
                            eprintln!("xshell => {e}");
                        }
                    }
                }
            }
            Signal::CtrlC | Signal::CtrlD => { 
                println!("Manual exit");
                break;
            }
                _ => {}
        }
    }
}
