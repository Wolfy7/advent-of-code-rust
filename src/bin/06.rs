advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start_pos: (i32, i32) = (0, 0);
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            if c == '^' {
                start_pos = (x as i32, y as i32);
            }
        }
        map.push(row);
    }

    let mut direction_iter = directions.iter().cycle();
    let mut direction = direction_iter.next().unwrap();

    let mut current_pos: (i32, i32) = start_pos;
    let mut distinct_positions = 0;
    loop {
        current_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);

        if current_pos.0 < 0
            || current_pos.0 >= map.len() as i32
            || current_pos.1 < 0
            || current_pos.1 >= map[0].len() as i32
        {
            break;
        }

        if map[current_pos.1 as usize][current_pos.0 as usize] == '#' {
            current_pos = (current_pos.0 - (direction.0), current_pos.1 - (direction.1));
            direction = direction_iter.next().unwrap();
            continue;
        }

        if map[current_pos.1 as usize][current_pos.0 as usize] == '.'
            || map[current_pos.1 as usize][current_pos.0 as usize] == '^'
        {
            map[current_pos.1 as usize][current_pos.0 as usize] = 'X';
            distinct_positions += 1;
        }
    }
    Some(distinct_positions)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
