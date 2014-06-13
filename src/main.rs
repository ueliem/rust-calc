use std::io::BufferedReader;
// use std::io::println;
use std::io;
mod token;
mod mathparse;
mod tokenize;

fn main() {
    let mut reader = BufferedReader::new(io::stdin());
    while true {
        let input = reader.read_line().unwrap();
        // println!("Hello World!");
        // println!("{}", &input);
        // let program = "1 + 1\n";
        // let program = "(22 - ((-3) * 2))\n";
        // let program = "(22 - (2 + 3) + 2)\n";
        let all_tokens = tokenize::tokenize(input.as_slice());
        //println!("{}", all_tokens);
        //println!("{}", mathparse::parse(all_tokens.as_slice()));
        println!("{}", mathparse::parse(all_tokens.as_slice()));
    }
}
