mod summary;
use summary::Summary;
use std::fmt::Display;

pub fn notify(item: impl Summary) {
    item.summarize_author();
}

pub fn trait_bound<T: Summary + Display>(item: T) {
    item.summarize_author();
}

pub fn where_trait_bound<T, U>(_item: T, _next: U) -> String
    where T: Display + Summary,
        U: Display + Summary
{
    String::from("do somthing")
}

pub fn return_trait<T: Summary>(item: T) -> impl Summary {
    item
}

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    };

    largest
}

fn main() {
    let t = summary::Tweet {
        username: String::from("herin"),
        content: String::from("hello"),
        reply: false,
        retweet: false,
    };

    let str = t.summarize();

    let list = vec![1,2,3];

    let result = largest(&list);
    
    println!("{}", result);
}
