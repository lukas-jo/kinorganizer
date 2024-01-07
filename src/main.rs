#[macro_use]
extern crate rocket;

mod tmdb;
use tmdb::TmdbClient;
mod entity;
use entity::{event, film};

mod db;
use db::Db;

use rocket::fairing::{self, AdHoc};
use rocket::form::Form;
use rocket::fs::{ FileServer, relative};
use rocket::response::Redirect;
use rocket::{Build, Rocket, State};
use rocket_dyn_templates::{context, Template};
use rocket::serde::{Deserialize, Serialize, json::Json};

mod migration;
use migration::MigratorTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, FromQueryResult};
use sea_orm_rocket::{Connection, Database};

pub use entity::event::Entity as Event;
pub use entity::film::Entity as Film;

// TODO: use FromQueryResult
#[derive(Serialize, Deserialize)]
struct FilmEvent {
    event: event::Model,
    film : film::Model,
}

#[get("/")]
async fn index(conn: Connection<'_, Db>) -> Template {
    let db = conn.into_inner();
    let events: Vec<event::Model> = Event::find().all(db).await.unwrap();
    let mut films: Vec<FilmEvent> = Vec::new();
    for event in events {
        // TODO: replace with Option::inspect() when stable
        // TODO: use custom join statement instead?
        // TODO: why double unwrap?
        let film = event.find_related(Film).one(db).await.unwrap().unwrap();
        films.push(FilmEvent { event, film });
    }
    Template::render("index", context! {filmevents: &films})
}

#[get("/event/<id>")]
async fn get_event(conn: Connection<'_, Db>, id: i32) -> Template {
    let db = conn.into_inner();
    // TODO: why double unwrap? (probably: Result<Option<>>)
    let event: event::Model = Event::find_by_id(id).one(db).await.unwrap().unwrap();
    let film = event.find_related(Film).one(db).await.unwrap().unwrap();
    Template::render("event", context! {event: &event, film: &film})
}

#[get("/event/new")]
async fn new_event() -> Template {
    Template::render("new_event", context! {})
}

#[post("/event/new", data = "<new_event>")]
async fn create_event(
    tmdb: &State<TmdbClient>,
    conn: Connection<'_, Db>,
    new_event: Form<event::Model>,
) -> Redirect {
    let new_film = tmdb.from_id(new_event.tmdb_id).await.unwrap();
    let new_event = event::ActiveModel {
        tmdb_id: Set(new_event.tmdb_id),
        text: Set(new_event.text.to_owned()),
        ..Default::default()
    };
    let db = conn.into_inner();
    let _ = new_film.insert(db).await;
    let new_event = new_event.insert(db).await.unwrap();
    Redirect::to(uri!(get_event(new_event.id)))
}

#[get("/search-tmdb/<title>")]
async fn search_tmdb(tmdb: &State<TmdbClient>, title: String) -> Json<Vec<film::Model>> {
    let films = tmdb.search(title).await.unwrap();
    Json(films)
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .manage(TmdbClient::new(std::env::var("TMDB_TOKEN_V3").unwrap()))
        .mount("/", routes![index, get_event, new_event, create_event])
        .mount("/api", routes![search_tmdb])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
