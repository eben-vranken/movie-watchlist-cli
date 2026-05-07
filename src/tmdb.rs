use serde::Deserialize;
use tabled::Tabled;
use tabled::settings::Style;

#[derive(Deserialize)]
struct SearchResponse {
    results: Vec<SearchResult>
}

#[derive(Deserialize, Tabled)]
struct SearchResult {
    #[tabled(rename = "#")]
    #[serde(default)]
    index: usize,
    title: String,
    release_date: String,
}

const API_KEY: &str = "3c5f587451e7a0dd03214e56485e818f";

pub fn search_movie_from_title(query: &str) -> Result<(), reqwest::Error>  {
    let client = reqwest::blocking::Client::new();

    let url: String = format!("https://api.themoviedb.org/3/search/movie?api_key={}&query={}", API_KEY, query);
    let res = client.get(url).send()?;

    // Going to parse data here
    let mut parsed = res.json::<SearchResponse>()?;
    parsed.results.iter_mut().enumerate().for_each(|(i, r)| r.index = i + 1);

    let table = tabled::Table::new(parsed.results).with(Style::rounded()).to_string();
    println!("{}", table);

    Ok(())
}