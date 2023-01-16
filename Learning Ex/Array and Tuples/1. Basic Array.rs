//Working with 1D arrays

fn main(){

    let letters = ['a', 'b', 'c']; 
        //Due to how rust works, if it is mutable this array can only have it's values changed to other Chars
    let first_letter = letters[0];
    println!("First letter is {}", first_letter);

    let numbers: [i32; 5]; 
        //First val is to assign the type of values that will be in the array, the "5" specifies length of the array
    numbers = [0; 5];
        //This will initialize the array to be filled with "0" for the first 5 spots

    let index: usize = numbers.len();
        //the len funstion returns Usize due to the proccessor, if x86 then it will return 4byte int, if x64 it will return 8byte int

    println!("The Last Num = {}", numbers[(index-1)]);

    let VarNums: [i32; 10];
    VarNums = [1; 10]; //Try changing 10 to anything else
        //The Array must be initialized all at once or otherwise it will give you an error about the array filler not being enough
    println!("First of VarNums = {}", VarNums[0]);
}