use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage {} p q, where p and q are the numbers of which to find the GCD", args[0]);
        return;
    }

    let p = match args[1].parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            println!("Error, {} is not a valid integer.", args[1]);
            return;
        }
    };

    let q = match args[2].parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            println!("Error, {} is not a valid integer.", args[2]);
            return;
        }
    };

    println!("The GCD of {} and {} is {}", p, q, euclid(p, q));
}

fn euclid(p: i32, q: i32) -> i32 {
    if q == 0 {
        return p;
    }

    let remainder = p % q;

    return euclid(q, remainder);
}
