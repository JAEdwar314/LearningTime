use std::env;
use std::fs;
use std::ops::Index;


fn main(){
    let mut openedFile = String::new();
    let mut Search = String::new();

    if env::args().len() != 3{
        println!("Program Only Needs 2 Arguments");
        return;
        //for (index, arg) in env::args().enumerate(){
        //let argA = env::args().nth(index).unwrap();
    }
    openedFile = env::args().nth(1).unwrap();
    Search = env::args().nth(2).unwrap();
    processENV(&openedFile, &Search)
}

fn processENV(desiredFile: &str, keyword: &str){
    let contents = fs::read_to_string(desiredFile).unwrap();
    
    for (index, line) in contents.lines().enumerate(){
        if keyword == line{
            println!("Astronaut {} is on line {}", line, index+1);
            return;
        }
    }
}

 //Set order of args to be 1. File you want to search 2. Name that you want to search



/* Input is going to be:.

arg 0 = /workspaces/LearningTime/hello_world/target/debug/hello_world
arg 1 = roster.txt
arg 2 = michael

*/
