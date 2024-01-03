use tmdb_api::movie::{MovieBase, details::MovieDetails, search::MovieSearch};
use tmdb_api::prelude::Command;
use tmdb_api::Client;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Film {
    title: String,
    tmdb_id: u64,
    year: String,
    desc: String,
    poster_path: String,
}

impl Film {
    fn from_tmdb(movie: MovieBase) -> Self {
        Self {
            title: movie.title,
            tmdb_id: movie.id,
            year: movie.release_date.unwrap().to_string(),
            desc: movie.overview,
            poster_path: movie.poster_path.unwrap(),
        }
    }

    pub async fn from_id(id: u64) -> Result<Self, String> {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let film = MovieDetails::new(id).execute(&client).await.unwrap();

        Ok(Self::from_tmdb(film.inner))
    }

    pub async fn search(title: String) -> Vec<Self> {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let search = MovieSearch::new(title);
        let result = search.execute(&client).await.unwrap();
        result
            .results
            .iter()
            .map(|f| f.inner.clone())
            .map(Self::from_tmdb)
            .collect()
    }
}
