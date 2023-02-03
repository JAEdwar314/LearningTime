use std::sync::mpsc::Receiver;

struct Rectangle{
    length: f64,
    width: f64
}

impl Rectangle{
    fn new(length: f64, width: f64) -> Rectangle{
        Rectangle{
            length: length,
            width: width
        }
    }
    fn get_area(&self) -> f64{
        self.length * self.width

    }
    fn scale(&mut self, scale:f64){
        self.length = self.length * scale;
        self.width = self.width * scale;
    }
}


fn main(){
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);

    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("test passed!");
    //Figure out why there are ownership issues here

}

//You had an issue with the ownership
//Mind you that when creating a customer struct you must treat it similar to a string NOT an integer even if there are no 
    //strings in the struct data type