use rusqlite::Connection;
use directories::ProjectDirs;
use anyhow::Result;

#[derive(Debug)]
enum Status {
    Watched,
    Watching,
    Watchlist,
    Dropped,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Watched => "watched",
            Status::Watching => "watching",
            Status::Watchlist => "watchlist",
            Status::Dropped => "dropped",
        }
    }
}

#[derive(Debug)]
struct Movie {
    title: String,
    year: u16,
    director: String,
    runtime: u16,
    rating: u8,
    status: Status,
}

pub fn initialize_db() -> Result<Connection> {
    let proj_dirs = ProjectDirs::from("", "", "personal_movie_watchlist").ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;
    let data_dir = proj_dirs.data_dir();
    std::fs::create_dir_all(data_dir)?;
    let db_path = data_dir.join("watchlist.db");

    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS movies (
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

pub fn add_movie(conn: &Connection) -> Result<()> {
    let movie = Movie {
        title: "Apocalypse Now".to_string(),
        year: 1979,
        director: "Francis Ford Coppola".to_string(),
        runtime: 120,
        rating: 10,
        status: Status::Watched,
    };

    conn.execute(
        "INSERT INTO movies (title, year, director, runtime, rating, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&movie.title, &movie.year, &movie.director, &movie.runtime, &movie.rating, movie.status.as_str())
    )?;

    Ok(())
}