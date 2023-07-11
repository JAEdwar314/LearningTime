fn main(){
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(3);
    let number = number.unwrap_or(&0) + 1; // this works because if there is "some" then it will continue with no trouble, and if "None" it will return 0 then continue onwards.

    // let number = number.unwrap() + 1; //using unwrap is discouraged for this because you can't always unwrap

    println!("number is {:?}", number);