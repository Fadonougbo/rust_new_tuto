use rand::RngExt;


#[derive(Debug,Clone, Copy)]
struct  User{

}

fn main() {

    let text=String::from("Salut c'est john doe");

    let mut i=0;

    let bytes=text.as_bytes();

    // Quand on parcourt une référence avec for, chaque élément est aussi une référence


    let arr=[30,20,8,6];

    let xx=arr.iter().enumerate();

    for (i,element) in xx {
        
        println!("{element} {i}");
    }

    let v=b't';




    // for byte in bytes {
        
    //     if *byte == b' ' {
    //         break;
    //     }

    //     i+=1;

    // }

    // dbg!(i);
  
}
