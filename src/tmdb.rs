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

pub fn search_movie_from_title(query: &str) -> anyhow::Result<()>  {
    let api_key = std::env::var("TMDB_API_KEY")
        .map_err(|_| anyhow::anyhow!("TMDB_API_KEY env var not set"))?;


    let client = reqwest::blocking::Client::new();

    let url: String = format!("https://api.themoviedb.org/3/search/movie?api_key={}&query={}", api_key, query);
    let res = client.get(url).send()?;

    // Going to parse data here
    let mut parsed = res.json::<SearchResponse>()?;
    parsed.results.iter_mut().enumerate().for_each(|(i, r)| r.index = i + 1);

    let table = tabled::Table::new(parsed.results).with(Style::rounded()).to_string();
    println!("{}", table);

    Ok(())
}
