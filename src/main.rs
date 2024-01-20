mod tmdb;
use tmdb::TmdbClient;
mod entity;
pub use entity::event::Entity as Event;
pub use entity::film::Entity as Film;
use entity::{event, film};
mod migration;
use migration::{Migrator, MigratorTrait};

use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, Redirect},
    routing::{get, post},
    Router,
};
use dotenv;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;

// TODO: use FromQueryResult
#[derive(Serialize, Deserialize)]
struct FilmEvent {
    event: event::Model,
    film: film::Model,
}

async fn index(State(state): State<AppState>) -> Result<Html<String>, (StatusCode, &'static str)> {
    let events: Vec<event::Model> = Event::find()
        .all(&state.conn)
        .await
        .expect("Cannot find events");
    let mut films: Vec<FilmEvent> = Vec::new();
    for event in events {
        // TODO: replace with Option::inspect() when stable
        // TODO: use custom join statement instead?
        // TODO: why double unwrap?
        let film = event
            .find_related(Film)
            .one(&state.conn)
            .await
            .unwrap()
            .unwrap();
        films.push(FilmEvent { event, film });
    }
    let mut context = tera::Context::new();
    context.insert("filmevents", &films);
    let body = state
        .templates
        .render("index.html", &context)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;
    Ok(Html(body))
}

async fn get_event(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    // TODO: why double unwrap? (probably: Result<Option<>>)
    let event: event::Model = Event::find_by_id(id)
        .one(&state.conn)
        .await
        .unwrap()
        .unwrap();
    let film = event
        .find_related(Film)
        .one(&state.conn)
        .await
        .unwrap()
        .unwrap();

    let mut context = tera::Context::new();
    context.insert("event", &event);
    context.insert("film", &film);

    let body = state
        .templates
        .render("event.html", &context)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

async fn new_event(
    State(state): State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let context = tera::Context::new();
    let body = state
        .templates
        .render("new_event.html", &context)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

async fn create_event(
    State(state): State<AppState>,
    Form(new_event): Form<event::Model>,
) -> Redirect {
    let new_film = state.tmdb.from_id(new_event.tmdb_id).await.unwrap();
    let new_event = event::ActiveModel {
        tmdb_id: Set(new_event.tmdb_id),
        text: Set(new_event.text.to_owned()),
        ..Default::default()
    };
    let _ = new_film.insert(&state.conn).await;
    let new_event = new_event.insert(&state.conn).await.unwrap();
    Redirect::to(&format!("/event/{}", new_event.id).as_str())
}

async fn search_tmdb(
    State(state): State<AppState>,
    title: String,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let mut films = state.tmdb.search(title.as_str()).await.unwrap();
    films.truncate(5);
    let mut context = tera::Context::new();
    context.insert("films", &films);
    let body = state
        .templates
        .render("tmdb_search_results.html", &context)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

#[derive(Clone)]
struct AppState {
    templates: Tera,
    conn: DatabaseConnection,
    tmdb: Arc<TmdbClient>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let host = env::var("HOST").expect("HOST is not set");
    let port = env::var("PORT").expect("PORT is not set");
    let server_url = format!("{host}:{port}");
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&conn, None)
        .await
        .expect("Database migration failed");

    let templates = Tera::new("templates/*").expect("Tera initialization failed");

    let tmdb = TmdbClient::new(env::var("TMDB_TOKEN_V3").expect("TMDB_TOKEN_V3 is not set"));

    let state = AppState {
        templates,
        conn,
        tmdb: Arc::new(tmdb),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/event/new", get(new_event).post(create_event))
        .route("/event/:id", get(get_event))
        .route("/api/search", post(search_tmdb))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    let _ = axum::serve(listener, app).await.unwrap();
}
