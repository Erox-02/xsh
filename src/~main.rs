use reedline::{DefaultPrompt, Reedline, Signal};
use std::process::Command;

fn main(){
    
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
                let args: Vec<&str> = parts.collect();
                
                match command {
                    "cd" => {
                        let tar = args.join(" ");
                        if let Err(e) = std::env::set_current_dir(&tar) {
                            eprintln!("cd => {e}");
                        }
                    }
                    _ => {
                        if let Err(e) = Command::new(command).args(&args).status() {
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
