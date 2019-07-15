pub mod parser;

use std::cell::RefCell;
use std::collections::HashMap;

thread_local!{
    static DICT_KEYWORDS: RefCell<HashMap<String, i16>> = RefCell::new(HashMap::new());
}

fn init() {
    let keywords_list = parser::get_keywords();
    DICT_KEYWORDS.with(|dict_k| {
        let mut temp = dict_k.borrow_mut();
        *temp = HashMap::new();
        for (index, keys) in keywords_list.into_iter().enumerate() {
            for key in keys.into_iter() {
                temp.insert(key, -(index as i16 + 1));
            }
        }
    });
}

#[cfg(test)]
pub(crate) fn init_test() {
    init();
}

pub fn run(){
    init();
}
