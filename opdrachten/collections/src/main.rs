use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
    //vectors

    // let a = [1, 2, 3];
    // let mut v:Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    // let v = vec![1, 2, 3, 4, 5];

    // let third = &v[2];
    // v.push(6);
    // println!("The third element is {third}");

    // match v.get(2){
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }

    // let mut v = vec![1, 2, 3, 4, 5];

    // for i in &mut v {
    //     *i += 50;
    // }

    // for i in &v {
    //     println!("{i}")
    // }


    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec! [
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // match &row[1] {
    //     SpreadsheetCell::Int(i) => println!("{i}"),
    //     _ => print!("Not a integer!")
    // };

    
    //strings

    // let s1 = String::new();
    // let s2 = "initial contents";
    // let s3 = s2.to_string();
    // let s4 = String::from("initial contents");

    // let mut s = String::from("foo");
    // s.push_str("bar");
    // s.push('!');

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("World!");
    // let s3: String = s1 + &s2;

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = format!("{}{}", s1, s2);

    // let hello: String = String::from("test");

    // for b in "test".bytes() {
    //     println!("{b}");
    // }

    // for c in "test".chars() {
    //     println!("{c}");
    // }

    // for g in "test".graphemes(true){
    //     println!("{g}");
    // }


    //hashmaps


    // let blue = String::from("Blue");
    // let yellow = String::from("Yellow");

    // let mut scores = HashMap::new();

    // scores.insert(blue, 10);
    // scores.insert(yellow, 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    // for(key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 20);

    // scores.entry(String::from("Yellow")).or_insert(30);
    // scores.entry(String::from("Yellow")).or_insert(40);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }

    println!("{:?}", map);

}
