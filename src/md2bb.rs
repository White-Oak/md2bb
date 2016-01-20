use rules_parser::parse_rules;

pub fn translate(mut input: String, rules_file: String) -> String{
    let rules = parse_rules(&rules_file);
    println!("Rules loaded.");
    for (re, replace) in rules {
        input = re.replace_all(&input, &replace as &str);
    }
    input
}
