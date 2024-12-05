advent_of_code::solution!(5);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut page_ordering_rules = Vec::new();
    let mut page_numbers = Vec::new();
    let mut section1 = true;

    for line in input.lines() {
        if section1 {
            let mut parts = line.split("|");
            if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
                page_ordering_rules.push((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()));
            }

            if line.is_empty() {
                section1 = false;
            }
        } else {
            let parts: Vec<i32> = line
                .split(",")
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();

            let mut page_hash_map = HashMap::new();
            for (i, n) in parts.iter().enumerate() {
                page_hash_map.insert(*n, i);
            }

            page_numbers.push(page_hash_map);
        }
    }

    let mut sum: u32 = 0;
    for update in page_numbers {
        let mut correct_order = true;
        let middle = update.len() / 2;
        let mut middle_number = 0;

        for (from, to) in page_ordering_rules.iter() {
            if !update.contains_key(from) || !update.contains_key(to) {
                continue;
            }

            let from_index = update.get(from).unwrap();
            let to_index = update.get(to).unwrap();

            if *from_index == middle {
                middle_number = *from;
            }

            if from_index > to_index {
                correct_order = false;
                break;
            }
        }

        if correct_order {
            sum += middle_number as u32;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut page_ordering_rules = Vec::new();
    let mut page_numbers = Vec::new();
    let mut section1 = true;

    for line in input.lines() {
        if section1 {
            let mut parts = line.split("|");
            if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
                page_ordering_rules.push((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()));
            }

            if line.is_empty() {
                section1 = false;
            }
        } else {
            let parts: Vec<i32> = line
                .split(",")
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();

            let mut page_hash_map = HashMap::new();
            for (i, n) in parts.iter().enumerate() {
                page_hash_map.insert(*n, i);
            }

            page_numbers.push(page_hash_map);
        }
    }

    let mut pages: Vec<Vec<i32>> = Vec::new();
    for update in page_numbers {
        for (from, to) in page_ordering_rules.iter() {
            if !update.contains_key(from) || !update.contains_key(to) {
                continue;
            }

            let from_index = update.get(from).unwrap();
            let to_index = update.get(to).unwrap();

            if from_index > to_index {
                pages.push(update.keys().cloned().collect());
                break;
            }
        }
    }

    let mut sum: u32 = 0;
    for page in pages.iter_mut() {
        page.sort_by(|a, b| {
            for &(x, y) in page_ordering_rules.iter() {
                if x == *a && y == *b {
                    return std::cmp::Ordering::Less; // a vor b
                } else if x == *b && y == *a {
                    return std::cmp::Ordering::Greater; // b vor a
                }
            }
            std::cmp::Ordering::Equal
        });

        let middle = page.len() / 2;
        sum += page[middle] as u32;
    }

    // println!("{:?}", pages);

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
