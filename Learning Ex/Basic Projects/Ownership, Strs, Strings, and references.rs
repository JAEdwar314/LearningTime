//Practicing ownership, references, and slices


fn main(){

    let string1 = String::from("My momma aint tell me nothin");

    let string2 = String::from("mamma mia");

    let string3 = String::from("U H O H _ L E F T _ T H E _ S P A C E R _ O N");
    
    let string4 = String::from("I aint doing anymore of this shi");


    let Blub = NoMoreSpaces(&string3); //Pass in a reference to the string to avoid ownership issues
                                                          //Retuned as a string to avoind dangling references
    println!("{}", Blub)

}

fn NoMoreSpaces(BaseString: &str) -> String{
    let mut processedString = String::new();

    for (index, character) in BaseString.chars().enumerate(){
        if character != ' '{
            processedString.push(character);
            //BaseString gets converted with .chars() that way the for loop can use var "character" to seach each byte of data in the string.
        }
    }
    processedString //If this is returned as "&processedString" it will create a dangling refernece
}