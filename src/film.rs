use serde::Serialize;
use tmdb_api::movie::{credits::MovieCredits, details::MovieDetails, search::MovieSearch};
use tmdb_api::prelude::Command;
use tmdb_api::Client;

#[derive(Serialize)]
pub struct Film {
    title: String,
    tmdb_id: u64,
    year: String,
    desc: String,
    poster_path: String,
    director: String,
}

impl Film {
    pub async fn from(id: u64) -> Result<Self, String> {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let film = MovieDetails::new(id).execute(&client).await.unwrap();
        let credits = MovieCredits::new(id).execute(&client).await.unwrap();
        let director = credits
            .crew
            .iter()
            .find(|f| f.job == "Director")
            .map(|d| d.person.name.clone())
            .unwrap_or(String::from(""));

        Ok(Self {
            title: film.inner.title,
            tmdb_id: film.inner.id,
            year: film.inner.release_date.unwrap().to_string(),
            desc: film.inner.overview,
            poster_path: film.inner.poster_path.unwrap(),
            director,
        })
    }

    pub async fn search(title: String) -> Vec<Self> {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let search = MovieSearch::new(title);
        let result = search.execute(&client).await.unwrap();
        // result.results.iter().map(|f| Self::from(f.inner.id).await.unwrap()).collect()
        let mut films: Vec<Self> = Vec::new();
        for film in result.results {
            films.push(Self::from(film.inner.id).await.unwrap());
        }
        films
    }
}
