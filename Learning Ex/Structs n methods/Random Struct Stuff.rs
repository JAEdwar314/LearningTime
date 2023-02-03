//Design a space fleet interaction thingy
use std::io;

//first make your shuttles

#[derive(Debug)]
struct Shuttle{
    name: String,
    size: String,
    max_fuel: f64,
    current_fuel: f64,
    fillable: bool
}
impl Shuttle{
    fn get_name(&self) -> &str{
        &self.name
        //referenced to itself here because it is a String which cannot be borrowed without changing ownership
    }
    fn can_be_filled(&mut self) -> bool{
        //not refernced because we are trying to modiy the value in the self-given input
        if &self.current_fuel >= &self.max_fuel{
            if &self.current_fuel > &self.max_fuel{
                self.current_fuel = self.max_fuel;
            }
            self.fillable = false;
            println!("Not Fillable, Fuel AMT: {}", &self.current_fuel);
            self.fillable
        }
        else if &self.current_fuel < &self.max_fuel{
            self.fillable = true;
            println!("Fillable");
            self.fillable
        }
        else{
            return false;
        }
    }
    fn rename_shuttle(&mut self){
        println!("What would you like to name your shuttle?");
        let NewName = get_Input();
        self.name = NewName;
    }


}

fn get_Input() -> String{
    let mut BaseStr = String::new();
    io::stdin().read_line(&mut BaseStr).expect("Type in a word NIMROD!");
    let BaseStr = BaseStr.trim().parse::<String>().unwrap();
    BaseStr

}


fn main(){
    let mut Meximus = Shuttle{
        name: String::from("Meximus"),
        size: String::from("Large"),
        max_fuel: 10000.0,
        current_fuel: 500000.0,
        fillable: true
    };

    Meximus.can_be_filled();
    println!("{:?}", Meximus);

    Meximus.rename_shuttle();
    println!("{:?}", Meximus);


}
