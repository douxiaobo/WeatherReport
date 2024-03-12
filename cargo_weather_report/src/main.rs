#![allow(unused_assignments)]
#[warn(unused_variables)]

use std::io::Write;
// use std::io::prelude::*;
use std::fs::File;
use std::io::{self,BufRead};
// use std::path::{self, Path};

// use std::collections::HashMap;

// use reqwest::Client;  
// use serde::Deserialize;  
// use std::error::Error; 

use serde::{
    Serialize,Deserialize
};

// extern crate serde_derive;


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


// 定义一个函数 read_lines，参数为一个文件名，返回一个 io::Result<io::Lines<io::BufReader<File>>>，这是一个读取器，用于逐行读取文件内容。
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<std::path::Path>, {
    // 打开指定的文件，如果出现错误则返回错误，否则继续执行下一行代码。
    let file = File::open(filename)?;
    // 使用 BufReader 来创建一个新的读取器，它可以从 file 中读取数据，并且可以一次读取多行。然后将这个读取器包装成 Ok 类型的结果。
    Ok(io::BufReader::new(file).lines())
}

fn file_input(path:String) ->Vec<String> {
    let mut vector:Vec<String>=Vec::new();
    if let Ok(lines) = read_lines(path) {
        // 对于每一行，进行处理
        for line in lines {
            // 再次使用 if let 语句处理每一行的读取结果
            if let Ok(data) = line {
                // 打印每一行的内容
                // println!("{}", data);
                vector.push(data);
            }
        }
    } else {
        // 如果读取文件失败，打印错误信息
        println!("Failed to read file");
    }

    // for item in &vector {
    //     println!("{}",item);
    // }

    vector
}

fn type_input(input_show:String,vector:Vec<String>) -> String{
    print!("{}", input_show);
    let mut result:String=String::new();
    loop{        
        let mut text=String::new();
        std::io::stdout().flush().unwrap();// 确保提示信息立即显示
        std::io::stdin().read_line(&mut text).expect("failed to read line.");

        if vector.contains(&text.trim().to_string()){
            result=text.trim().to_string();   //返回移除前导和尾随空格的字符串切处
            // if input_show.contains("省份"){
            //     println!("找到了省份：{}",result);
            // } else {
            //     println!("找到了城市：{}",result);
            // }
            
            break;
        } else {
            if input_show.contains("省份"){
                print!("对不起，没有省份的名单，请重新输入：");
            } else {
                print!("对不起，没有城市的名单，请重新输入：");
            }
        }        
    }
    result
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut vector_province:Vec<String>=Vec::new();
    let mut vector_city:Vec<String>=Vec::new();

    let path:String=format!("../placename/province.txt");

    vector_province=file_input(path);

    let mut province=String::new();   

    province=type_input("请输入省份：".to_owned(),vector_province);

    let path=format!("../placename/{}.txt",province);

    vector_city=file_input(path);

    let mut city=String::new();

    city=type_input("请输入城市：".to_owned(),vector_city);
    
    // println!("省份：{0}，城市：{1}。",province,city);

    let url=format!("https://wis.qq.com/weather/common?source=pc&city={}&province={}&weather_type=observe",city,province);

    let client = reqwest::Client::builder().build()?;

    let request = client.request(reqwest::Method::GET, url);

    let response = request.send().await?;
    let body = response.text().await?;

    // println!("{}", body);

    let model:WeatherReport=serde_json::from_str(&body).unwrap();
    
    println!("{}{}的天气是：{}",province,city,model.data.observe.weather);
    println!("{}{}的温度是：{}",province,city,model.data.observe.degree);

    Ok(())

}

// {
//     "data":
//     {
//         "observe":
//         {
//             "degree":"15",
//             "humidity":"20",
//             "precipitation":"0",
//             "pressure":"1012",
//             "update_time":"202403111650",
//             "weather":"多云",
//             "weather_bg_pag":"",
//             "weather_code":"01",
//             "weather_color":null,
//             "weather_first":"",
//             "weather_pag":"",
//             "weather_short":"多云",
//             "weather_url":"",
//             "wind_direction":"8",
//             "wind_direction_name":"北风",
//             "wind_power":"5-6"
//         }
//     },
//     "message":"OK",
//     "status":200
// }