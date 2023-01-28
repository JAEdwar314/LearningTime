use std::env;
use std::fs;
use std::io;
use std::io::Write;


fn main(){
    let mut openedFile = String::new();
    let mut Search = String::new();

    if env::args().len() != 3{
        println!("Program Only Needs 2 Arguments");
        return;
    }
    openedFile = env::args().nth(1).unwrap();
    Search = env::args().nth(2).unwrap();
    processENV(&openedFile, &Search)
}

fn processENV(desiredFile: &str, keyword: &str){
    let contents = fs::read_to_string(desiredFile).unwrap();
    let mut verifyName:bool = false; //Default set to false


    for (index, line) in contents.lines().enumerate(){
        if keyword == line{
            println!("Astronaut {} is on line {}", line, index+1);
            verifyName = true; 
            return;
        }
        
    }
    if !verifyName{
        println!("{} was not found in {}", keyword, desiredFile);
        println!("Would you like to too add this name to the list?");
        let mut addCheck = getYorN();
        if addCheck{
            addName(&keyword, &desiredFile);
            println!("{} added to the list!", keyword);
        }
        else if !addCheck{
            println!("They are not an astronaut... YET")
        }

    }
    
}

fn addName(nameToAdd: &str, fileUsed: &str){
    let mut file = fs::OpenOptions::new().append(true).open(fileUsed).unwrap();
    let mut makeNewLine = String::from("\n");
    makeNewLine.push_str(nameToAdd);

    file.write(makeNewLine.as_bytes());
}

fn getYorN() -> bool{
    //Put this in anywhere to ask the player Y or N
    //println!("\n!! Would you like to play a game? !!");
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

 //Set order of args to be 1. File you want to search 2. Name that you want to search



/* Input is going to be:.

arg 0 = /workspaces/LearningTime/hello_world/target/debug/hello_world
arg 1 = roster.txt
arg 2 = michael

*/
