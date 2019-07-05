use regex::Regex;

pub fn get_keywords() -> Vec<Vec<String>>{
    let mut result: Vec<Vec<String>> = Vec::new();
    let keywords_file = include_str!("keywords.v");
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
    use parser::*;
    #[test]
    fn get_keywords_is_run(){
        assert_eq!(get_keywords(),
            vec![
                vec!["fn".to_string(), "func".to_string(), "function".to_string()], 
                vec!["sub".to_string(), "subroutine".to_string()], 
                vec!["impl".to_string(), "implements".to_string()], 
                vec!["interface".to_string()], 
                vec!["enum".to_string()], 
                vec!["struct".to_string()], 
                vec!["import".to_string()], 
                vec!["use".to_string(), "using".to_string()], 
                vec!["as".to_string()], 
                vec!["macro".to_string()], 
                vec!["for".to_string()], 
                vec!["in".to_string()], 
                vec!["mod".to_string(), "module".to_string()], 
                vec!["mut".to_string(), "mutable".to_string()], 
                vec!["return".to_string()], 
                vec!["let".to_string()]
            ]
        );
    }
}
