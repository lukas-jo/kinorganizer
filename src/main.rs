#[macro_use] extern crate rocket;
use letterboxd;

#[get("/")]
async fn index() -> String {
    let api_key_pair = letterboxd::ApiKeyPair::new("".to_string(), "".to_string());
    let client = letterboxd::Client::new(api_key_pair);
    let film = client.film("802113").await.unwrap();
    let name = film.name;
    name
    // "Test"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
