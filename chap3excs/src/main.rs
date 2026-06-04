use std::io;

fn fahrenheit_to_celsius(fdegs: f32) -> f32 {
    (fdegs - 32.0) * (5.0 / 9.0)
}

fn exc1() {
    let fdegs: f32 = loop {
        let mut input = String::new();

        println!("Please input the value in Fahrenheit.");

        io::stdin()
            .read_line(&mut input)
            .expect("There was a problem reading the input.");

        let _: f32 = match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    let cdegs = fahrenheit_to_celsius(fdegs);

    println!("{fdegs} Fahrenheit is {cdegs} Celsius");
}

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 { 0 }
    else if n == 2 { 1 }
    else {
        fib(n - 1) + fib(n - 2)
    }
}

fn exc2() {
    let n: u32 = loop {
        let mut input = String::new();

        println!("What element of the fibonacci sequence do you want to see?");

        io::stdin()
            .read_line(&mut input)
            .expect("There was a problem reading the input.");

        let _: u32 = match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    let fib_n = fib(n);

    println!("F_{n}={fib_n}");
}

fn exc3() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eigth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for (i, day) in days.iter().enumerate() {
        println!("[Verse {}]", i + 1);
        println!("On the {day} day of Christmas, my true love sent to me");
        for j in (0..=i).rev() {
            println!("{}", gifts[j]);
        }
    }
}

fn main() {
    exc1();
    exc2();
    exc3();
}
