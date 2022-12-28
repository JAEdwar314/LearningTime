
// Making a quick calc

fn main(){   
    let mut firstVal = 5.5;
    let mut secVal = 7.0;

    println!("{}", Squared(firstVal));
    println!("{}", Add(firstVal, secVal));
    println!("{}", Sub(firstVal, secVal));
    println!("{}", Multiply(firstVal, secVal));
    println!("{}", Divi(firstVal, secVal));
    
}

fn Squared(a:f64) -> f64{
    a * a
}

fn Add(a:f64, b:f64) -> f64{
    a + b
}

fn Sub(a:f64, b:f64) -> f64{
    a - b
}

fn Multiply(a:f64, b:f64) -> f64{
    a * b
}

fn Divi(a:f64, b:f64) -> f64{
    a / b
}