use super::film::Film;
use serde::Serialize;

#[derive(Serialize)]
pub struct JfkEvent {
    film: Film,
    text: String,
}

impl JfkEvent {
    pub fn new(film: Film, text: String) -> Self {
        Self{film, text}
    }
}
