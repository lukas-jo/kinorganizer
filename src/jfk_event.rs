use super::film::Film;
use serde::Serialize;

#[derive(Serialize)]
pub struct JfkEvent {
    film: Film,
    text: String,
}
