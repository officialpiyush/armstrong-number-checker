use std::convert::TryInto;
use std::io::{stdin, stdout, Write};

fn main() {
    let number = get_number_from_user();
    let split_numbers: Vec<_> = number
        .to_string()
        .chars()
        .map(|n| n.to_digit(10).unwrap())
        .collect();
    let power = split_numbers.len();
    let mut sum = 0;

    for num in &split_numbers {
        sum += num.pow(power.try_into().unwrap());
    }

    if sum == number.try_into().unwrap() {
        println!("The number is an armstrong number.");
    } else {
        println!("The given number is not an armstrong number.");
    }
}

fn get_number_from_user() -> i32 {
    let mut s = String::new();
    print!("Please enter some text: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    let num: i32 = s
        .trim()
        .parse()
        .expect("please give me correct string number!");

    return num;
}
