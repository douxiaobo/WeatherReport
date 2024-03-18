#[macro_use] extern crate rocket;

use rocket::http::RawStr;

#[get("/<province>/<city>")]
fn weather(province: String, city: String) -> String {
    format!("Weather for {} in {}", province, city)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![weather])
}