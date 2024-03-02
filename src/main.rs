// Author: Apostolos Chalis 2024 
// Rust Remove is a rm clone written in Rust as a method to learn the language. 
use std::env; 
use regex::Regex; 

fn main() {
    let file_path_regex = Regex::new(r"^(\/?[a-zA-Z0-9_.-]+\/)*[a-zA-Z0-9_.-]+$").unwrap();

    // Building first party CLI argument parser. 
    let args: Vec<String> = env::args().collect();
    let flag = &args[1]; // supports one CLI argument right now 

    if flag == "-v"{
        println!("Version 0.1.0 (Prototype)\nAuthor: Apostolos Chalis 2024");
    }
    else if flag == "-h" {
        println!("rrm [FILE PATH] or rrm [OPTION]\nOptions:\n-v displays version info.\n-h displays this message.")
    }
    else if file_path_regex.is_match(flag){
        println!("About to remove file: {flag}");
    }
    else{
        println!("Error: Unsupported flag!");
    }
 
}
