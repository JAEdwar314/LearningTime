use std::io;
use rand::prelude::*;

fn main(){

    let mut luckyNum = rand::thread_rng().gen_range(1..=100);
    let mut tries = 0;
    let mut playerGuess: u32;

    println!("Welcome to the game!!");
    println!("Guess my lucky number! {}", luckyNum);

    while tries < 6{
        playerGuess = getInput();
        

        if playerGuess > luckyNum{
            println!("Too High");
            tries +=1;
            println!("You've tries {} times so far", tries);
        }
        else if playerGuess < luckyNum{
            println!("Too Low");
            tries +=1;
            println!("You've tries {} times so far", tries);
        }
        else if playerGuess == luckyNum{
            println!("It took you {} tries to guess {}", tries, playerGuess);
            println!("\nWould you like to play again? | 1.Yes 2.NO");
            let y = getInput();
            if y.eq(&1){
                tries = 0;
                luckyNum = rand::thread_rng().gen_range(1..=100);
                println!("Guess my lucky number! {}", luckyNum);
            }
            else if y.eq(&2){
                println!("Thanks for playing!");
                tries = 7;

            }
        }
    }
    
}

fn getInput() -> u32{
    let mut guesser = String::new();
    io::stdin().read_line(&mut guesser).expect("Type in a number NIMROD!");
    let guesser = guesser.trim().parse::<u32>().unwrap();
    guesser
}
