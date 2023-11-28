use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    let path = Path::new("/workspaces/LearningTime/FileOps/TestFolder_1/info1.txt");
    let display = path.display();

    println!("{}", &display);

    let mut file = match File::open(&path){
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut info = String::new();
    match file.read_to_string(&mut info){
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, info),
    };


}
