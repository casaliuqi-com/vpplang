use regex::Regex;
use super::super::vlang_src_load;


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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_keywords_is_run(){
        let temp: Vec<Vec<String>> = vec![
            vec!["fn", "func", "function"], 
            vec!["sub", "subroutine"], 
            vec!["impl", "implements"], 
            vec!["interface"], 
            vec!["enum"], 
            vec!["struct"], 
            vec!["import"], 
            vec!["use", "using"], 
            vec!["as"], 
            vec!["macro"], 
            vec!["for"], 
            vec!["in"], 
            vec!["mod", "module"], 
            vec!["mut", "mutable"], 
            vec!["return"], 
            vec!["let"]
        ].into_iter()
         .map(|x| 
            x.into_iter()
             .map(|y| y.to_string())
             .collect()
         ).collect();
        assert_eq!(get_keywords(), temp);
    }
}
