use aoc23::file;
use regex::Regex;

fn main() {
    let file_contents = file::get_all_lines("input/day1");
    let total_score = file_contents.iter().fold(0, |acc, x| acc + get_single_score(x));
    println!("{total_score}");
}

fn get_single_score(line: &str) -> i32 {
    // println!("'{line}'");
    let r = Regex::new(r"\d").unwrap();
    let matches: Vec<_> = r.find_iter(line).map(|x| x.as_str()).collect();
    // println!("'{:?}'", matches);
    let first = match matches.first() {
        Some(s) => *s,
        None => "0"
    };
    let last = match matches.last() {
        Some(s) => *s,
        None => "0"
    };
    let score: String = first.to_owned() + last;
    // println!("'{score}'");
    score.parse().unwrap_or(0)
}
