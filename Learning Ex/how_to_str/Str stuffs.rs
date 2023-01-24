use std::io;

fn main(){
    while true{
        println!("Start");
        let mut thing1 = String::new();
        io::stdin().read_line(&mut thing1);
    
        let thing1 = thing1.trim().parse::<u32>().unwrap();
    
        println!("{}", thing1);
    }
    
}
