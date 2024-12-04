advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();

    let mut sum = 0;

    for captures in re.captures_iter(input) {
        let x = captures.get(1).map(|m| m.as_str()).unwrap();
        let y = captures.get(2).map(|m| m.as_str()).unwrap();
        sum += x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut find_dont = true;
    let mut i = 0;
    let mut sum = 0;

    while i <= input.len() {
        let offset;

        let found = if find_dont {
            offset = 7;
            input[i..].find("don't()")
        } else {
            offset = 4;
            input[i..].find("do()")
        };

        if find_dont {
            sum += part_one(&input[i..(found.unwrap_or(input.len() - i) + i)]).unwrap();
        }

        find_dont = !find_dont;
        i += found.unwrap_or(input.len()) + offset;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
