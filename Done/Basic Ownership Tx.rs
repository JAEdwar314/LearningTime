//Memory Management via Ownership, and strings

fn main(){ 
    let rocket_fuelC = String::from("RP-1");
    let rocket_fuelC = process_fuel(rocket_fuelC);
    println!("rocket_fuel is {}", rocket_fuelC);

}

fn process_fuel(mut propellant: String) -> String{
    println!("processing propellant {}...", propellant);
    let new_fuel = String::from("pLNG");
    new_fuel
}