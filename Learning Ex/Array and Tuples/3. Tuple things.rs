fn main(){
    let mut stuff: (u8, f32, char, &str) = (10, 3.14, 'c', "Boooo");
    stuff.0 += 5;

    println!("{}", stuff.0);

    let (Apple, Boxes, Cars, Dogs) = stuff;

    println!("Apples {} | Boxes {} | Cars {} | Dogs {}", Apple, Boxes, Cars, Dogs);


}