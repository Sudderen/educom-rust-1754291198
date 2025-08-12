use core::num;
use std::{ops::Index, result};

#[derive(Debug)]
enum Language {
    English,
    Spanish, 
    Russian,
    Japanese,
}

enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}

enum Message {
    Quit, 
    Move {x: i32, y:i32},
    Write(String), 
    ChangeColor(Color),
}

struct Point{
    x: i32,
    y: i32,
}


fn main() {

    // let origin = Point {x:0, y:0};

    // match origin {
    //     Point {x, ..} => println!("x is {}", x),
    // }

    // let _x = 5;
    // let y = 10;

    // let s = Some(String::from("Hello!"));

    // if let Some(_) = s {
    //     println!("Found a string");
    // }

    // println!("{:?}", s);

    // let numbers = (2, 4, 8, 16, 32);

    // match numbers {
    //     (first, _, third, fifth) => {
    //         println!("Some numbers: {}, {}, {}", first, third, fifth)
    //     }
    // }

    // let mut setting_value = Some(5);
    // let new_setting_value = Some(10);

    // match (setting_value, new_setting_value) {
    //     (Some(_), Some(_)) => {
    //         println!("Can't overwrite an existing customized value");
    //     }
    //     _ => {
    //         setting_value = new_setting_value;
    //     }
    // }

    // println!("Setting is {:?}", setting_value);

    // let ((feet, inches), Point {x, y}) = ((3,10), Point {x: 3, y: -10});

    // let msg = Message::ChangeColor(Color::HSV(0, 160, 255));

    // match msg {
    //     // Message::Quit => {
    //     //     println!("Quit");
    //     // }
    //     // Message::Move { x, y } => {
    //     //     println!("Move to x: {} y: {}", x, y)
    //     // }
    //     // Message::Write(text) => {
    //     //     println!("Text message: {}", text)
    //     // },
    //     Message::ChangeColor(Color::RGB(r, g,b )) => {
    //         println!("Change color: red {}, green {}, and blue {}", r, g, b);
    //     }
    //     Message::ChangeColor(Color::HSV(h,s ,v )) => {
    //         println!(
    //             "Change color: hue {}, saturation {}, and value{}", h, s, v
    //         );
    //     }
    //     _ => (),
    // }

    // let p = Point {x: 0, y: 7};

    // match p {
    //     Point {x, y: 0} => {
    //         println!("On the x axis at {}", x)
    //     },
    //     Point {x: 0, y} => {
    //         println!("On the y axis at {}", y)
    //     },
    //     Point {x, y} => {
    //         println!("On neither axis: ({}, {})", x, y)
    //     }
    // }

    // let p = Point { x: 0, y: 7};

    // let Point { x, y} = p;
    // assert_eq!(0,x);
    // assert_eq!(7,y);

    // let x = 5;

    // match x {
    //     1..=5 => println!("One through Five"),
    //     _ => println!("Something else"),
    // }

    // let x = "c";

    // match x {
    //     "a"..="j" => println!("Early ASCII letter"),
    //     "k"..="z" => println!("Late ASCII letter"),
    //     _ => println!("Something else"),
    // }

    // let x = 1;

    // match x {
    //     1 | 2 => println!("One or Two"),
    //     3 => println!("Three"),
    //     _ => println!("Anything"),
    // }

    // let x = Some(5);
    // let y = 10;

    // match x {
    //     Some(50) => println!("Got 50"),
    //     Some(y) => println!("Matched, y = {:?}", y),
    //     _ => println!("Default case, x= {:?}", x),
    // }

    // let x = 1;

    // match x {
    //     1 => println!("One"),
    //     2 => println!("Two"),
    //     3 => println!("Three"),
    //     _ => println!("Anything"),
    // }

    // let x = 5;

    // let x: Option<&str> = None;
    // if let Some(x) = x {
    //     println!("{}", x);
    // }

    // let point = (3, 5);
    // print_coordinates|(&point);

    // fn print_coordinates(&(x, y): &(i32, i32)) {
    //     println!("Current location: ({}, {})", x, y);
    // }

    // let x = 5;

    // let (x, y, z) = (1, 2, 3); 

    // let v = vec!["a", "b", "c"];

    // for (index, value) in v.iter().enumerate() {
    //     println!("{} is at index {}", value, index);
    // }

    // let mut stack = Vec::new();

    // stack.push(1);
    // stack.push(2);
    // stack.push(3);

    // while let Some(top) = stack.pop() {
    //     println!("{}", top)
    // }

    // let authorization_status: Option<&str> = None;
    // let is_admin = false;
    // let group_id: Result<u8, _> = "34".parse();

    // if let Some(status) = authorization_status {
    //     println!("Autorization status: {}", status);
    // } else if is_admin {
    //     println!("Authorization status: admin");
    // } else if let Ok(group_id) = group_id {
    //     if group_id > 30 {
    //         println!("Authorization status: privileged")
    //     } else {
    //         println!("Authorization status: basic")
    //     }
    // } else {
    //     println!("Authorization status: guest");
    // }

    // let language = Language::English;

    // match language {
    //     Language::English => println!("Hellow World!"),
    //     Language::Spanish => println!("Hola Mundo!"),
    //     Language::Russian => println!("Привет, мир"),
    //     lang => println!("Unsupported language! {:?}", lang)
    // }
    
}
