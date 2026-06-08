use clap::Parser;

#[derive(Parser)]
pub struct Cli{
    pattern:String,
    path:std::path::PathBuf
}

pub fn cli_analyse(){
    let args = Cli::parse();
    println!("pattern = {:?}, path = {:?}",args.pattern,args.path);
}

