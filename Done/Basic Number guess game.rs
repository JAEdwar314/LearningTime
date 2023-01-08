use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    let mut LuckyNum = rand::thread_rng().gen_range(1..=100);
    let mut PGuess = String::new();
    let mut Attempts = 0;

    println!("Welcome to the GAME");
    loop{
        println!("Whats my lucky number?? {}", LuckyNum);

        io::stdin()
            .read_line(&mut PGuess)
            .expect("Why haven't you guessed yet?");
        
        let PGuess: u32 = PGuess.trim().parse().expect("Thats not a number nimrod");
        println!("You guessed {}", PGuess);

        match PGuess.cmp(&LuckyNum){
            Ordering::Less => println!("Which is incorrect!"),
            Ordering::Greater => println!("Too large like your forehead!!"),
            Ordering::Equal => {
                println!("Good job, you're kinda smart!");
                break;
            }
        }


    }
}