



fn main() {

    let vec:Vec<i32>=Vec::from([20,40,20]);

    for x in vec.iter() {
        println!("{x}");
    }

    let path=Path::new("helper.txt");

    let f=File::open(path)?;



     dbg!(f);

}
