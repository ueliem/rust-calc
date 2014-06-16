use std::io::BufferedReader;
// use std::io::println;
use std::io;
mod token;
mod mathparse;
mod tokenize;

fn main() {
    let mut reader = BufferedReader::new(io::stdin());
    loop {
        let input = reader.read_line().unwrap();
        // println!("Hello World!");
        // println!("{}", &input);
        // let program = "1 + 1\n";
        // let program = "(22 - ((-3) * 2))\n";
        // let program = "(22 - (2 + 3) + 2)\n";
        //let all_tokens = tokenize::tokenize(input.as_slice());
        match tokenize::tokenize(input.as_slice()) {
            Some(n) => {
                match mathparse::parse(n.as_slice()) {
                    Some(m) => println!("{}", m),
                    None => println!("There was some error!")
                }
            },
            None => {
                println!("There was some error!");
                continue;
            }
        }

        //println!("{}", all_tokens);
        //println!("{}", mathparse::parse(all_tokens.as_slice()));
    }
}
