/*
What is your goal here?

Why dont you make a simple api that allows someone to type a command and d something with a little ship.

1. Make what a spaceship is
2. Make whatever a space station will be
3. Allow the palyer to
    a. Buy fuel
    b. interact with shop
    c. manually request to leave

4. Then build muliple stations for tfor player to go to and do the same thing.
*/

/*
A Spaceship:
    All ships ought to be named
    have a pilot count plus passengers
    fuel tank for a gas ship, battery / reactor for electric based ship
    adapter size would be the type of gas nozel used
    Every ship has to store supplies
    Most ships should be abe to defend themselves or have sapce to isntlal add'l modules


*/
use std::io;


struct GasSpaceship{
    Name: String, //Create a welcome process that lets the palyer name their ship
    Passengers: u16, //This does not include pilot seats
    ReqPilots: u8, //Required Pilots for ship


    CurrentFuel: u32, //A normal spaceship usually uses about 900,000 gallon of fuel 2/2/2023
    MaxFuel: u32, //YOur ship has to ahve a way to mvoe afterall
    FuelAdapterSize: u8, //This will help late rto detemrine spaceship landing pad size by basing it off of the cliber of the fuel adapter
                        //Maybe add ships with Reverible adapters later?

    CargoSpace: u32, // U32 to allow massive ships wihtout another struct
    ThrusterSpace: u16,
    WeaponSpace: u16,
    MiscSpace: u16, //This will be the unused Thruster + Weapon Space for add'l modules.

}

impl GasSpaceship{
    fn new(Name: String, Passengers: u16, ReqPilots: u8, CurrentFuel: u32, MaxFuel: u32, FuelAdapterSize: u8, CargoSpace: u32, ThrusterSpace: u16, WeaponSpace: u16, MiscSpace: u16) -> GasSpaceship{
        GasSpaceship{
            Name: Name,
            Passengers: Passengers,
            ReqPilots: ReqPilots,
            CurrentFuel: CurrentFuel,
            MaxFuel: MaxFuel,
            FuelAdapterSize: FuelAdapterSize,
            CargoSpace: CargoSpace,
            ThrusterSpace: ThrusterSpace,
            WeaponSpace: WeaponSpace,
            MiscSpace: MiscSpace
        }
    }
}

struct Station{
    Name: String,

    E_Fuel: bool,
    E_Rate: u128,

    G_Fuel: bool,
    Current_Fuel: u128,
    Max_Fuel: u128,

    Max_AdapterSize: u8, //Max adapter size that can be supported

    Has_Shop: bool,
    Shop_Amount: u32,

    Has_Repair: bool,
    Repair_Shops: u32

}

impl Station{
    fn new(Name: String, E_Fuel: bool, E_Rate: u128, G_Fuel: bool, Current_Fuel: u128, Max_Fuel: u128, Max_AdapterSize: u8, Has_Shop: bool, Shop_Amount: u32, Has_Repair: bool, Repair_Shops: u32) -> Station{
       Station {Name: Name,
        E_Fuel: E_Fuel,
        E_Rate: E_Rate,
        G_Fuel: G_Fuel,
        Current_Fuel: Current_Fuel,
        Max_Fuel: Max_Fuel,
        Max_AdapterSize: Max_AdapterSize,
        Has_Shop: Has_Shop,
        Shop_Amount: Shop_Amount,
        Has_Repair: Has_Repair,
        Repair_Shops: Repair_Shops
       }
    }
}


fn main(){
    let mut PlayerShip: GasSpaceship;
    let mut CheckList = (false, true, true, true); //1. Start_GameCheck
    //           A list of checks for if a function has ran or not
    (PlayerShip, CheckList.0) = Start_Game();
    println!("It is {}", CheckList.0);

    println!("Space Name = {}", PlayerShip.Name)

    
}

fn Start_Game() -> (GasSpaceship, bool){
    
    let TestShip = GasSpaceship::new(
        "Johnson".to_string(),
        3,
        1,
        500,
        1000,
        1,
        50,
        5,
        5,
        5
    );
    println!("Give me a num please!");
    let guess = GetNumber();
    println!("You guessed {}", guess);
    let TestStation = Station::new(
        "Mother Station".to_string(),

        true,
        500,

        true,
        250,
        10000,

        5,

        true,
        2,

        true,
        1
    );
    println!("{} is parked at {}", TestShip.Name, TestStation.Name);
    let Maxie = Setup_GasSpaceShip();
    //This is basicaly just test stuff and can be deleted at anytime
    (Maxie, true)
}

fn Setup_GasSpaceShip() -> GasSpaceship{
    let mut CorrectShip = false;
    let mut CreatedShip = GasSpaceship::new("Apple".to_string(), 0,0,0,0,0,0,0,0,0);
    while !CorrectShip{
        println!("Lets create a spaceship");

        println!("What would you name your ship?");
        let mut Name = GetString();

        println!("How many passengers?");
        let mut Passengers = GetNumber() as u16;

        println!("How many pilots are needed?");
        let mut ReqPilots = GetNumber() as u8;

        println!("Gas Fuel Tank Size?");
        let mut MaxFuel = GetNumber() as u32;

        println!("How Much Gas Filled?");
        let mut CurrentFuel = GetNumber() as u32;

        println!("What class of ship?");
        let mut FuelAdapterSize = GetNumber() as u8;

        println!("Max Cargo Space:");
        let mut CargoSpace = GetNumber() as u32;

        println!("Max Thrusters:");
        let mut ThrusterSpace = GetNumber() as u16;

        println!("Max Weapons:");
        let mut WeaponSpace = GetNumber() as u16;

        println!("Extra Install Spaces:");
        let mut MiscSpace = GetNumber() as u16;

        //let mut AllVals = (Name, Passengers, ReqPilots, CurrentFuel, MaxFuel, CurrentFuel, FuelAdapterSize, CargoSpace, ThrusterSpace, WeaponSpace, MiscSpace);

        CreatedShip = GasSpaceship::new(Name, Passengers, ReqPilots, CurrentFuel, MaxFuel, FuelAdapterSize, CargoSpace, ThrusterSpace, WeaponSpace, MiscSpace);
        println!("You have setup Spaceship:\nName: {}\nRequired Pilots: {}\nPassegenger Count: {}\nTank Size: {} \nClass: {}\nCargo Space: {} \nInstall Space: y{}",
        CreatedShip.Name, CreatedShip.Passengers, CreatedShip.ReqPilots, CreatedShip.MaxFuel, CreatedShip.FuelAdapterSize, CreatedShip.CargoSpace, CreatedShip.MiscSpace);
        println!("Is this correct?");
        CorrectShip = getYorN();
    }
    CreatedShip
    
}

//Below Is JUST for getting an input and parsing to an expected val

fn GetNumber() -> u128{
    let mut Response = String::new();
    io::stdin().read_line(&mut Response);
    let Response = Response.trim().parse::<u128>().unwrap();
    Response
}

fn GetString() -> String{
    let mut Response = String::new();
    io::stdin().read_line(&mut Response);
    let Response = Response.trim().parse::<String>().unwrap();
    Response
}

fn getYorN() -> bool{
    
    let mut answer = String::new();
    let mut report: bool = false;
    io::stdin().read_line(&mut answer).expect("Yes or No");
    let answer = answer.trim().parse::<String>().unwrap();

    if answer.eq("Yes") || answer.eq("yes") || answer.eq("Y") || answer.eq("y"){
        report = true;
        //println!("You answered {} which == {}", answer, repo);
    }
    else if answer.eq("no") || answer.eq("No")|| answer.eq("N") || answer.eq("n"){
        report = false;
        //println!("You answered {} which == {}", answer, repo);
    }
    
    report
}
