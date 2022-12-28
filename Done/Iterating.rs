
// Making a quick calc

fn main(){   
    let lista = ["H","E","L","L","O"];

    for (index, &item) in lista.iter().enumerate(){
        if item == "E"{
            break;
        }
        println!("Index {} = {}", index, item);
    }
//.enumerate is a fucntion of .iter, if you want to print an index you MUST do .iter().enumerate()

}