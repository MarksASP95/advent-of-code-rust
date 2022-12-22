use std::{str::FromStr};
mod input;

fn main() {
    let value = input::get_input();
    let value_split = value.split("\n");
    
    let mut elf_sum = 0;
    let mut top_three: Vec<i32> = vec![0, 0, 0];
    for item in value_split.into_iter() {
        if item.trim().is_empty() {
            for (idx, top_three_item) in top_three.to_owned().into_iter().enumerate() {
                if &elf_sum > &top_three_item {
                    if idx < (top_three.len() - 1) {
                        top_three[idx + 1] = top_three[idx];
                    }
                    top_three[idx] = elf_sum;
                    break;
                }
            }
            elf_sum = 0;
        } else {
            elf_sum += i32::from_str(item.trim()).expect("Error parsing number");
        }
    }

    let top_three_sum: i32 = top_three.iter().sum();
    println!("Top three is: {:?}", top_three);
    println!("Result is: {}", top_three_sum);
    
}
