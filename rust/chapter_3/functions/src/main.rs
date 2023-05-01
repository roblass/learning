use std::cmp::Ordering;

fn main() {
    print_func(num_times());
}

//probably terrible way to write a function, just using the constructs i know so far
fn print_func(x: i32) {

    let mut i = 0;

    loop {
        match i.cmp(&x) {
            Ordering::Less => println!("Hello World!"),
            Ordering::Equal => {println!("Goodbye"); break;},
            Ordering::Greater => {println!("I am impossible"); break;},
        }
        i = i + 1;
    }
}

fn num_times() -> i32 {
    plus_one(4)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
