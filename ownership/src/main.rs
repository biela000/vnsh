fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!"); trying to borrow a value that has already been moved

    let mut s = String::from("hello");
    s = String::from("ahoy"); // the hello value is invalidated here

    println!("{s}, world!");

    let str = String::from("test");
    takes_ownership(str); // The value here is moved

    // println!("{str}"); so we cannot use the variable anymore after the call
    //
    let x = 5;
    makes_copy(x);

    println!("{x}");

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
