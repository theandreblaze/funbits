#![allow(unused)]


use std::{env, fs::File, io::Read};
use std::collections::HashMap;




fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut filename: File = File::open(&args[1]).expect("failed to open provided file by name");
    let command: &String = &args[2];
    let mut filebuf: Vec<u8> = vec![];
    let mut char_counter: HashMap<String, i32> = HashMap::new();

    match command as &str {
        "-c" => {
            let data: usize = File::read_to_end(&mut filename, &mut filebuf).unwrap();
            println!("{:?} bytes", &data);
        }
        "-m" | "--chars" => {

            let temp_buf = filebuf.clone();
            println!("{:?}", &temp_buf);
           if let Ok(ch) = String::from_utf8(temp_buf){
                println!("{:?}",ch.chars().count());
            };

            println!("print the character counts");
        }
    },
        "-l" | "--lines" => {


            println!("print the newline counts");
        },

        "-L" | "--max-line-length" => {
            println!("print the maximum display width");
        }
        "-w" | "--words" => {
            println!("print the word counts");
        }
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
