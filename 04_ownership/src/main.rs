fn main() {
    // - each value in Rust has an owner
    // - there can only be one owner at a time
    // - when the owner goes out of scope, the value will be dropped

    // we can mutate strings on the heap
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{}", s);

    // copying only reference
    let s1 = String::from("hello");
    // this lines moves s1 to s2
    let s2 = s1;
    // now s1 is out of scope
    println!("{}", s1);

    // // cloning is okay
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    // this is fine because integers implement the Copy trait
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // // passing a variable to a function will move or copy, just
    // // as we have seen with assignments
    // let s = String::from("hello");
    // takes_ownership(s);
    // // println!("after_move: {}", s);

    // let x = 5;
    // makes_copy(x);
    // println!("after copy: {}", x);

    // let _s1 = gives_ownership();
    // let _s2 = String::from("hello");
    // let _s3 = takes_and_gives_back(s2);

    // what if we want a function use a value but not take ownership?
    let t1 = String::from("hello");
    let (t2, len) = calculate_length(t1);
    println!("The length of '{}' is {}.", t2, len);

    // this works, but it's a bit annoying.. let's use REFERENCES
    let u1 = String::from("hello");
    let len = calculate_length_ref(&u1);
    println!("The length of '{}' is {}.", u1, len);

    // // by default we can't mutate borrowed values, but...
    // let mut v1 = String::from("hello");
    // change(&mut v1);
    // println!("The result is '{}'.", v1);

    // // The Rules of References:
    // // - At any given time, you can have either one mutable reference or any number of immutable references.
    // // - References must always be valid.

    // // SLICES
    // let w = String::from("hello world");
    // let word = first_word(&w);
    // println!("The first word is '{}'.", word);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
