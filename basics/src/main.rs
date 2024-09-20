// struct rectangle {
//     width: i32,
//     height: i32
// }

// impl rectangle {
//     fn area(&self) -> i32{
//         self.width * self.height
//     }
// }

enum Direction {
    Left,
    Right,
    Up,
    Down
}

fn main() {
    // println!("Learning rust basics");
    // println!("{}", is_even(32));
    // println!("{}", find_nth_fibonaci(9));
    // println!("{}", String::from("Pavan"));
    // let mut s: String = String::from("Dhonia");
    // let mut str = "Pavan Kumar";
    // s = String::from("virat");
    // str = "birat";
    // s.push_str("Pk");
    // println!("{}",s.replace("a", "_"));
    // println!("{} {}",str,s);
    // str = "Ravan";
    // s = "Pavan";
    // let r1 = rectangle {
    //     width: 4,
    //     height: 9
    // };
    // println!("{}",r1.width + r1.height + 11);
    // println!("{}",r1.area());
    let d = Direction::Down;
    let mut moved = match d {
        Direction::Down => String::from("down"),
        Direction::Left => String::from("left"),
        Direction::Right => String::from("right"),
        Direction::Up => String::from("up")
    };
    moved.push_str(" side moved");
    println!("{}",moved);
}

// function for even number

// fn is_even(n: i32) -> bool {
//     if n % 2 == 0 {
//         return true;
//     }
//     return false;
// }

// function for fibonaci number

// fn find_nth_fibonaci(n: i32) -> i32 {
//     let mut a = 0;
//     let mut b = 1;
//     if n == 0 {
//         return a;
//     } else if n == 1 {
//         return b;
//     }
//     for _ in 1..n-2 {
//         let c = a + b;
//         a = b;
//         b = c;
//     }
//     return b;
// }


/*
Notes: 
    -> use let key word to declare variable
    -> &str is default if we don't assign any type for a string
    -> &str is immutable reference, it means we can change refernce but we can't modify the existing value
    -> String is completely mutable if we use mut keyword
    -> ("{}",variable) this is how we print variables in a string
    -> Structs are just like classes in OOPS
    -> we can assign variables in  struct directly, but not in case of methods
    -> For methods we need to implement the struct and in that we can define it
        example: struct rectangle {
                    width: i32,
                    height: i32
                };
                impl rectangle {
                    fn area(&self) -> i32 {
                        return self.width + self.height;
                    }
                }
    -> self is reference to the object
    -> return statement can be like {
        "return variable_name;" or just "variable_name"
    }
    -> enums in rust are used when we need something like constants
    -> we can use pattern matching in enums
*/