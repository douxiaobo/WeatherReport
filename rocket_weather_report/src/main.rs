#[macro_use] extern crate rocket;



#[get("/<province>/<city>")]
fn weather(province: String, city: String) -> String {
    let mut text = format!("Hello, {}, {}\n", province, city);
    text += &format!("Weather for {} in {}", province, city);

    text
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![weather])
}