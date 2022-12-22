use std::{str::FromStr};
mod input;

fn main() {
    let value = input::get_input();
    let value_split = value.split("\n");
    
    let mut elf_sum = 0;
    let mut all_sums = Vec::<i32>::new();
    for item in value_split.into_iter() {
        if item.trim().is_empty() {
            all_sums.push(elf_sum);
            elf_sum = 0;
        } else {
            elf_sum += i32::from_str(item.trim()).expect("Error parsing number");
        }
    }

    let max_sum = all_sums.into_iter().max();
    match max_sum {
        Some(value) => println!("Result is: {}", value),
        None => println!("Error getting max value"),
    }
    
}
