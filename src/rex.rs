use reedline::{Highlighter, Color, StyledText};
use nu_ansi_term::Style;
use regex::Regex;
use std::fs;
use std::path::Path;

struct Rule {
    regex: Regex,
    style: Style,
    priority: usize,
}

pub struct SyntaxHighlighter {
    rules: Vec<Rule>,
}

impl SyntaxHighlighter {
    pub fn new(dir: &str) -> Self {
        let mut rules = Vec::new();
        let path = Path::new(dir);
        if let Ok(entries) = fs::read_dir(path) {
            let mut paths: Vec<_> = entries.flat_map(|entry| entry.ok().map(|e| e.path())).collect();
            paths.sort();

            for path in paths {
                if path.is_file() && path.extension().map_or(false, |e| e == "rx") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let parsed:Vec<_> = content.lines().filter_map(|l|{
                            let l = l.trim();
                            if l.is_empty() || l.starts_with('#') { return None;}
                                let mut parts = l.splitn(2, char::is_whitespace);
                                Some((parts.next().unwrap(), parts.next().unwrap_or("").trim()))}).collect();

                                let file_len = parsed.len();
                                for (idx, (pattern, style_name)) in parsed.into_iter().enumerate() {
                                   if let Ok(regex) = Regex::new(pattern) {
                                    let style = match style_name {
                                        "red" => Style::new().fg(Color::Red),
                                        "green" => Style::new().fg(Color::Green),
                                        "blue" => Style::new().fg(Color::Blue),
                                        "yellow" => Style::new().fg(Color::Yellow),
                                        "magenta" => Style::new().fg(Color::Magenta),
                                        "cyan" => Style::new().fg(Color::Cyan),
                                        "white" => Style::new().fg(Color::White),
                                        _ => Style::default(),
                                    };
                                    let priority = (file_len * 1000).saturating_sub(idx);
                                    rules.push(Rule { regex, style, priority });
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
            rule.regex.find_iter(line).map(move |m| (m.start(), m.end(), rule.style.clone(), rule.priority, rule.style.clone()))
        }).collect();
        if matches.is_empty() {
            return StyledText::new();
        }
        matches.sort_by(|a, b| (a.0, std::cmp::Reverse(a.3)).cmp(&(b.0, std::cmp::Reverse(b.3)))); // this thing gave me an iterator err, i stil dont get it and the nastiest err i found in this whole proj

        let mut styled = StyledText::new();
        let mut pos = 0;
        for (start, end, style, _, _) in matches {
            if end <= pos {
                continue;
            }
            if start > pos {
                styled.push((Style::default(), line[pos..start].to_string()));
            }

            let actual_start = start.max(pos);
            styled.push((style, line[actual_start..end].to_string()));
            pos = end;
        }
        if pos < line.len() {
            styled.push((Style::default(), line[pos..].to_string()));
        }
        styled
    }
}