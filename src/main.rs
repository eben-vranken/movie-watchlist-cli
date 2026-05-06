mod db;

use colored::Colorize;
use anyhow::anyhow;

enum Command {
    Add(String),
    View,
    Rate(String, u8),
    Stats,
    Delete(String),
}

fn parse_command() -> Result<Command, String> {
    let cmd = std::env::args().nth(1).ok_or("No command given")?;

    match cmd.as_str() {
        "add" => {
            let movie = std::env::args()
                .nth(2)
                .ok_or("add requires a movie title")?;
            Ok(Command::Add(movie))
        }
        "view" => Ok(Command::View),
        "rate" => {
            let movie = std::env::args()
                .nth(2)
                .ok_or("rate requires a movie title")?;
            let rating = std::env::args().nth(3).ok_or("rate requires a rating")?;

            let rating: f32 = rating
                .as_str()
                .parse()
                .map_err(|_| "Rating was not a numerical number".to_string())?;

            match validate_rating(rating) {
                Ok(rating) => Ok(Command::Rate(movie, rating)),
                Err(e) => Err(e)
            }
        }
        "stats" => Ok(Command::Stats),
        "delete" => {
            let movie: String = std::env::args()
                .nth(2)
                .ok_or("delete requires a movie title")?;
            Ok(Command::Delete(movie))
        }
        _ => Err(format!("{} is not a valid command", cmd)),
    }
}

fn validate_rating(rating: f32) -> Result<u8, String> {
    let doubled = rating * 2.0;

    if doubled < 0.0 || doubled > 10.0 {
        Err("Rating must be between 0 and 5".to_string())
    } else if doubled.fract() != 0.0 {
        Err("Rating must either be a whole number or .5 decimal".to_string())
    } else {
        Ok(doubled as u8)
    }
}

fn main() -> anyhow::Result<()>{
    let conn = db::initialize_db()?;

    match parse_command() {
        Ok(Command::Add(_title)) => db::add_movie(&conn)?,
        Ok(Command::View) => db::view_movies(&conn)?,
        Ok(Command::Rate(title, rating)) => rate_movie(title, rating),
        Ok(Command::Stats) => watchlist_stats(),
        Ok(Command::Delete(title)) => remove_movie_from_watchlist(title),
        Err(e) => println!("{}", e.red()),
    }

    Ok(())
}

fn view_watchlist() {
    println!("Watchlist");
}

fn add_movie_to_watchlist(title: String) {
    println!("Added '{}' to watchlist", title)
}

fn rate_movie(title: String, rating: u8) {
    let rating: f32 = rating as f32 / 2.0;

    if rating.fract() == 0.0 {
        println!("Rated '{}' a {}/5", title, rating);
    } else {
        println!("Rated '{}' a {:.1}/5", title, rating);
    }
}

fn watchlist_stats() {
    println!("Watchlist stats");
}

fn remove_movie_from_watchlist(title: String) {
    println!("Removed '{}' from watchlist", title);
}

#[cfg(test)]
mod tests {
    use super::*;
   
    mod rating_input {
        use super::*;

        // Rating tests
        #[test]
        fn test_whole_number_rating_is_valid() {
            assert_eq!(validate_rating(3.0), Ok(6));
        }

        #[test]
        fn test_half_step_rating_is_valid() {
            assert_eq!(validate_rating(3.5), Ok(7));
        }
       
        #[test]
        fn test_minimum_rating_is_valid() {
            assert_eq!(validate_rating(0.0), Ok(0));
        }
       
        #[test]
        fn test_maximum_rating_is_valid() {
            assert_eq!(validate_rating(5.0), Ok(10));
        }

        #[test]
        fn test_rating_over_maximum_is_rejected() {
            assert_eq!(validate_rating(10.0), Err("Rating must be between 0 and 5".to_string()));
        }

        #[test]
        fn test_rating_under_minimum_is_rejected() {
            assert_eq!(validate_rating(-5.0), Err("Rating must be between 0 and 5".to_string()));
        }

        #[test]
        fn test_wrong_half_step_rating_is_rejected() {
            assert_eq!(validate_rating(2.7), Err("Rating must either be a whole number or .5 decimal".to_string()));
        }
    }
}