mod film;
mod jfk_event;
use film::Film;
use jfk_event::JfkEvent;
#[macro_use] extern crate rocket;
use rocket_dyn_templates::{ Template, context};
use rocket::form::Form;

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
    let new_jfk_event = JfkEvent::new(film, text);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_event, new_event, create_event])
        .attach(Template::fairing())
}
