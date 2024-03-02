// Author: Apostolos Chalis 2024 
// Rust Remove is a rm clone written in Rust as a method to learn the language. 
use std::fs; 
use std::env; 
use std::path::Path; 
use std::fs::metadata; 

fn main() {
    // Building first party CLI argument parser. 
    let args: Vec<String> = env::args().collect();
    let flag = &args[1]; // supports one CLI argument right now 

    // Options should be first because the REGEX expression sucks at understanding that -v and -h
    // are not file names 
    if flag == "-v"{
        println!("Version 0.1.0 (Prototype)\nAuthor: Apostolos Chalis 2024");
    }
    else if flag == "-h" {
        println!("rrm [FILE PATH] or rrm [OPTION]\nOptions:\n-v displays version info.\n-h displays this message.")
    }
    else{
        if Path::new(flag).exists() == true{
            let dir_or_file = metadata(flag).unwrap(); 
            
            if dir_or_file.is_dir() == false{
                let _ = fs::remove_file(flag);
            }
            else if dir_or_file.is_dir() == true{
                let _ = fs::remove_dir_all(flag); 
            }
        }
    }
}
