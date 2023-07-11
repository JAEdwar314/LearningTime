fn main(){
    let my_number = 22u8;

    let result = match my_number{
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => {
            println!("{} is not an option", my_number);
            "something else"
        }

    };

    
    println!("Resulting in {}", result)
}