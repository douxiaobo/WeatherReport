use rocket::*;
use rocket::response::content::RawHtml;
extern  crate serde;

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
async fn weather(province: String, city: String) -> RawHtml<String> {  
    let mut text = format!(  "<h1>The Weather Report</h1> <hr>" );  
    text += &format!("<p>Hi there! Welcome {0},{1}</p>", province, city);
    text+=&format!(
        "<table>
            <thead>
                <tr>
                    <th>
                        <b>Province:</b>
                    </th>
                    <th>
                        <b>City:</b>
                    </th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>{0}</td>
                    <td>{1}</td>
                </tr>
            </tbody>
        </table>", province, city);

    let url=format!("https://wis.qq.com/weather/common?source=pc&city={}&province={}&weather_type=observe",city,province);

    let client = match reqwest::Client::builder().build(){
        Ok(client)=>client,
        Err(e)=>{
            println!("Failed to create client:{}",e);//也可以使用panic!
            return rocket::response::content::RawHtml(text);
        }
    };

    let request = client.request(reqwest::Method::GET, url);

    let response = match request.send().await{
        Ok(response)=>response,
        Err(e)=>{
            println!("Failed to send request:{}",e);
            return rocket::response::content::RawHtml(text);
        }
    };
    let body = match response.text().await{
        Ok(body)=>body,
        Err(e)=>{
            println!("Failed to get response body:{}",e);
            return rocket::response::content::RawHtml(text);
        }
    };
    text+=&format!("<p>{}</p>",body);
    
    let model:WeatherReport=serde_json::from_str(&body).unwrap();
    text+=&format!("<p>{}{}的天气是：{}</p>",province,city,model.data.observe.weather);
    text+=&format!("<p>{}{}的温度是：{}</p>",province,city,model.data.observe.degree);

    rocket::response::content::RawHtml(text)  
}  

#[launch]
fn rocket()->_{
    rocket::build().mount("/", routes![weather])
}