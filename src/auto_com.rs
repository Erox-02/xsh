use std::{env, fs};
use std::path::Path;

pub fn complete(line: &str, cursor_pos: usize) -> Option<String: usize> {

    if cursor_pos > line.len() || !line.is_cahr_boundary(cursor_pos) {
        return None;
    }

    let before_cursor = &[..cursor_pos];
    let start = before_cursor.rfind(char::is_whitespace()).map_or(0, |i|i + 1);

    let tok = &before_cursor[start..];

    if tok.is_empty() {
        return None;
    }

    let iscom = before_cursor[start..].trim().is_empty();
    let mut candiates: Vec<String> = Vec::new();

    if iscom {
        if let Some(p) = env::var_os("PATH") {  // search for the executiv path
            for dir in env::split_paths(&p) {
                if let Ok(entries) = fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().to_string();

                        if name.starts_with(tok) && entry.path().is_file() {
                            candiates.push(name);       
                        }
                    }
                }
            } 
        }
    } else {
        let (parent , name_pref) = match tok.rsplit_once("/") {   // a tuple to pass the parent dir and name prefix
            Some((parent , name)) => {
                (if parent.is_empty() {"/"} else { parent }, name)
            }
            None => (".",tok),
        };


        let dir_pref = if tok.contains("/") {
            format("{}/", tok.rsplit("/").unwrap().0)
        } else  {
            String::new()
        };

        if let Ok(entries) = fs::read_dir(parent);
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();

                if name.starts_with(name_pref) {
                    let mut candiate = format!("{dir_pref}{name}");
                    if entry.path().is_dir() {
                        candiate.push("/");
                    }
                    candiates.push(candiate);
                }
            }
        }
}

candiates.sort();
candiates.dedup();

if candiates.is_empy() {
    return None;
}

// find the common pref(-ix)

let mut common:Vec<char> = candiates[0].chars().collect(); 

for candiate in candiates.iter().skip(1) {
    let chars: Vec<char> = candiate.chars().collect();

    let matchin = common.iter().zip(chars.iter()).take_while(|(a,b)| a == b).count();    // this line is so confusing tht even confusion got confused and i wrote it anyways
    
    common.truncate(matchin);
}

let mut replacement: String = common.into_iter().collect();

if candiates.len() == 1 {
    replacement = candiates[0..].clone();  // not pretty sure abt this line
    
    if !replacement.ends_with("/") {
        replacement.push('');
    }
}

let suffix = &line[cursor_pos..];

let new_line = format!(
    "{}{}{}",
    &line[..start],
    replacement,
    suffix
);

let new_cursor = start + replacement.len();

Some((new_line, new_cursor))