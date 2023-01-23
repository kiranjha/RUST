// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

struct Different_Points<T, U> {
    x: T,
    y: U,
}
impl<T,U> Different_Points<T, U> {
    fn mixup<V, W>(self, other: Different_Points<V, W>) -> Different_Points<T, W> {
        Different_Points {
            x: self.x,
            y: other.y,
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
impl Point<f32> {
    fn distance_free_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {result}");

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest number is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 24.0, y: 1.0 };

    println!("integer.x = {}", integer.x());
    println!("integer.y = {}", integer.y());

    println!("float.distance_free_origin = {}", float.distance_free_origin());

    let p1 = Different_Points{ x: 5, y: 10.4 };
    let p2 = Different_Points{ x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2); //p1 is self which T, U and p2 is Different_Point which is V, W,  now p3 contains x from p1 which is T and y from p2 which is W
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

