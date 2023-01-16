//Basic function use


fn main(){
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("{}°\nTest Passed", fahrenheit_temp);

}

fn celsius_to_fahrenheit(TempIn: f64) -> f64{
    let fahrenheit = (TempIn * 1.8) + 32.0;
    return fahrenheit

}