#[macro_use]
extern crate rocket;

mod film;
use film::Film;

mod db;
use db::Db;

use rocket_dyn_templates::{ Template, context};
use rocket::fairing::{self, AdHoc};
use rocket::form::{Context, Form};
use rocket::{Build, Request, Rocket};

use migration::MigratorTrait;
use sea_orm_rocket::{Connection, Database};

pub use entity::event::Entity as Event;

#[derive(FromForm)]
struct NewEvent<'r> {
    film_id: u64,
    r#text: &'r str,
}

#[get("/")]
async fn index() -> Template {
    let title = String::from("Enemy");
    let films = Film::search(title).await;
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

#[post("/event/new", data = "<event>")]
async fn create_event(event: Form<NewEvent<'_>>) {
    let film = Film::from_id(event.film_id).await.unwrap();
    let text = event.text.to_string();
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
