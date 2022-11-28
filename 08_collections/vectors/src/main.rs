fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    mutable_and_immutable_borrow();

    for i in &v {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 46];
    for i in &mut v2 {
        *i += 50;
    }
    for i in &v2 {
        println!("{}", i);
    }
}

fn mutable_and_immutable_borrow() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6);

    println!("The first element is: {}", first);
}
