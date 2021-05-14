// rust is a statically typed language, so the it must know the type of all
// variables when compiled
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter and array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    let element = a[index];

    println!(
        "The value of the element at index {} is: {}", index, element
    );

}