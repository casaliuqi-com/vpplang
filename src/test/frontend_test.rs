use super::super::frontend::parser::*;
use super::helper::*;

#[test]
fn get_keywords_is_run() {
    let temp = keywords();
    assert_eq!(get_keywords(), temp);
}

#[test]
fn init_is_run(){
    use frontend::init_test;
    init_test();
    // TODO
}
