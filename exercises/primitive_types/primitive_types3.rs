// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.


fn main() {
    //let a: Vec<i32> = [1, 2].to_vec();
    //let a = vec![42.0, 21.37];
    let a = ["Are we there yet?"; 404];



    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
