extern crate regex;
extern crate csv;
extern crate getopts;

use std::fs::File;
use std::env::args;
use std::io::Read;
use getopts::Options;

pub mod rules_parser;

fn main() {
    use rules_parser::parse_rules;
    let (mut input, output, rules_file) = get_args();
    let rules = parse_rules(&rules_file);
    for (re, replace) in rules {
        input = re.replace_all(&input, &replace as &str);
    }
    write_bb(&output, input);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn get_args() -> (String, String, String){
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("r", "rules", "set rules for translation", "FILE");
    opts.optopt("o", "output", "set output file for translation; default -- text.bb", "FILE");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        panic!();
    }

    let rules = if matches.opt_present("r") {
        matches.opt_str("r").unwrap()
    } else {
        "rules.csv".into()
    };

    let file = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        "text.md".into()
    };

    let output = if matches.opt_present("o") {
        matches.opt_str("o").unwrap()
    } else {
        "text.bb".into()
    };

    let mut contents = String::new();
    let mut f = File::open(&file).unwrap();
    let _ = f.read_to_string(&mut contents);

    (contents, output, rules)
}

fn write_bb(path: &str, contents: String){
    use std::error::Error;
    use std::io::prelude::*;
    use std::fs::File;
    use std::path::Path;

    let path = Path::new(path);
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
