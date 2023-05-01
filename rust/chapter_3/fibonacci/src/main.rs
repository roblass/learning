fn main() {

    let n = std::env::args().nth(1).expect("no pattern given");
    let n = n.parse::<i32>().unwrap();

    let mut last = 1;
    let mut second_last = 0;

    let mut i = 0;

    while i < n {

        let tmp = last;
        last = last + second_last;
        second_last = tmp;
        
        i = i + 1;
    }

    println!("The {n}th fibonacci is {last}");
}
