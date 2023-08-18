use std::env;
use std::collections::VecDeque;

fn main() {

    let args: Vec<String> = env::args().collect();

    let numbers: Result<Vec<i32>, _> = args.iter().skip(1)
        .map(|arg| arg.parse::<i32>())
        .collect();

    match numbers {
        Ok(nums) => {
            let result = max(&nums);
            match result {
                Some(biggest) => println!("The largest value in the array is {}", biggest),
                None => println!("The list is empty, so there is no max."),
            }

            let result = mean(&nums);

            match result {
                Some(avg) => println!("The mean of the values in the array is {}", avg),
                None => println!("The list is empty, so it has no mean."),
            }

            println!("A copy of the array is {:?}", copy(&nums));
            println!("The reversed array is {:?}", reverse(&nums));
        },
        Err(_) => {
            eprintln!("One of your parameters is not a number!");
        }
    }

}

//max
fn max(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }

    let mut max_val = nums[0];

    for &num in nums {
        if num > max_val {
            max_val = num;
        }
    }

    Some(max_val)
}

//average (mean)
fn mean(nums: &[i32]) -> Option<f64> {

    if nums.is_empty() {
        return None;
    }

    let sum: i32 = nums.iter().sum();

    Some(sum as f64 / nums.len() as f64)
}

//copy
fn copy(nums: &[i32]) -> Vec<i32> {
    let mut queue: Vec<i32> = Vec::new();

    for &num in nums {
        queue.push(num);
    }

    queue
}

//reverse
fn reverse(nums: &[i32]) -> Vec<i32> {
    
    let mut dequeue: VecDeque<i32> = VecDeque::new();

    for &num in nums {
        dequeue.push_front(num);
    }

    dequeue.into()
}
