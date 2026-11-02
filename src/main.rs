use std::io::{self, Write};
use reedline::{DefaultPrompt, Reedline, Signal};
use std::process::Command;

fn main(){
    
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();
    
    loop{
        let sig = line_editor.read_line(&prompt).expect("Error reading line").unwrap_or(Signal::CtrlD);
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
                let args: Vec<&str> = parts.collect();
                
                match command {
                    "cd" =>
                }
                if let Err(e) = std::env::set_current_dir(&tar) {
                    eprintln!("cd => {e}");
                }
            }
            _=> {
                if let Err(e) = Command::new(command).args(&args).spawn() {
                    eprintln!("Error executing command: {e}");
               }
          }
        }   
    }
}