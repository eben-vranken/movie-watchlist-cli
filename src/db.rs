use rusqlite::Connection;
use directories::ProjectDirs;
use anyhow::Result;
use tabled::Tabled;

#[derive(Debug)]
pub enum Status {
    Watched,
    Watching,
    Watchlist,
    Dropped,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Watched => "watched",
            Status::Watching => "watching",
            Status::Watchlist => "watchlist",
            Status::Dropped => "dropped",
        }
    }   

    fn as_status(args: &str) -> Status {
    match args {
            "watched" => Status::Watched,
            "watching" => Status::Watching ,
            "watchlist" => Status::Watchlist ,
            "dropped" => Status::Dropped,
            _ => Status::Watched, // Fallback for now, technically impossible to happen, but later I will add Error logging
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

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
pub struct Movie {
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
    #[tabled(rename = "Status")]
    pub status: Status,
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
            rating   INTEGER,
            status   TEXT NOT NULL DEFAULT 'watchlist'
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
            rating   INTEGER,
            status   TEXT NOT NULL DEFAULT 'watchlist'
        )",
        (),
    )?;

    Ok(conn)
}

pub fn add_movie(conn: &Connection, title: String) -> Result<()> {
    let movie = Movie {
        id: None,
        title: title,
        year: 1979,
        director: "Francis Ford Coppola".to_string(),
        runtime: 120,
        rating: 3,
        status: Status::Watched,
    };

    conn.execute(
        "INSERT INTO movies_watchlist (title, year, director, runtime, rating, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&movie.title, &movie.year, &movie.director, &movie.runtime, &movie.rating, movie.status.as_str())
    )?;

    Ok(())
}

pub fn rate_movie(conn: &Connection, title: String, rating: u8) -> Result<()> {
    let movie = Movie {
        id: None,
        title: title,
        year: 1985,
        director: "Star Wars".to_string(),
        runtime: 25,
        rating: rating,
        status: Status::Watched,
    };

    conn.execute(
        "INSERT INTO movies_seen_list (title, year, director, runtime, rating, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&movie.title, &movie.year, &movie.director, &movie.runtime, &movie.rating, movie.status.as_str())
    )?;

    Ok(())
}

pub fn view_watchlist(conn: &Connection) -> Result<Vec<Movie>> {
    let mut stmt = conn.prepare("SELECT id, title, year, director, runtime, rating, status FROM movies_watchlist")?;

    let movies: Vec<Movie> = stmt.query_map([], |row| {
        let status_str: String = row.get(6)?;
        Ok(Movie {
            id: row.get(0)?,
            title: row.get(1)?,
            year : row.get(2)?,
            director: row.get(3)?,
            runtime: row.get(4)?,
            rating: row.get(5)?,
            status: Status::as_status(&status_str),
        })
    })?
    .collect::<rusqlite::Result<Vec<Movie>>>()?;

    Ok(movies)
}

pub fn view_diary(conn: &Connection) -> Result<Vec<Movie>> {
    let mut stmt = conn.prepare("SELECT id, title, year, director, runtime, rating, status FROM movies_seen_list")?;

    let movies: Vec<Movie> = stmt.query_map([], |row| {
        let status_str: String = row.get(6)?;
        Ok(Movie {
            id: row.get(0)?,
            title: row.get(1)?,
            year : row.get(2)?,
            director: row.get(3)?,
            runtime: row.get(4)?,
            rating: row.get(5)?,
            status: Status::as_status(&status_str),
        })
    })?
    .collect::<rusqlite::Result<Vec<Movie>>>()?;

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