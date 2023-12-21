mod film;
use film::Film;
#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;

#[get("/")]
async fn index() -> Template {
    let film = Film::from(274).await.unwrap();
    Template::render("film", &film)
}

#[get("/<title>")]
async fn search_film(title: String) -> Template {
    let films = Film::search(title).await;
    let film = films.first();
    Template::render("film", &film)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, search_film])
        .attach(Template::fairing())
}
