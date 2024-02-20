#![allow(unused)]

use std::collections::HashMap;
use std::{env, fs::File, io::Read};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut file: File = File::open(&args[1]).expect("failed to open provided file by name");
    let command: &String = &args[2];
   

    match command as &str {
        "-c" => {
            let mut filebuf: Vec<u8> = vec![];
            let data: usize = File::read_to_end(&mut file, &mut filebuf).unwrap();
            println!("{:?} {:?}bytes", &data, &args[1]);
        },
        "-m" | "--chars" => {
            let mut filebuf: String = String::new();
            file.read_to_string(&mut filebuf).unwrap();
            println!("{:?} {:?}",filebuf.chars().count(), &args[1]);
        },
        "-l" | "--lines" => {
            
            
            let mut filebuf: String = String::new();
            file.read_to_string(&mut filebuf).unwrap();
            let mut line_count: i64 = 0;
            for line in filebuf.lines(){
                    line_count += 1; 
                };
            
            println!("{:?} {:?}",line_count, &args[1]);
        },

        "-L" | "--max-line-length" => {
            println!("print the maximum display width");
            todo!("implement max-line-length");
        },

        "-w" | "--words" => {

            let mut filebuf: String = String::new();
            file.read_to_string(&mut filebuf).unwrap();
            let mut word_count: usize = 0;
            let mut word_list: Vec<_> = vec![];
            for line in filebuf.lines(){
                if !line.is_empty(){
                word_list = line.split(" ").collect::<Vec<&str>>();
                word_count += word_list.len();
                }
                continue;
            };

             println!("{:?} {:?}", &word_count, &args[1]);
        },           

        "--help" | "-h" => {
            println!("Display help menu");
        }

        "--version" => {
            println!("
            ccwc 0.1\n
            This is free, experimental and non production-ready software: use at own risk. 
            There is NO WARRANTY, to the extent permitted by law.\n
            Written by Andre Blaze Henshaw."
                );
        }

        _ => println!("unrecognized instruction. Please see help menu (enter ccwc --help or -h)   for use cases and flags"),
    }
}






#[test]

fn test_non_unicode_input(){

}

fn test_maximum_text_file_length(){

}


