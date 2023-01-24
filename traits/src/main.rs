pub trait Summary {
    fn summarize(&self) -> String;
} 
pub trait DefaultImplementation {
    fn defaultsummarize(&self) -> String {
        String::from("(Read More...)")
    }
}
pub trait Summary1 {
    fn summarize_author(&self) -> String;

    fn summarize1(&self) -> String {
        format!("(Read more from {}...",self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})",self.headline, self.author, self.location)
    }
}
impl DefaultImplementation for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}",self.username, self.content)
    }
}
impl Summary1 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}",self.username)
    }
}

// //impl trait syntax
// pub fn notify(item: &impl Summary) {
//     println!("Breaking News! {}",item.summarize());
// }

//trait bound syntax
pub fn notify<T: Summay>(item: &T) {
    println!("\nBreaking News! {}",item.summarize());
}

// //two arguments that can have different types
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

// //two arguments that have to have the same types
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// //using the plus syntax to show, it must have two traits
// pub fn notify(item1: &(impl Summary + Display)) {}

// //Function without where clause
// fn some_function_without_where_clause<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// //Function with where clause
// fn some_function_with_where_clause<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {}

use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}",self.x);
        } else {
            println!("The largest member is y = {}",self.y);
        }
    }
}
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("chapter 10 of 10.3"),
        content: String::from(
            "traits - return type traits",
        ),
        reply: false,
        retweet: false,
    }
}
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}\n", tweet.summarize1());

    let news = NewsArticle {
        headline: String::from("Everywhere RUST"),
        location: String::from(
            "at Rapid Innovation",
        ),
        author: String::from("Kiran"),
        content: String::from("It is announced that everyone should learn rust."),
    };

    println!("1 news article: {}", news.summarize());
    println!("1 not define news article: {}\n", news.defaultsummarize());
    // println!("Hello, world!");

    //try using notify function 
    notify(&news);
    notify(&tweet);

}
