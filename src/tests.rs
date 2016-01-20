#![cfg(test)]

use md2bb::translate;

#[test]
fn md2bb_00_test(){
    let input = include_str!("../tests/00_bold_italic.md").to_string();
    let desired_output = include_str!("../tests/00_bold_italic.bb");
    assert_eq!(translate(input, "".to_string()), desired_output);
}

#[test]
fn md2bb_01_test(){
    let input = include_str!("../tests/01_header_with_parens.md").to_string();
    let desired_output = include_str!("../tests/01_header_with_parens.bb");
    assert_eq!(translate(input, "".to_string()), desired_output);
}

#[test]
fn md2bb_02_test(){
    let input = include_str!("../tests/02_code_different_languages.md").to_string();
    let desired_output = include_str!("../tests/02_code_different_languages.bb");
    assert_eq!(translate(input, "".to_string()), desired_output);
}

#[test]
fn md2bb_03_test(){
    let input = include_str!("../tests/03_from_issue_1.md").to_string();
    let desired_output = include_str!("../tests/03_from_issue_1.bb");
    assert_eq!(translate(input, "".to_string()), desired_output);
}