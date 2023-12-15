use std::fmt::Debug;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// The notify function takes in an argument/paramenter named item, it accepts anything that implements the Summary trait.
// It is actually a syntact suger for trait bound syntax
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize())
// }

// Trait bound syntax
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize())
// }

// More complicated syntax where multiple trait bounds are required
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    for item in vec![item1, item2] {
        println!("Breaking news! {}", item.summarize())
    }
}

// fn some_function<T: Summary + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Clone,
    U: Clone + Debug,
{
    0
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("This is a headline"),
        content: String::from("This is the content"),
    };

    println!("1 new tweet from {}", tweet.summarize());
    println!("New article available! {}", article.summarize());

    // notify(&tweet);
}
