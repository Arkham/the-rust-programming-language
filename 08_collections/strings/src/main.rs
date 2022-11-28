fn main() {
    let data = "initial contents";
    println!("{}", data.to_string());

    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
