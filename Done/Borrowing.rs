fn main(){

    let mut MyFuel = String::from("RP-1");
    let length = FuelChek(&MyFuel);
    
    println!("Length of Fuel is {}", length);

}

fn FuelChek(Fuel: &String) -> usize{

    println!("You're using the fuel {}", Fuel);
    let length = Fuel.len();
    length
}

//Using a reference to change a variable by "borrowing it" rather than cahnging ownership and passing it back