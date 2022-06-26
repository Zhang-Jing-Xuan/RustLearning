// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    let mut success:bool=false;
    let mut incorrect_guesses:u32=0;
    let mut guessed_chars:Vec<char>=Vec::new();
    println!("Welcome to CS110L Hangman!");
    print!("The word so far is ");
    for i in secret_word_chars.iter(){
        print!("-");
    }
    println!("");
    println!("You have guessed the following letters:");
    println!("You have {} guesses left", NUM_INCORRECT_GUESSES);
    print!("Please guess a letter: ");
    io::stdout()
    .flush()
    .expect("Error flushing stdout.");
    let mut guess:String=String::new();
    let mut success:bool=false;
    while success!=true && incorrect_guesses<NUM_INCORRECT_GUESSES{
        io::stdin().read_line(&mut guess).expect("Error reading input");
        guess =guess.trim().replace("\\s*|\t|\r|\n", "");
        let mut guess_chars:Vec<char>=guess.chars().collect();
        // println!("{}",guess_chars[guess_chars.len()-1]);
        // println!("{}",secret_word);  
        let mut suc:bool=false;
        let check:char=guess_chars[guess_chars.len()-1];
        // todo: check if guess_chars shoud add to guessed_chars
        guessed_chars.push(check);
        if secret_word_chars.contains(&check){
            suc=true;
        }
        if suc==false{
            incorrect_guesses+=1;
            println!("Sorry, that letter is not in the word");
            println!("");
            print!("The word so far is ");
            for i in secret_word_chars.iter(){
                if guessed_chars.contains(i){
                    print!("{}",*i);
                }else{
                    print!("-");
                }
            }
            println!("");
        }else{
            suc=false;
            success=true;
            println!("");
            print!("The word so far is ");
            for i in secret_word_chars.iter(){
                if guessed_chars.contains(i){
                    print!("{}",*i);
                }else{
                    print!("-");
                    success=false;
                }
            }
            println!("");
        }
        if NUM_INCORRECT_GUESSES-incorrect_guesses==0{
            println!("");
            println!("Sorry, you ran out of guesses!");
            break;
        }else if success==true{
            println!("");
            println!("Congratulations you guessed the secret word: {}!",secret_word);
            break;
        }
        print!("You have guessed the following letters:");
        for i in guessed_chars.iter(){
            print!("{}",*i);
        }
        println!("");
        println!("You have {} guesses left", NUM_INCORRECT_GUESSES-incorrect_guesses);
        print!("Please guess a letter: ");
        io::stdout()
        .flush()
        .expect("Error flushing stdout.");
    }
}
