
fn add(num1:f64, num2:f64) -> f64{
    num1 + num2
}

fn subtract(num1:f64, num2:f64) -> f64{
    num1 - num2
}

fn multiply(num1:f64, num2:f64) -> f64{
    num1 * num2
}

fn divide(num1:f64, num2:f64) -> f64{
    num1 / num2
}

fn main() {
    let numA = add(5.0, 10.0);
    let numS = subtract(5.0, 10.0);
    let numM = multiply(5.0, 10.0);
    let numD = divide(5.0, 10.0);

    println!("{} | {} | {} | {}", numA, numS, numM, numD);
}
