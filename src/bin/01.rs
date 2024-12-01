advent_of_code::solution!(1);

use std::collections::HashMap;
use std::iter::zip;

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
            list1.push(a.parse().unwrap());
            list2.push(b.parse().unwrap());
        }
    }

    list1.sort();
    list2.sort();

    let sum = zip(list1, list2).fold(0, |acc, (a, b)| acc + (a - b).abs());

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
            list1.push(a.parse().unwrap());
            list2.push(b.parse().unwrap());
        }
    }

    let mut map: HashMap<i32, i32> = HashMap::new();

    for item in list2 {
        *map.entry(item).or_insert(0) += 1;
    }

    let mut sum = 0;
    for item in list1 {
        if let Some(count) = map.get(&item) {
            sum += item * count
        }
    }

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
