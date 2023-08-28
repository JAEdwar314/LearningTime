#[derive(Debug)]
enum shape{
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}



fn main(){
    let my_shape = shape:: Rectangle(1.2, 3.4);
    println!("My Shape is {:?}", my_shape);

    match my_shape{
        shape::Circle(R) => println!("Radius = {}", R),
        shape::Rectangle(w, h) => println!("Width = {}, Height = {}", w, h),
        shape::Triangle(a, b, c) => println!("Sides are {}, {}, {}", a,b,c)
    }

    println!("Done")
}
