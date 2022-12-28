fn main(){   
    let c_input = 23.0;
    let f_out:f64 = CTF(c_input);
    assert_eq!(f_out, 73.4);
    println!("{} Degrees C = {} in F", c_input, f_out);
}

fn CTF(CIN:f64) -> f64{
    return (1.8 * CIN) + 32.0;
}