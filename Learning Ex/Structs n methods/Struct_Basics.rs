
#[derive(Debug)]

struct Shuttle{
    name: String,
    crew_size: u8,
    propellant: f64
}

fn main(){
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };
    println!("Vehicle is {:?}", vehicle);

    vehicle.name = String::from("Atlantis");
    println!("Vehicle is {:?}", vehicle);

    let vehicle2 = Shuttle {
        ..vehicle
    };
    println!("Vehicle is {:?}", vehicle2);

}

