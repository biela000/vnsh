use std::io;

fn main() {
    let arr = [1, 2, 3, 4, 5];

    println!("Please enter a new array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Could not read line.");

    let index: usize = index.trim().parse().expect("Must be a valid number!");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}
