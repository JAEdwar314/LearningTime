use std::net::Shutdown;

#[allow(non_camel_case_types)]

struct Space_shuttle{
    Name: String,
    Size: u8,
    Max_Fuel: f64,
    Current_Fuel: f64
}
impl Space_shuttle{
    fn new_shuttle(Name: String, Size: u8, Max_Fuel: f64, Current_Fuel: f64) -> Space_shuttle{
        Space_shuttle { 
            Name: Name,
            Size: Size,
            Max_Fuel: Max_Fuel,
            Current_Fuel: Current_Fuel
         }
    }
}

struct Fuel_Station{
    Location: String,
    Max_Fuel: f64,
    Fuel_Rate: i32, //make sure to always compare "as f64"
    Current_Fuel: f64,
    Ship_Adapter: u8, //this will be compared to sizes 1, 2, or 3
}

impl Fuel_Station{
    fn new_station(Location: String, Max_Fuel: f64, Fuel_Rate: i32, Current_Fuel: f64, Ship_Adapter: u8) -> Fuel_Station{
        Fuel_Station{
            Location: Location,
            Max_Fuel: Max_Fuel,
            Fuel_Rate: Fuel_Rate,
            Current_Fuel: Current_Fuel,
            Ship_Adapter: Ship_Adapter
        }
    }
}


fn main(){
    let mut Meximus = Space_shuttle::new_shuttle("Meximus".to_string(), 3, 5000.0, 2500.0);

    let mut Center_Station = Fuel_Station::new_station("Milky Way".to_string(), 100000.0, 50, 50000.0, 3);

    fuel_ship(Meximus, Center_Station);
}

fn fuel_ship(mut Ship_to_Fuel: Space_shuttle, mut Station_Used: Fuel_Station){
    let mut secs_to_fill = 0;
    let start_fuel = Ship_to_Fuel.Current_Fuel;
    if Ship_to_Fuel.Size != Station_Used.Ship_Adapter{
        println!("Your ship is not compatible, please provide a Type {} Adapter", Station_Used.Ship_Adapter);
        return;
    }
    while Station_Used.Current_Fuel != 0.0 && Ship_to_Fuel.Current_Fuel < Ship_to_Fuel.Max_Fuel{
        Ship_to_Fuel.Current_Fuel += Station_Used.Fuel_Rate as f64;
        Station_Used.Current_Fuel -=  Station_Used.Fuel_Rate as f64;
        secs_to_fill +=1;
    }
    if Ship_to_Fuel.Current_Fuel > Ship_to_Fuel.Max_Fuel{
        Ship_to_Fuel.Current_Fuel = Ship_to_Fuel.Max_Fuel;
    }
    println!("It took {} secs to fill {}L", secs_to_fill, (Ship_to_Fuel.Current_Fuel - start_fuel));

}