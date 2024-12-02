advent_of_code::solution!(2);
fn is_safe(report: &[i32]) -> bool {
    let mut safe = true;
    let mut is_increasing = None;

    for i in 0..report.len() - 1 {
        let diff = report[i] - report[i + 1];
        if is_increasing.is_none() {
            is_increasing = Some(diff.is_positive());
        } else if is_increasing.unwrap() != diff.is_positive() {
            safe = false;
            break;
        }

        if diff.abs() == 0 || diff.abs() > 3 {
            safe = false;
            break;
        }
    }
    safe
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if is_safe(&report) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if is_safe(&report) {
            count += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut new_report = report.to_vec();
            new_report.remove(i);

            if is_safe(&new_report) {
                count += 1;
                break;
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
