use std;

fn demo2(){
    let mut parse = "   ";
    let mut parse = parse.len();
    let p = " ";
    let p = p.len();
    println!("{} {}",parse,p);
    /*let guess:i32 = match "32d".parse(){
        Ok(num) => num,
        Err(_)  => {
            println!("Not a number!");
        }
    };*/
    //tuple
    let tuple:(i32,f64,bool)=(3,3.2,false);
    let (x,y,z)=tuple;
    println!("{} {}",z,tuple.0);
    println!("{}", tuple.1);
    //array
    let arr:array=[1,2,3,4];
    let arr1=arr[0];
    let arr_last = arr[];
}

