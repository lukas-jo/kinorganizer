use sea_orm::TryIntoModel;
use tmdb_api::error::Error as TmdbError;
use tmdb_api::movie::{details::MovieDetails, search::MovieSearch, MovieBase};
use tmdb_api::prelude::Command;
use tmdb_api::Client;

use super::film;
use sea_orm::ActiveValue::Set;

pub struct TmdbClient {
    client: Client,
}

impl TmdbClient {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(token),
        }
    }

    fn to_entity(&self, movie: MovieBase) -> film::ActiveModel {
        super::film::ActiveModel {
            title: Set(movie.title),
            tmdb_id: Set(movie.id.try_into().unwrap_or_default()),
            year: Set(movie.release_date.unwrap_or_default().to_string()),
            desc: Set(movie.overview),
            poster_path: Set(movie.poster_path.unwrap_or_default()),
        }
    }

    pub async fn from_id(&self, id: i64) -> Result<film::ActiveModel, TmdbError> {
        let film = MovieDetails::new(id.try_into().unwrap()).execute(&self.client).await?;
        Ok(self.to_entity(film.inner))
    }

    pub async fn search(&self, title: &str) -> Result<Vec<film::Model>, TmdbError> {
        let search = MovieSearch::new(String::from(title));
        let result = search.execute(&self.client).await?;
        Ok(result
            .results
            .iter()
            .map(|f| f.inner.clone())
            .map(|f| self.to_entity(f).try_into_model().unwrap())
            .collect())
    }
}
