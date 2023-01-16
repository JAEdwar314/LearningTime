//lists inside of a tuple

fn main(){
    let Tupler = ([1,2,3,4,5,6,7], ['a', 'b', 'c', 'd', 'e']);
    let TuplerB = ([11,22,33,44,55,66], ['f', 'g', 'h', 'i', 'j']);

    let lister = [[Tupler], [TuplerB]];

    println!("Booo {:?}", lister);

    //if shoving tuples into a list it is applicable to list rules but each tuple is 1 value 
}