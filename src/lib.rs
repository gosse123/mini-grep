// use std::{io, string};
// use  std :: io :: BufReader ;
// use  std :: fs :: File ;

use clap::Parser;

#[derive(Parser)]
pub struct Cli{
    pattern:String,
    path:std::path::PathBuf
}

pub fn cli_analyse()-> (String,std::path::PathBuf){
    let args = Cli::parse();
    let (pattern,path) = (args.pattern,args.path);
    (pattern,path)
}


fn file_reader(path:std::path::PathBuf)->Result<String,Box<dyn std::error::Error>>{
    let content = std::fs::read_to_string(&path);
    match content {
        Ok(content)=>Ok(content),
        Err(error)=> return  Err(error.into())
    }
}

fn find_matches(content: &str, pattern: &str) ->Vec<String>{
    content
        .lines()
        .filter(|line|line.contains(pattern))
        .map(|line|line.to_string())
        .collect()
}

#[test]
fn test_find_matches() {
    let content = "rust est rapide\njava est lent\nrust est sûr";
    let pattern = "rust";

    let result = find_matches(content, pattern);

    assert_eq!(
        result,
        vec![
            "rust est rapide".to_string(),
            "rust est sûr".to_string()
        ]
    );
}