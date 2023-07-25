// https://youtu.be/lzKeecy4OmQ?list=PLOISWgsXLgA60iCuk3UwPVmrbW6MHWN0f&t=4677

fn first_name(name: &str) {
    println!("Hey! {}", name);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn what_about(number: i32) {
    let n = 5;
    if number > n {
        println!("{} is greater than {}", number, n);
    }
    else if number < n {
        println!("{} is less than {}", number, n);
    }
    else {
        println!("{} is equal to {}", number, n);
    }
}

fn main() {
    let mut sum_number = 0;
    for n in 5..=10 {
        if n %2 == 0 {
            println!("{} is even", n);
        }
        else {
            println!("{} is odd", n);
        }
        sum_number += n;
    }
    println!("{}", sum_number);
    first_name("Jayson");
    println!("{}", add(10, 5));
    what_about(4);
}
