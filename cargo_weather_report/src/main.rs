#![allow(unused_assignments)]

use std::io::Write;
// use std::io::prelude::*;
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;


// 定义一个函数 read_lines，参数为一个文件名，返回一个 io::Result<io::Lines<io::BufReader<File>>>，这是一个读取器，用于逐行读取文件内容。
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    // 打开指定的文件，如果出现错误则返回错误，否则继续执行下一行代码。
    let file = File::open(filename)?;
    // 使用 BufReader 来创建一个新的读取器，它可以从 file 中读取数据，并且可以一次读取多行。然后将这个读取器包装成 Ok 类型的结果。
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // Create a path to the desired file
    // let path = std::path::Path::new("province.txt");
    // let display = path.display();

    // // Open the path in read-only mode, returns `io::Result<File>`
    // let mut file = match std::fs::File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // // Read the file contents into a string, returns `io::Result<usize>`
    // let mut text = String::new();
    // match file.read_to_string(&mut text) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => print!("All Province:\n{}", text),
    // }

    // `file` goes out of scope, and the "hello.txt" file gets closed


    // let text=std::fs::read_to_string("./province.txt").unwrap();
    // println!("{}",text);

    let mut vector_province:Vec<String>=Vec::new();
    let mut vector_city:Vec<String>=Vec::new();


    if let Ok(lines) = read_lines("../placename/province.txt") {
        // 对于每一行，进行处理
        for line in lines {
            // 再次使用 if let 语句处理每一行的读取结果
            if let Ok(data) = line {
                // 打印每一行的内容
                // println!("{}", data);
                vector_province.push(data);
            }
        }
    } else {
        // 如果读取文件失败，打印错误信息
        println!("Failed to read file");
    }

    // for item in &vector_province {
    //     println!("{}",item);
    // }

    let mut province=String::new();   

    print!("请输入省份：");
    loop{        
        let mut text=String::new();
        std::io::stdout().flush().unwrap();// 确保提示信息立即显示
        std::io::stdin().read_line(&mut text).expect("failed to read line.");

        if vector_province.contains(&text.trim_end().to_string()){
            province=text.trim_end().to_string();
            println!("找到了省份：{}",province);
            break;
        } else {
            print!("对不起，没有省份的名单，请重新输入：");
        }        
    }
    let path=format!("../placename/{}.txt",province);

    if let Ok(lines) = read_lines(path) {
        // 对于每一行，进行处理
        for line in lines {
            // 再次使用 if let 语句处理每一行的读取结果
            if let Ok(data) = line {
                // 打印每一行的内容
                // println!("{}", data);
                vector_city.push(data);
            }
        }
    } else {
        // 如果读取文件失败，打印错误信息
        println!("Failed to read file");
    }

    // for item in &vector_city {
    //     println!("{}",item);
    // }


    let mut city=String::new();
    
    print!("请输入城市：");
    loop {
        let mut text=String::new();
        std::io::stdout().flush().unwrap();// 确保提示信息立即显示  
        std::io::stdin().read_line(&mut text).expect("failed to read line.");

        if vector_city.contains(&text.trim_end().to_string()){
            city=text.trim_end().to_string();
            println!("找到了城市：{}",city);
            break;
        } else {
            print!("对不起，没有城市的名单，请重新输入：");
        }
    }
    
    println!("省份：{0}，城市：{1}。",province.trim(),city.trim());
}