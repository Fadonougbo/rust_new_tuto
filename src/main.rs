use rand::RngExt;


#[derive(Debug,Clone, Copy)]
struct  User{

}

fn main() {

    let xx=String::from("œœ");


    let d=xx.len(); //2

    let d=xx.chars().count(); //1

    let mut n = 10;

    let m = &mut n;
    *m += 5;           // modification
    println!("{n}");   // 15

    // println!("{} ",xx+"text");

    // dbg!(a,b);
  
}
