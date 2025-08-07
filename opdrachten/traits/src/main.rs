use std::{fmt::{format, Debug, Display}, iter::Sum};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
   fn summarize_author(&self) -> String {
       format!("{}", self.author)
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

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

pub fn notify(item: &impl Summary){
    println!("Breaking news! {}", item.summarize())
}

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }


// pub fn notify<T: Summary + Display>(item1: &T, item2: &T){}

// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where   T: Display + Clone,
//             U: Clone + Debug
// {}

fn main() {
    // let tweet = Tweet {
    //     username: String::from("@test"),
    //     content: String::from("Hello World"),
    //     reply: false,
    //     retweet: false,
    // };

    // let article = NewsArticle {
    //     author: String::from("John Doe"),
    //     headline: String::from("The Sky is Falling!"),
    //     content: String::from("The sky is not actually falling."),
    // };

    // println!("Tweet summary: {}", tweet.summarize());
    // println!("Article summary: {}", article.summarize());

    // notify(&article);
    println!("{}", returns_summarizable().summarize());
}
