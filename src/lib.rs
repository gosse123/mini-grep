use std::{io, string};
use  std :: io :: BufReader ;
use  std :: fs :: File ;

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