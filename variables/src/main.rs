fn _mut() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn _shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn _data_types() {
    let _int: u32 = 100;
    let _float: f64 = 100.45;
    let _bool: bool = false;
    let _char: char = 'Z';
    let _tuple: (i32, f64, u8) = (500, 6.4, 1);
    let _array: [i32; 5] = [1, 2, 3, 4, 5];
}

use std::io;

fn _safe_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

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
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn _control_structure() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = plus_one(5);

    println!("The value of x is: {}", x);

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {}", element);
    }

    for number in (1..10).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("80 F is {} C", fahrenheit2celsius(80.0));
    println!("The 13th Fibonacci number is {}", fib(13));
}

fn fahrenheit2celsius(n: f64) -> f64 {
    (n - 32.0) * 5.0 / 9.0
}

fn fib(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        n => fib(n - 2) + fib(n - 1),
    }
}
