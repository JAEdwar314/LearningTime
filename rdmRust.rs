use std::io;

/*
1st. Make a terminal engine that allows the player to interact with the world (do not build a coordinate grid)
2nd. Allow the player to create a type of object in the game
3rd. Allow the player to have objects interact with each other
*/
struct Spaceship{
    Name: String,
    Type: u8,
    crew_size: u16,
    cargo_space: u16,
    max_fuel: u64,
    current_fuel: u64,
}
impl Spaceship{
    fn new(N: String, T: u8, CRS: u16, CAS: u16, MF: u64, CF: u64) -> Spaceship{
        Spaceship{
            Name: N,
            Type: T,
            crew_size: CRS,
            cargo_space: CAS,
            max_fuel: MF,
            current_fuel: CF
        }
    }
}


fn main(){
    //First you need to create the intro and allow the player to create a spaceship
    let mut CargoHaulerA1 = Spaceship::new("Freights United AA".to_string(), 1, 3, 50, 5000, 500);
    let mut PeopleCruiserA1 = Spaceship::new("Skyline Scrapers".to_string(), 1, 15, 10, 5000, 500);
    let mut FightDealerA1 = Spaceship::new("Battle Block".to_string(), 1, 2, 20, 5000, 500);
    let mut Starter_Ships = (CargoHaulerA1, PeopleCruiserA1, FightDealerA1);
    println!("Welcome to Spacer Space");
    println!("Would you like to play a game?");
    if getYorN(){
        start_game(Starter_Ships);
    }
    else{
        println!("You sure you dont wanna play?");
    }


               
}
fn start_game(*args) {
    println!("Starting Game");
    println!("Welcome to Spacers \nChoose your ship to get started!");
    println!(*args.0)

}



fn get_Number() -> i32{
    let mut Number = String::new();
    io::stdin().read_line(&mut Number).expect("How? Just How??");
    let Number = Number.trim().parse::<i32>().unwrap();
    Number
}

fn get_Input() -> String{

    let mut Reply = String::new();
    io::stdin().read_line(&mut Reply).expect("How? Just How??");
    let Reply = Reply.trim().parse::<String>().unwrap();
    Reply
}

fn getYorN() -> bool{
    
    println!("\n!! Would you like to play a game? !!");
    let mut answer = String::new();
    let mut repo: bool = false;
    io::stdin().read_line(&mut answer).expect("Yes or No");
    let answer = answer.trim().parse::<String>().unwrap();

    if answer.eq("Yes") || answer.eq("yes") || answer.eq("Y") || answer.eq("y"){
        repo = true;
        //println!("You answered {} which == {}", answer, repo);
    }
    else if answer.eq("no") || answer.eq("No")|| answer.eq("N") || answer.eq("n"){
        repo = false;
        //println!("You answered {} which == {}", answer, repo);
    }
    
    repo
}

