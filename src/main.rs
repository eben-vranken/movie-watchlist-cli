use colored::Colorize;
use std::io::{self, Write};

const TITLE: &str = "PERSONAL MOVIE WATCHLIST";
const VERSION: &str = "0.0.1";
const AUTHOR: &str = "Eben Vranken";

const MENU_OPTIONS: [&str; 4] = [
    "View watchlist",
    "Mark movie as seen",
    "Add movie",
    "Remove movie"
];

fn main() {
    startup_info();
    
    loop {
        print_menu();
        
        let input: usize = ask_for_integer_input("Pick an option: ");
        println!("\nYour input: {}", input.to_string())
    }
}

fn startup_info() {
    print_line();
    println!("{}", TITLE.blue());
    print_line();
    println!("Version: {}", VERSION.cyan());
    println!("Author: {}", AUTHOR.cyan());
    print_line();
}

fn print_line() {
    println!("{}", "-".repeat(TITLE.len()).blue());
}

fn print_menu() {
    for index in 0..MENU_OPTIONS.len() {
        let visual_index: &str = &(index + 1).to_string();
        println!("{}. {}", visual_index.red(), MENU_OPTIONS[index].green())
    }

    print_line();
}

fn ask_for_integer_input(prompt: &str) -> usize {
    print!("{}", prompt.magenta());
    io::stdout().flush().unwrap();

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Unexpected input");
    
    let option: usize = option.trim().parse().expect("Input was not a decimal number!");

    option
}