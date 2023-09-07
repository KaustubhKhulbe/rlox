use std::{env, io};
use std::fs::File;
use std::io::{BufReader, Write};
use std::io::Read;
use std::process;
use std::str;

use crate::scanner::Scanner;
use crate::token::Token;

mod token;
mod scanner;
static mut HAD_ERROR: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());
    if args.len() > 2 {
        dbg!("Too many arguments");
        process::exit(0b1000000);
    } else if args.len() == 2 {
        run_file(&args[1]).ok();
    } else {
        run_prompt();
    }
}

pub fn convert_bytes_to_string(buffer: &Vec<u8>) -> &str {
     match str::from_utf8(buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

pub fn run_file(path: &String) -> io::Result<()>{
    // reads file `f` into a vector of bytes
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // converts vector of bytes into a string
    let s = convert_bytes_to_string(&buffer);
    run(s.to_owned());

    if unsafe { HAD_ERROR } {
        process::exit(0b1000000);
    }
    Ok(())
}

pub fn run_prompt() {
    eprintln!("Press Enter to return.");
    let s = &mut String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(s).unwrap();

        if s.is_empty() { //EOF command
            break;
        }

        run(s.to_owned());
        unsafe {HAD_ERROR = false;}
        s.clear();
    }
}

pub fn run(s: String) {
    // todo!("Implement run");
    println!("{}", s);
}

pub fn error(line: u32, message: &str) {
    panic!("[line {}] Error: {}", line, message);
}
