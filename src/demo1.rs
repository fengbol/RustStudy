use std::io;

fn main(){
    println!("猜猜看！数字是多少？");
    println!("请输入你猜的数字：");

    let mut number = String::new();
    io::stdin().read_line(&mut number)
        .expect("读取失败！");
    println!("你猜的数字是：",number);
}