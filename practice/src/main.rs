fn main() {
    println!("Hello, world!");
    // let arr = [1, 2, 94, 4, 5, 1, 81];
    // println!("Maximum element in array is: {}", findMaxElement(&arr));
}

fn findMaxElement(arr: &[i32]) -> i32 {
    let mut store = -1;
    for &i in arr.iter() {
        if store < i {
            store = i;
        }
    }
    return store;
}

/*
    Basic commands to create a project in rust:
    -> mkdir <your_project_name>
    -> cd <your_project_name>
    -> cargo init
    -> cargo build
    -> write code in main.rs file
    -> cargo run
*/
