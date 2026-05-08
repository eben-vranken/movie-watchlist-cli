mod db;
mod tmdb;

use colored::Colorize;
use tabled::settings::Style;
use tabled::Tabled;

enum Command {
    Add,
    Watchlist,
    Diary,
    Rate,
    Search(String),
    Delete(String, u32),
}

fn parse_command() -> Result<Command, String> {
    let cmd = std::env::args().nth(1).ok_or("No command given")?;

    match cmd.as_str() {
        "add" => Ok(Command::Add),
        "watchlist" => Ok(Command::Watchlist),
        "diary" => Ok(Command::Diary),
        "rate" => Ok(Command::Rate),
        "search" => {
            let title = std::env::args().nth(2).ok_or("search requires a title")?;
            Ok(Command::Search(title))
        }
        "delete" => {
            let list_type: String = std::env::args()
                .nth(2)
                .ok_or("delete requires a movie title")?;

            
            let id = std::env::args().nth(3).ok_or("id needs to be a number")?;

            let id:u32 = id.as_str().parse().map_err(|_| "ID was not a numerical number.".to_string())?;
            Ok(Command::Delete(list_type, id))
        }
        _ => Err(format!("{} is not a valid command", cmd)),
    }
}

fn main() -> anyhow::Result<()>{
    dotenvy::dotenv().ok();

    let conn = db::initialize_db()?;

    match parse_command() {
        Ok(Command::Add) => db::add_movie(&conn)?,
        Ok(Command::Watchlist) => print_list(db::view_watchlist(&conn)?),
        Ok(Command::Diary) => print_list(db::view_diary(&conn)?),
        Ok(Command::Rate) => db::rate_movie(&conn)?,
        Ok(Command::Search(title)) => tmdb::search_movie_from_title(&title)?, 
        Ok(Command::Delete(list_type, id)) => db::delete_from_list(&conn, list_type, id)?,
        Err(e) => println!("{}", e.red()),
    }

    Ok(())
}

fn print_list<T: Tabled>(movies: Vec<T>) {
    let table = tabled::Table::new(movies).with(Style::rounded()).to_string();
    println!("{}", table);
}

fn _add_movie_to_watchlist(title: String) {
    println!("Added '{}' to watchlist", title)
}