use std::io;

fn main() {

    println!("Give me a number: ");

    let mut x = String::new();
    io::stdin().read_line(&mut x);
    let x: usize = x.trim().parse().expect("wtf");

    if x <= 5 {
        println!("It's too small");
    }else{
        println!("It's big enough");
    }

    let even = if x % 2 == 0 {true} else {false};

    if even {
        println!("It is even!");
    } else {
        println!("It is odd!");
    }


    let odd = x % 2 == 1;

    if odd {
        println!("It is odd!");
    } else {
        println!("It is even!");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("It's {result}, which is {counter} * 2");

    let mut i = 0;
    while i < 10 {
        let mut j = 0;
        while j < i {
            print!("*");
            j = j + 1;
        }
        println!("");
        i = i + 1;
    }

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];

    for month in months {
        println!("It's {month}!");
    }

    for number in (1..10).rev() {
        println!("t-minus {number} and counting");
    }


    println!("Blast off!");

    for number in 1..10 {
        println!("{number} since launch");
    }
}
