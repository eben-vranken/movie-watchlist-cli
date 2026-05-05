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

        match input {
            0 => break,
            1 => view_watchlist(),
            2 => mark_movie_as_seen(),
            3 => add_movie_to_watchlist(),
            4 => remove_movie_from_watchlist(),
            _ => println!("{} {}", input.to_string().cyan(), "is not a valid option!".red())
        }

        println!()
    }

    println!("{}", "Goodbye!".cyan());
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
    for (index, option) in MENU_OPTIONS.iter().enumerate() {
        let visual_index: &str = &(index + 1).to_string();
        println!("{}. {}", visual_index.red(), MENU_OPTIONS[index].green())
    }

    print_line();
}

fn ask_for_integer_input(prompt: &str) -> usize {
    loop {
        print!("{}", prompt.magenta());
        io::stdout().flush().unwrap();
    
        let mut option = String::new();
    
        io::stdin()
            .read_line(&mut option)
            .expect("Unexpected input");
        
        match option.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("{}", "Please enter a whole number".red())
        }
    }
}

fn view_watchlist() {

}

fn mark_movie_as_seen() {

}

fn add_movie_to_watchlist() {

}

fn remove_movie_from_watchlist() {

}