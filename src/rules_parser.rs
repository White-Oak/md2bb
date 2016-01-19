use std::io::Read;
use regex::Regex;
use csv::Reader;
const DEFAULT_RULES: &'static str = include_str!("../rules.csv");

pub fn parse_rules(path: &str) -> Vec<(Regex, String)> {
    if let Ok(rdr) = Reader::from_file(path) {
        decode(rdr)
    } else {
        decode(Reader::from_string(DEFAULT_RULES))
    }
}

fn decode<T: Read>(mut rdr: Reader<T>) -> Vec<(Regex, String)> {
    let mut vec = vec![];
    for record in rdr.decode() {
        let (s1, s2): (String, String) = record.unwrap();
        vec.push((Regex::new(&s1).unwrap(), s2));
    }
    vec.push((Regex::new(r"\\n").unwrap(), "\n".to_string()));

    vec
}
