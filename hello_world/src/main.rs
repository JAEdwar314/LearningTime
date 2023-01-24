use std::env;
use std::fs;


fn main(){

    if env::args().len() != 2{
        println!("Program Needs Only 2 Arguments");
        return;
    }
    //Set order of args to be 1. File you want to search 2. Name that you want to search

    let GiveList = 





    for (index, arg) in env::args().enumerate(){
        println!("arg {} = {}", index, arg);
    }


}

/* Input is going to be:.

arg 0 = /workspaces/LearningTime/hello_world/target/debug/hello_world
arg 1 = roster.txt
arg 2 = michael

*/
