extern crate rand;      

fn main() {
    temperature_transformation(50,0);
}

fn temperature_transformation(count:i32,type:i32){
    if type == 0{
        println!("{}°C", 5/9(count-32));
    }else{
        println!("{}°F", 9/5*count+32);
    }
}

fn fibonacci (n:i32){
    if n <= 0{
        1
    }else if n == 1{
        1
    }else {
        fibonacci(n)-fibonacci(n-1);
    }
}
fn christmas(){
    for i in (1..13) {
        println!("On the 1st day of Christmas");
        println!("my true love sent to me:");
        for j in (1..13){

        }
        println!("A Partridge in a Pear Tree");
    }
}