use regex::Regex;
use super::super::vlang_src_load;

#[inline]
pub fn get_keywords() -> Vec<Vec<String>>{
    let mut result: Vec<Vec<String>> = Vec::new();
    let keywords_file = vlang_src_load::keywords_v();
    for caps in Regex::new("\\[(\".+\"(,\\s*)?)+\\]").unwrap().captures_iter(keywords_file) {
        let temp = caps.get(1).unwrap().as_str();
        let mut keyword_list: Vec<String> = Vec::new();
        for keywords in Regex::new("[^\",\\s]+").unwrap().captures_iter(temp) {
            keyword_list.push(keywords.get(0).unwrap().as_str().to_string());
        }
        result.push(keyword_list);
    }
    result
}
