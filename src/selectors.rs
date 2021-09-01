use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader};
use std::path::Path;
use std::process;

use csv::Writer;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
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

pub fn search_target(query: &str, target: &str, logging: bool) {
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
                    Err(e) => println!("Selector parsing error: {}", e),
                }
            }
        });
    let tally = tally_matches(match_list.clone());

    if logging {
        if let Err(err) = log_results(match_list.clone(), &tally) {
            println!("{}", err);
            process::exit(1);
        }
    }
}

fn parse_file_selectors(path: &Path) -> Result<Vec<(usize, String)>, Box<dyn Error>> {
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
        *match_tally.entry(res.selector).or_insert(0) += 1;
    }

    match_tally
}

fn log_results(
    match_list: Vec<SelectorMatchResult>,
    tally: &HashMap<String, usize>,
) -> Result<(), Box<dyn Error>> {
    println!(
        "--Logging Results--\nList--\n{:#?}\nTally--\n{:#?}",
        match_list, tally
    );

    let csv_file = "selector_search_results.csv";
    // let mut wrtr = Writer::from_writer(stdout());
    let mut wrtr = Writer::from_path(csv_file)?;
    wrtr.write_record(&["Selector", "Count", "Locations"])?;
    for (selector, count) in tally.into_iter() {
        wrtr.write_record(&[selector, &count.to_string(), ""])?;

        for selector_match in match_list.iter() {
            if &selector_match.selector == selector {
                let location = format!(
                    "Line {}: {}",
                    selector_match.line_num, selector_match.file_name
                );
                wrtr.write_record(&["", "", location.as_str()])?;
            }
        }
    }

    wrtr.flush()?;
    Ok(())
}
