use reedline::{Highlighter, StyledText};
use crossterm::style::{Color, Stylize};
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
        }

impl Highlighter for SyntaxHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> StyledText {
        let mut matches: Vec<_> = self.rules.iter().flat_map(|rule| {
            rule.regex.find_iter(line).map(move |m| (m.start(), m.end(), rule.color_name.clone(), rule.priority, rule.color_name.clone()))
        }).collect();
        if matches.is_empty() {
            return StyledText::new();
        }
        matches.sort_by(|a, b| (a.0, std::cmp::Reverse(a.3)).cmp(&(b.0, std::cmp::Reverse(b.3)))); // this thing gave me an iterator err, i stil dont get it and the nastiest err i found in this whole proj

        let mut styled = StyledText::new();
        let mut pos = 0;
        for (start, end, color_name, _, _) in matches {
            if end <= pos {
                continue;
            }
            if start > pos {
                styled.push((Style::default(), line[pos..start].to_string())); //todo
            }

            let actual_start = start.max(pos);
            styled.push((reedline::Style::default(), colou ::)); //todo
            pos = end;
        }
        if pos < line.len() {
            styled.push((())); //todo
        
        styled
    }
}