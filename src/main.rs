#[macro_use]
extern crate rocket;

mod film;
use entity::event;
use film::Film;

mod db;
use db::Db;

use rocket::fairing::{self, AdHoc};
use rocket::form::{Context, Form};
use rocket::response::Redirect;
use rocket::{Build, Request, Rocket};
use rocket_dyn_templates::{context, Template};

use migration::MigratorTrait;
use sea_orm::EntityTrait;
use sea_orm::ActiveModelTrait;
use sea_orm_rocket::{Connection, Database};
use sea_orm::ActiveValue::{Set, NotSet};

pub use entity::event::Entity as Event;

#[get("/")]
async fn index(conn: Connection<'_, Db>) -> Template {
    let db = conn.into_inner();
    let events: Vec<event::Model> = Event::find().all(db).await.unwrap();
    // TODO: async functional programming?
    let mut films: Vec<Film> = Vec::new();
    for event in events {
        let film_id: u64 = event.film.try_into().unwrap();
        let film = Film::from_id(film_id).await.unwrap();
        films.push(film);
    }
    Template::render("index", context! {films: &films})
}

#[get("/event/<id>")]
async fn get_event(id: u64) -> Template {
    let film = Film::from_id(id).await.unwrap();
    Template::render("film", &film)
}

#[get("/event/new")]
async fn new_event() -> Template {
    Template::render("new_event", context! {})
}

#[post("/event/new", data = "<new_event>")]
async fn create_event(conn: Connection<'_, Db>, new_event: Form<event::Model>) -> Redirect {
    let new_event = event::ActiveModel {
        film: Set(new_event.film),
        text: Set(new_event.text.to_owned()),
        ..Default::default()
    };
    let db = conn.into_inner();
    new_event.insert(db).await.unwrap();
    Redirect::to(uri!("/"))
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
        .mount("/", routes![index, get_event, new_event, create_event])
        .attach(Template::fairing())
}
