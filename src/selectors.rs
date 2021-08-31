use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

use walkdir::WalkDir;

#[derive(Debug)]
struct SelectorMatchResult {
    file_name: String,
    line_num: usize,
    selector: String,
}

impl SelectorMatchResult {
    fn new(file: String, linum: usize, selector: String) -> Self {
        Self {
            file_name: file,
            line_num: linum,
            selector,
        }
    }
}

pub fn search_target(query: &str, target: &str) {
    println!("Searching selectors in target: {}", target);
    let path = Path::new(target);
    let mut match_list: Vec<SelectorMatchResult> = Vec::new();
    WalkDir::new(path)
        .into_iter()
        .filter_map(|v| v.ok())
        .for_each(|x| {
            if x.path().is_file() {
                match parse_file_selectors(x.path()) {
                    Ok(v) => {
                        for (line, s) in v {
                            if s.contains(query) {
                                let selector = s.replace("{", "");
                                let res = SelectorMatchResult::new(
                                    x.path().to_string_lossy().to_string(),
                                    line,
                                    selector.trim_end().to_string(),
                                );
                                match_list.push(res);
                            }
                        }
                    }
                    Err(e) => print!("Selector parsing error: {}", e),
                }
            }
        });
    let tally = tally_matches(match_list);
    println!("Tally - {:#?}", tally);
}

fn parse_file_selectors(path: &Path) -> Result<Vec<(usize, String)>, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut selectors = vec![];
    for (num, line) in buffered.lines().enumerate() {
        match line {
            Ok(line) => {
                if line.contains("{") {
                    let linum = num + 1;
                    selectors.push((linum, line))
                }
            }
            Err(_) => println!("No line found"),
        }
    }

    Ok(selectors)
}

fn tally_matches(match_list: Vec<SelectorMatchResult>) -> HashMap<String, usize> {
    let mut match_tally = HashMap::new();
    println!("Matches - {:#?}", match_list);
    for res in match_list {
        let count = match_tally.entry(res.selector).or_insert(0);
        *count += 1;
    }

    match_tally
}
