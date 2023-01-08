//Memory Management via Ownership, and strings

fn main(){   
    let a = true;
    let mut Outisde: String;
    if a == true{
        let Inside = String::from("I'm inside the if statement 1.");
        println!("{}", Inside);
        Outisde = Inside;
        println!("{}", Outisde);
        // This first create a string var that then tx's ownership to outside and then prints it
    }

    if a == true{
        let Inside = String::from("I'm inside the if statement 2.");
        Outisde = Inside.clone();
        println!("{}", Inside);
        //This initiallized Outside like before but it prints after ownership has been changed from "Inside"
    }

    if a == true{
        let mut Inside = String::from("BooHoo");
        println!("Muahahaha {}", Inside);
        Inside.clear();
        println!("Muahahaha {}", Inside);
    }
    
    if a == true{
       let mut inside = 25;
       println!("Inside int {}", inside);
       let outside = inside;
       inside = 400;
       println!("Inside: {}, Outside: {}", inside, outside);
    }

}