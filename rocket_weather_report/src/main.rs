#[macro_use] extern crate rocket;

use serde::{
    Serialize,Deserialize
};

#[derive(Serialize, Deserialize)]
pub struct WeatherReport {
    data: Data,
    message: String,
    status: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    observe: Observe,
}

#[derive(Serialize, Deserialize)]
pub struct Observe {
    degree: String,
    humidity: String,
    precipitation: String,
    pressure: String,
    update_time: String,
    weather: String,
    weather_bg_pag: String,
    weather_code: String,
    weather_color: Option<serde_json::Value>,
    weather_first: String,
    weather_pag: String,
    weather_short: String,
    weather_url: String,
    wind_direction: String,
    wind_direction_name: String,
    wind_power: String,
}

#[get("/<province>/<city>")]
async fn weather(province: String, city: String) -> String {
    let mut text = format!("Hello, {}, {}\n", province, city);
    text += &format!("Weather for {} in {}", province, city);
    let url=format!("https://wis.qq.com/weather/common?source=pc&city={}&province={}&weather_type=observe",city,province);

    let client = match reqwest::Client::builder().build(){
        Ok(client)=>client,
        Err(e)=>{
            println!("Failed to create client:{}",e);//也可以使用panic!
            return text;
        }
    };

    let request = client.request(reqwest::Method::GET, url);

    let response = match request.send().await{
        Ok(response)=>response,
        Err(e)=>{
            println!("Failed to send request:{}",e);
            return text;
        }
    };
    let body = match response.text().await{
        Ok(body)=>body,
        Err(e)=>{
            println!("Failed to get response body:{}",e);
            return text;
        }
    };

    text+=&format!("{}\n",body);

    let model:WeatherReport=serde_json::from_str(&body).unwrap();
    
    //println!("{}{}的天气是：{}",province,city,model.data.observe.weather);//确实是在Terminal能显示
    //println!("{}{}的温度是：{}",province,city,model.data.observe.degree);

    text+=&format!("{}{}的天气是：{}\n",province,city,model.data.observe.weather);
    text+=&format!("{}{}的温度是：{}",province,city,model.data.observe.degree);

    text
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![weather])
}