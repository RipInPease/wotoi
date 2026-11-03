use std::env::args;
use wotoi::cli::Args;

fn main() {
    //let args = args();
//
    //for arg in args {
    //    println!("arg: {arg}");
    //}
    
    let args = Args::from_args(args());
    println!("{:#?}", args);
}
