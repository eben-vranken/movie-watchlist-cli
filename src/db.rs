use rusqlite::Connection;
use directories::ProjectDirs;
use anyhow::Result;
use std::io::Write;
use tabled::Tabled;

fn display_rating(rating: &u8) -> String {
    format!("{}", *rating as f32 / 2.0)
}

fn display_id(id: &Option<u32>) -> String {
    match id {
        Some(n) => n.to_string(),
        None => "-".to_string()
    }
}

#[derive(Debug, Tabled)]
pub struct MovieWatchlist {
    #[tabled(rename = "ID", display("display_id"))]
    pub id: Option<u32>,
    #[tabled(rename = "Title")]
    pub title: String,
    #[tabled(rename = "Year")]
    pub year: u16,
    #[tabled(rename = "Director")]
    pub director: String,
    #[tabled(rename = "Runtime")]
    pub runtime: u16,
}

#[derive(Debug, Tabled)]
pub struct MovieDiary {
    #[tabled(rename = "ID", display("display_id"))]
    pub id: Option<u32>,
    #[tabled(rename = "Title")]
    pub title: String,
    #[tabled(rename = "Year")]
    pub year: u16,
    #[tabled(rename = "Director")]
    pub director: String,
    #[tabled(rename = "Runtime")]
    pub runtime: u16,
    #[tabled(rename = "Rating", display("display_rating"))]
    pub rating: u8,
}

pub fn initialize_db() -> Result<Connection> {
    let proj_dirs = ProjectDirs::from("", "", "personal_movie_watchlist").ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;
    let data_dir = proj_dirs.data_dir();
    std::fs::create_dir_all(data_dir)?;
    let db_path = data_dir.join("watchlist.db");

    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS movies_seen_list (
            id       INTEGER PRIMARY KEY,
            title    TEXT NOT NULL UNIQUE,
            year     INTEGER,
            director TEXT,
            runtime  INTEGER,
            rating   INTEGER
        )",
        (),
    )?;

        conn.execute(
        "CREATE TABLE IF NOT EXISTS movies_watchlist (
            id       INTEGER PRIMARY KEY,
            title    TEXT NOT NULL UNIQUE,
            year     INTEGER,
            director TEXT,
            runtime  INTEGER,
            rating   INTEGER
        )",
        (),
    )?;

    Ok(conn)
}

pub fn add_movie(conn: &Connection) -> Result<()> {
    let title = ask_for_input("Title: ");
    let year = ask_for_input("Year: ");
    let director = ask_for_input("Director: ");
    let runtime = ask_for_input("Runtime: ");

    conn.execute(
        "INSERT INTO movies_watchlist (title, year, director, runtime) VALUES (?1, ?2, ?3, ?4)",
        (&title, &year, &director, &runtime)
    )?;

    Ok(())
}

pub fn rate_movie(conn: &Connection) -> Result<()> {
    let title = ask_for_input("Title: ");
    let year = ask_for_input("Year: ");
    let director = ask_for_input("Director: ");
    let runtime = ask_for_input("Runtime: ");
    let rating_input = ask_for_input("Rating (0-5, half-steps allowed): ");

    let rating = parse_rating(&rating_input)?;

    conn.execute(
        "INSERT INTO movies_seen_list (title, year, director, runtime, rating) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&title, &year, &director, &runtime, &rating)
    )?;

    Ok(())
}

fn parse_rating(input: &str) -> Result<u8> {
    let value: f32 = input
        .parse()
        .map_err(|_| anyhow::anyhow!("rating must be a number"))?;

    let doubled = value * 2.0;

    if doubled < 0.0 || doubled > 10.0 {
        return Err(anyhow::anyhow!("rating must be between 0 and 5"));
    }
    if doubled.fract() != 0.0 {
        return Err(anyhow::anyhow!("rating must be a whole or half step (e.g. 3 or 3.5)"));
    }

    Ok(doubled as u8)
}

pub fn view_watchlist(conn: &Connection) -> Result<Vec<MovieWatchlist>> {
    let mut stmt = conn.prepare("SELECT id, title, year, director, runtime FROM movies_watchlist")?;

    let movies: Vec<MovieWatchlist> = stmt.query_map([], |row| {
        Ok(MovieWatchlist {
            id: row.get(0)?,
            title: row.get(1)?,
            year : row.get(2)?,
            director: row.get(3)?,
            runtime: row.get(4)?,
        })
    })?
    .collect::<rusqlite::Result<Vec<MovieWatchlist>>>()?;

    Ok(movies)
}

pub fn view_diary(conn: &Connection) -> Result<Vec<MovieDiary>> {
    let mut stmt = conn.prepare("SELECT id, title, year, director, runtime, rating FROM movies_seen_list")?;

    let movies: Vec<MovieDiary> = stmt.query_map([], |row| {
        Ok(MovieDiary {
            id: row.get(0)?,
            title: row.get(1)?,
            year : row.get(2)?,
            director: row.get(3)?,
            runtime: row.get(4)?,
            rating: row.get(5)?,
        })
    })?
    .collect::<rusqlite::Result<Vec<MovieDiary>>>()?;

    Ok(movies)
}

pub fn delete_from_list(conn: &Connection, list_type: String, id: u32) -> Result<()> {
    let table = match list_type.as_str() {
        "diary" => "movies_seen_list",
        "watchlist" => "movies_watchlist",
        other => return Err(anyhow::anyhow!("unknown list: {}", other)),
    };

    let sql = format!("DELETE FROM {} WHERE id = ?1", table);
    let rows = conn.execute(&sql, [&id])?;

    if rows == 0 {
        println!("no movie with id {}", id);
    }

    Ok(())
}

fn ask_for_input(prompt: &str) -> String {
    let mut input = String::new();
    
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("Something went wrong");

    input.trim().to_string()
} 