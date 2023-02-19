fn main() {
    find_largest();
    generic_structs();
    mixing_it_up();
    summarize_traits();
}

fn find_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec!['a', 'c', 'r', 's'];

    let result = largest(&number_list);
    println!("The largest char is {}", result);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn generic_structs() {
    let _a = Point { x: 10, y: 18 };
    let b = Point { x: 'a', y: 'z' };
    println!("{} {}", b.x(), b.y);
}

// mixing generics defined in impl and methods

#[derive(Debug)]
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn mixing_it_up() {
    let a = Point2 { x: 1, y: 2.0 };
    let b = Point2 { x: "Hello", y: 'c' };
    println!("{:?}", a.mixup(b));
}

fn summarize_traits() {}
