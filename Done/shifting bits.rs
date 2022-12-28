fn main() {
    let mut a = 0b_10001111;
    let mut b = 0b_10000000;
    let mut c = 0b_10000011;
    let mut d = 0b_10001100;

    let x = c << 1; // ou Can also do >> to make bits shifts the other way
    let y = c << 2;
    let z = c << 3;

    println!("Shiift 0: {} Shifted 1: {} Shifted 2: {} Shifted 3: {}",(c), (x), (y), (z));

    println!("");


}