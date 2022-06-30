// cargo run test.txt
use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    // unimplemented!();
    // Be sure to delete the #[allow(unused)] line above
    let file = File::open(filename)?;
    let mut lines=Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        // do something with line_str
        lines.push(line_str);
    }
    Ok(lines)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let file=read_file_lines(filename).expect("Read file error");
    println!("The number of lines is {}.",{file.len()});
    let mut words_counts=0;
    let mut chars_counts=0;
    for line in file{
        let words=line.split_whitespace();
        for _word in words{
            words_counts+=1;
            chars_counts+=_word.len();
        }
    }
    println!("The number of words is {}.",words_counts);
    println!("The number of characters is {}.",chars_counts);
}
