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

fn get_common_item_in_group(group_items: &Vec<String>) -> char {

    let items_dicts: &mut Vec<HashMap<char, bool>>
        = &mut vec![HashMap::new(), HashMap::new(), HashMap::new()];

    for (elf_idx, elf_items) in group_items.into_iter().enumerate() {
        for item in elf_items.chars() {
            items_dicts[elf_idx].insert(item.to_owned(), true);
        }
    }

    let common_item_idx_at_first = group_items[0].chars().find(|c| {
        for dict in items_dicts.clone() {
            let is_in_dict = match dict.get(&c) {
                Some(true) => true,
                Some(_) => false,
                None => false,
            };

            if !is_in_dict { return false }
        }

        true
    });

    common_item_idx_at_first.unwrap()
}

fn main() {    

    let value = input::get_input().trim();
    let value_split = value.split("\n");
    let mut value_split_in_chunks: Vec<Vec<String>> = Vec::new();

    const GROUP_SIZE: usize = 3;
    let mut current_group: Vec<String> = Vec::new();
    for (idx, item) in value_split.enumerate() {
        current_group.push(item.trim().to_owned());
        if (idx + 1) % GROUP_SIZE == 0 {
            value_split_in_chunks.push(current_group.to_vec());
            current_group = Vec::new();
        }
    }
    
    let total_priority = value_split_in_chunks.into_iter().fold(0, | acc, group | {
        return acc + get_item_priority(get_common_item_in_group(&group));
    });
    
    println!("Result: {}", total_priority);
}

#[test]
fn test_get_common_item() {
    assert_eq!(
        get_common_item_in_group(
            &vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(), 
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(), 
                "PmmdzqPrVvPwwTWBwg".to_string(),
            ]
        ), 
        'r'
    );
    assert_eq!(
        get_common_item_in_group(
            &vec![
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(), 
                "ttgJtRGJQctTZtZT".to_string(), 
                "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
            ]
        ), 
        'Z'
    );
}

#[test]
fn test_get_item_priority() {
    assert_eq!(get_item_priority('a'), 1);
    assert_eq!(get_item_priority('z'), 26);
    assert_eq!(get_item_priority('A'), 27);
    assert_eq!(get_item_priority('Z'), 52);
}