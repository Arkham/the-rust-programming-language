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

fn _largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
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

impl Point<f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn generic_structs() {
    // let wont_work = Point { x: 5, y: 8.0 };
    let _a = Point { x: 10, y: 18 };
    let b = Point { x: 'a', y: 'z' };
    println!("b: x = {}, y = {}", b.x(), b.y);
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

// the good news is that using generic types won't make your
// program run any slower than it would with concrete types.
// MONOMORPHIZATION!

// A trait defines functionality a particular type has an can
// share with other types. We can use trait bounds to specify
// that a generic type can be any type that has certain behaviour.

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        self.author.to_string()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// pub fn notify(item: &impl Summary) {
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

fn summarize_traits() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("foobar"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

// we can't implement external traits on external types.
// we can't implement Display on Vec<T>, because they're
// both defined in the stamdard library and aren't local
// to our crate. This is called the orphan rule, and it
// makes sure that two modules cannot implement the same
// trait for the same type.
