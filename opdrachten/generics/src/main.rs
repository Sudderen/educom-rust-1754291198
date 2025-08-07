struct Point<T, U> {
    x: T,
    y: U,
}

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<U> Point<U> {
//     fn x(&self) -> &U {
//         &self.x
//     }
// }

// impl Point<f64> {
//     fn y(&self) -> f64 {
//         self.y
//     }
//  }

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { 
            x: self.x,
             y: other.y, 
        }
    }
}

fn main() {
    // let number_list = vec![1, 20, 6000, 0, 4];

    // let largest = get_largest(number_list);

    // println!("The largest is {largest}");

    // let char_list = vec!["i", "a", "k", "e"];

    // let largest = get_largest(char_list);

    // println!("The largest is {largest}");

    // let p1 = Point { x:5, y:10 };
    // let p2 = Point { x:5.0, y:10.0 };
    // let p3 = Point{ x:5, y:10.0 };

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // let p = Point { x: 5, y: 10 };
    // p.x();
    // let p1 = Point { x:5.0, y:1.0 };
    // p1.y();

    let p1 = Point {x: 5, y:10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x= {}, p3.y = {}", p3.x, p3.y);
}

// fn get_largest<T: PartialOrd + Copy>(list:Vec<T>) -> T {
//     let mut largest = list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
