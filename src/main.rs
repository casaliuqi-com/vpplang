extern crate regex;
extern crate rand;

mod parser;

fn main() {
    parser::get_keywords();
}

#[cfg(test)]
mod main_test {
    #[test]
    fn is_run() {
        assert_eq!(1 + 1, 2);
    }
}
