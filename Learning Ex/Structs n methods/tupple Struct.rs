struct Color(u8, u8, u8); //RGB
struct Point(u8, u8, u8); //XYZ


fn get_y(p: Point) -> u8{
    p.1
}

fn main(){
    let red = Color(255, 0, 0);
    let coord = Point(5, 10, 15);

    let y = get_y(coord);

    println!("First val is {}", red.0);
    println!("First val is {}", y);

}