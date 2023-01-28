use std::env;
use std::fs;
use std::ops::Index;


fn main(){

    if env::args().len() != 3{
        println!("Program Needs Only 2 Arguments");
        return;
    }
    //Set order of args to be 1. File you want to search 2. Name that you want to search

    



    for (index, arg) in env::args().enumerate(){
        let argA = env::args().nth(index).unwrap();
        println!("{}", argA);

        while !index.eq(&0){
            if argA.eq(&"ur"){
                println!("You better not say mom! Arg: {}", argA);
                break;
            }
            else{
                println!("You're a child. Arg: {}", argA);
                break;
            }
        }
        
    }

    


}

/* Input is going to be:.

arg 0 = /workspaces/LearningTime/hello_world/target/debug/hello_world
arg 1 = roster.txt
arg 2 = michael

*/
