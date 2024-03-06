use std::io::Write;

fn main() {
    let mut province=String::new();
    let mut city=String::new();

    print!("请输入省份：");
    std::io::stdout().flush().unwrap();// 确保提示信息立即显示  
    std::io::stdin().read_line(&mut province).expect("failed to read line.");
    
    print!("请输入城市：");
    std::io::stdout().flush().unwrap();// 确保提示信息立即显示  
    std::io::stdin().read_line(&mut city).expect("failed to read line.");

    println!("省份：{0}，城市：{1}。",province.trim(),city.trim());
}
