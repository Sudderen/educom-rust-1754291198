use std::result;
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str, 
    y: &'a str, 
    ann: T,
) -> &'a str
where 
    T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

fn main() {
    // let x = 5;

    // let r = &x;

    // println!("r: {}", r);

    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: & str) -> &'a str {
    x
}
