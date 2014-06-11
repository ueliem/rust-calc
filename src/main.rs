// use std::io::BufferedReader;
// use std::io::println;
// use std::io;
mod token;
mod mathparse;
mod tokenize;

fn main() {
    // let mut reader = BufferedReader::new(io::stdin());
    // let input = reader.read_line().unwrap();
    // println!("Hello World!");
    // println!("{}", &input);
    //let program = "1 + 1";
    let program = "22 - (2 + 3)\n";
    let all_tokens = tokenize::tokenize(program);
    println!("{}", all_tokens);
    println!("{}", mathparse::parse(all_tokens));
}
