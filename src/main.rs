extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜猜看！数字是多少？");
    println!("请输入你猜的数字：");
    let guessNo = rand::thread_rng().gen_range(0, 101);
    loop{
        let mut number = String::new();
        io::stdin().read_line(&mut number)
            .expect("读取失败！");
        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };
        println!("你猜的数字是：{}",number);

        match number.cmp(&guessNo){
            Ordering::Less      => println!("低了!"),
            Ordering::Greater   => println!("高了！"),
            Ordering::Equal     => {
                println!("猜中了，你赢了！");
                break;
            }
        }
    }
    
}
