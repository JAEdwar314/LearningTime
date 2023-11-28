fn main(){   
    // Array Vs Tuples

    let collection = [[1,2,3], [1,2,3]];
    println!("Hello #{}", collection[0][1]);

    let list = (54, 2.31, "jack");
    println!("{}", list.2);

    let together = [[("Tuple 1", 1, 1.0)], [("Tuple 2", 2, 2.0)]];
    let val = together[0][0].0;
    println!("{} {} {}", val, together[0][0].1, together[1][0].2);
    }
/* Notes:
    1.Arrays use square brackets "[]" and 0-index notation, refer to normally as per lists in other languages (can only have the same data)
        1a. i.e. Same data includes: Only 1 actual data type in an list of array (i.e. ints only, lists of 3 long each but not varying sizes)
    2. Tuples are lists of varying data type, call item by refering to var_name dot 0-index notation value

*/
