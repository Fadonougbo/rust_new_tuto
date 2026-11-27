use std::io::ErrorKind::Other;

use rand::RngExt;



#[derive(Debug)]
enum  UserEnum {
    Age,
    Name,
    Ville(String),
    Move {x:i32}
}


impl UserEnum {

    fn get_age(&self,a:i32){
        println!("{a}");
    }
    
}



fn main() {

    let age:Option<i32>=None;

//    let xx= age.unwrap(); // 19

        // EN cas de none unwrap renvoie une panic

        let x=190;

       let y= match x {
            1=>1,
            rest=>rest
        };

      let xx= age.unwrap_or(30);

     dbg!(y);

}
