//Working with Multidimensional arrays
use std::io;
fn main(){
    let parking_lot = [[1,2,3],  //[0] 0-2
                        [4,5,6]]; //[1] 0-2
    //The lists inside the array must all be the same length / size, you cannot have varying lengths of lists
        //inside a multidimensional array
    let number = parking_lot[1][2];
    println!("Current Spot = {}", number);

    let garage:[[[i32; 100];20];5];

    let manual = [[1,2,3], [4,5,6]];
    let manual3D = [[[1,2,3], [4,5,6]], 
                    [[7,8,9], [10,11,12]]];

    let spot = manual[1][2]; //Should be 3
    let spot3D = manual3D [0][1][2]; //Should also be 3

    println!("1 Layer Garage = {}", spot);
    println!("3D Garage = {}", spot3D);

    

    parking();

}

fn parking(){
    let mut PInput = String::new();

    let BigGarage = [[[1,2,3,4,5] ,      [6,7,8,9,10]], //Floor 1 ind 0
                    [[21,22,23,24,25] , [26,27,28,29,30]], //Floor 2 ind 1
                    [[31,32,33,34,35] , [36,37,38,39,40]]]; // Flor 3 ind 2

    let CxSpot = BigGarage [2][1][3];
    println!("Where would you like to park? \n Spaces 1-10, 21-30, 31-40");
    println!("Your reserved spot is {}", CxSpot);


}