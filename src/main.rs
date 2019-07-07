extern crate regex;
extern crate rand;

pub(crate) mod vlang_src_load;
mod frontend;

fn main() {
    frontend::run();
}

#[cfg(test)]
mod main_test {
    #[test]
    fn is_run() {
        assert_eq!(1 + 1, 2);
    }
}
