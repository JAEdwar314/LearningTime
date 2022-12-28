fn main(){
    let a= 13;
    let b = 2.3;
    let c: f32 = 120.0;

    //Code Below
    let average = ((a as f32)+(b as f32)+(c))/ 3.0;
    let averageb = ((a as f64)+(b as f64)+(c as f64))/3.0;

    println!("{}, {}\n", average, averageb);

    let accent =  1.123456789123456789;

    assert_eq!(average, 45.1);
    assert_eq!(averageb, 45.1);

    println!("{}, {}", accent as f32, accent as f64);
}