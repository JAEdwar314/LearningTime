fn main(){

    let mut planet = "earth";

    {
        //Without shadowing the original variable it modifies the original data
        println!("Current Planet = {}", planet);
        planet = "Ur Mom";
        println!("Current Planet = {}", planet);
    }
    
    println!("Current Planet = {}", planet);
    println!("_______________________");
    {
        //By shadowing it weill create a temporary variable only for use within the brackets
        println!("2. Current Planet = {}", planet);
        let planet = 4;
        println!("2. Current Planet = {}", planet);
    }
    println!("2. Current Planet = {}", planet);
}

//Know that for ownership the stack only data types will just automatically clone the value
//Anything that lives on the HEAP will transfer the ownership when another car is assigned. 