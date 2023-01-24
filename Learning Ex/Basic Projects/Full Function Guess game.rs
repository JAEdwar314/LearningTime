use std::io;
use rand::prelude::*;


fn main(){
    let mut playGame = getYorN();
    
    while playGame{
        let luckyNum = rand::thread_rng().gen_range(1..=100);
        println!("My Lucky number is {}", luckyNum);
        let totalTries = getTries();
        gamePlay(luckyNum, totalTries);
        playGame = getYorN();
    }
    if !playGame{
        println!("See you around!");
    }
    
}

fn gamePlay(luckyNum: i32, totalTries: i32){
    let mut tries = 0;
    

    while tries < totalTries{
        println!("Give me a number:");
        let guess = getNumber();
        if  guess > luckyNum{
            println!("--Too High--");
            tries +=1;
            println!("-You've tries {} times so far\n", tries);
            gameFailed(tries, totalTries);
        }
        else if  guess < luckyNum{
            println!("--Too Low--");
            tries +=1;
            println!("-You've tries {} times so far\n", tries);
            gameFailed(tries, totalTries);
        }
        else if guess == luckyNum{
            println!("Good job! you figured out my lucky num is {}", luckyNum);
            tries = totalTries+1;
        }
    }

    

}
fn gameFailed(tries: i32, totaltries: i32){
    if tries >= totaltries{
        println!("!GAME OVER!");
    }
}

fn getNumber() -> i32{
    let mut guesser = String::new();
    io::stdin().read_line(&mut guesser).expect("Type in a number NIMROD!");
    let guesser = guesser.trim().parse::<i32>().unwrap();
    guesser
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

fn getTries() -> i32{
    println!("How many tries would you like?");
    let TryNum = getNumber();

    TryNum

}