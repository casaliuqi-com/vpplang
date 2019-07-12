extern crate regex;
extern crate rand;

pub(crate) mod vlang_src_load;
mod frontend;

#[cfg(test)]
mod test;

fn main() {
    frontend::run();
}
