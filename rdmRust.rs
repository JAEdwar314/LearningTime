use std::io;

fn main(){
    let apple = String::from("apple");
    let orange = String::from("orange");

    println!("Apple are not oranges {}", apple.eq(&orange));

    println!("Apples are apples {}", apple.eq(&apple));
}