pub fn keywords() -> Vec<Vec<String>> {
    vec![
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
     ).collect()
}
