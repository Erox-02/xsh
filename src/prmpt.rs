use std::env;

pub fn get_prompt(exit_code: i32) -> (String, u16) {
    let cwd = env::current_dir().unwrap_or_default();
    let home = env::var("HOME").unwrap_or_default();
    let mut dir_str = cwd.to_string_lossy().to_string();

    if !home.is_empty() && dir_str.starts_with(&home) {
        dir_str = format!("~{}", &dir_str[home.len()..]);
    }

    let prompt_symbol = if exit_code == 0 {">"} else {"!"};
    let prompt_str = format!("{} {}", dir_str, prompt_symbol);
    let len = prompt_str.len() as u16;
    (prompt_str, len)
}