use crossterm::style::Stylize;
use regex::Regex;
use std::fs;
use std::path::Path;

struct Rule {
    regex: Regex,
    color_name: String,
    priority: usize,
}

pub struct SyntaxHighlighter {
    rules: Vec<Rule>,
}

impl SyntaxHighlighter {
    pub fn new(dir: &str) -> Self {
        let mut rules = Vec::new();
        let path = Path::new(dir);
        println!("Loading syntax rules from: {:?}", path);
        if let Ok(entries) = fs::read_dir(path) {
            let mut paths: Vec<_> = entries.flat_map(|entry| entry.ok().map(|e| e.path())).collect();
            paths.sort();

            for path in paths {
                if path.is_file() && path.extension().map_or(false, |e| e == "rx") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let parsed:Vec<_> = content.lines().filter_map(|l|{
                            let l = l.trim();
                            if l.is_empty() || l.starts_with('#') { return None;}
                                let mut parts = l.split_whitespace();
                                Some((parts.next().unwrap(), parts.next().unwrap_or("")))}).collect();

                                let file_len = parsed.len();
                                for (idx, (pattern, style_name)) in parsed.into_iter().enumerate() {
                                   if let Ok(regex) = Regex::new(pattern) {
                                    let priority = (file_len * 1000).saturating_sub(idx);
                                    rules.push(Rule { regex, color_name: style_name.to_string(), priority });
                                }
                            }
                            }
                        }
                    }
                }
                return Self{ rules };
            }
        

pub fn format_line(&self, line: &str) -> String {
    let mut matches: Vec<_> = self.rules.iter()
        .filter_map(|rule| {
            rule.regex.find(line).map(|m| (m.start(), m.end(), &rule.color_name, rule.priority))
        })
        .collect();

        matches.sort_by(|a, b| a.0.cmp(&b.0));
        let mut result = String::new();
        let mut pos = 0;

        if matches.is_empty() {
            return line.to_string();
        }

        for (start, end , color_name, _priority) in matches {  // HOLY crap i had it in diff order and i was wondering why am i an trait err on deserializer
            if start > pos {
                result.push_str(&line[pos..start]);
            }
            let styled = match color_name.as_str() {
                "red" => line[start..end].red().to_string(),
                "green" => line[start..end].green().to_string(),
                "blue" => line[start..end].blue().to_string(),
                "yellow" => line[start..end].yellow().to_string(),
                "magenta" => line[start..end].magenta().to_string(),
                "cyan" => line[start..end].cyan().to_string(),
                _ => line[start..end].to_string(),
            };
            result.push_str(&styled);
            pos = end;
        }

        if pos < line.len() {
            result.push_str(&line[pos..]);
        }
        result
    }
}