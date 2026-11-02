use std::io::{self, Write};
use std::process::Command;

fn main(){
    loop{
        print!("xsh > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match command{
            "cd" => {
                let tar = args.get(0).copied().unwrap_or("~");
                let tar = if tar == "~" {
                    std::env::var("HOME").unwrap_or_else(|_| String::from("/"))
                } else {
                    tar.to_string()
                }; 
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