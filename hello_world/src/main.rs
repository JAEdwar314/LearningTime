//Memory Management via Ownership, and strings

fn main(){   
    let a = true;
    let mut Outisde: String;
    if a == true{
        let Inside = String::from("I'm inside the if statement");
        println!("{}", Inside);
        Outisde = Inside;
        println!("{}", Outisde);
        // This first create a string var that then tx's ownership to outside and then prints it
    }

    if a == true{
        let Inside = String::from("I'm inside the if statement");
        Outisde = Inside.clone();
        println!("{}", Inside);
        //This initiallized Outside like before but it prints after ownership has been changed from "Inside"
    }

    


}