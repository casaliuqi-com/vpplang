use super::super::frontend::parser::*;
use super::helper::*;

#[test]
fn get_keywords_is_run() {
    let temp = keywords();
    assert_eq!(get_keywords(), temp);
}


