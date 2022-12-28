fn main(){
    let a = true;
    let b = false;

    println!("A is {} | B is {}", a, b);
    println!("Are A and B the same? {}", a & b);
    println!("Is A leat A or B true? {}", a | b);
    println!("Is A OR B true but not both? {}", a ^ b); 
}