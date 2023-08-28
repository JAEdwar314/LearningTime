#[derive(Debug)]
enum Shape{
    Circle(f64),
    Rectangle(f64, f64),
    Triangle (f64, f64, f64)
}

impl Shape{
    fn get_parimeter(&self) -> f64{
        match self{
            Shape::Circle(r) => r*2.0*std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c
        }
    }
}

fn main(){
    let my_shape = Shape::Rectangle(1.5, 3.5);
    println!("my_shape");

    let perimeter = my_shape.get_parimeter();
    println!("The perimeter is {}", perimeter)
}