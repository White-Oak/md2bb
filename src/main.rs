extern crate regex;
extern crate csv;

pub mod rules_parser;

fn main() {
    use rules_parser::parse_rules;
    let rules = parse_rules("");
    let mut s = include_str!("../text.md").to_string();
    for (re, replace) in rules {
        s = re.replace_all(&s, &replace as &str);
    }
    write_bb("", s);
}
fn write_bb(path: &str, contents: String){
    use std::error::Error;
    use std::io::prelude::*;
    use std::fs::File;
    use std::path::Path;

    let path = Path::new("text.bb");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               Error::description(&why))
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
