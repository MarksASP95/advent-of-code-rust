use std::{collections::HashMap};
mod input;

fn get_item_priority(item: char) -> i32 {
    let ascii_code = item as u32;

    if item.is_ascii_uppercase() {
        (ascii_code as i32) - 38
    } else {
        (ascii_code as i32) - 96
    }
}

fn get_common_item(all_items: &str) -> char {
    let comp_length = all_items.len() / 2;
    let (first_comp, second_comp)  = all_items.split_at(comp_length);

    let mut item_dict: HashMap<char, bool> = HashMap::new();

    for item in first_comp.chars() {
        item_dict.insert(item.to_owned(), true);
    }

    let common_item_idx_at_second = second_comp.find(|c| {
        match item_dict.get(&c) {
            Some(true) => true,
            Some(_) => false,
            None => false,
        }
    }).expect("Error finding common item");

    second_comp.chars().nth(common_item_idx_at_second).unwrap()
}

fn main() {    

    let value = input::get_input().trim();
    let value_split = value.split("\n");
    
    let total_priority = value_split.into_iter().fold(0, | acc, b | {
        let priority = get_item_priority(get_common_item(b.trim()));
        println!("{}", priority);
        acc + priority
    });
    
    println!("Result: {}", total_priority);
}

#[test]
fn test_get_common_item() {
    assert_eq!(get_common_item("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
    assert_eq!(get_common_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
    assert_eq!(get_common_item("PmmdzqPrVvPwwTWBwg"), 'P');
    assert_eq!(get_common_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 'v');
    assert_eq!(get_common_item("ttgJtRGJQctTZtZT"), 't');
    assert_eq!(get_common_item("CrZsJsPPZsGzwwsLwLmpwMDw"), 's');
}

#[test]
fn test_get_item_priority() {
    assert_eq!(get_item_priority('a'), 1);
    assert_eq!(get_item_priority('z'), 26);
    assert_eq!(get_item_priority('A'), 27);
    assert_eq!(get_item_priority('Z'), 52);
}