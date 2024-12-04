advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec_input: Vec<Vec<char>> = Vec::new();
    let word = "XMAS";

    for line in input.lines() {
        vec_input.push(line.chars().collect());
    }

    let directions = [
        (0, 1),   // Horizontal rechts
        (0, -1),  // Horizontal links
        (1, 0),   // Vertikal nach unten
        (-1, 0),  // Vertikal nach oben
        (1, 1),   // Diagonal nach unten rechts
        (1, -1),  // Diagonal nach unten links
        (-1, 1),  // Diagonal nach oben rechts
        (-1, -1), // Diagonal nach oben links
    ];

    let rows = vec_input.len();
    let cols = vec_input[0].len();
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word.len();

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                let mut match_found = true;
                for i in 0..word_len {
                    let new_row = row as isize + i as isize * dx;
                    let new_col = col as isize + i as isize * dy;

                    if new_row < 0
                        || new_row >= rows as isize
                        || new_col < 0
                        || new_col >= cols as isize
                    {
                        match_found = false;
                        break;
                    }

                    if vec_input[new_row as usize][new_col as usize] != word_chars[i] {
                        match_found = false;
                        break;
                    }
                }

                if match_found {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec_input: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        vec_input.push(line.chars().collect());
    }

    let rows = vec_input.len();
    let cols = vec_input[0].len();
    let mut count = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if vec_input[row][col] == 'A' {
                // Check all valid X-MAS patterns
                let top_left = vec_input[row - 1][col - 1];
                let top_right = vec_input[row - 1][col + 1];
                let bottom_left = vec_input[row + 1][col - 1];
                let bottom_right = vec_input[row + 1][col + 1];

                if (top_left == 'M' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'S') || // Pattern 1: M.M
                   (top_left == 'M' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'S') || // Pattern 2: M.S
                   (top_left == 'S' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'M') || // Pattern 3: S.S
                   (top_left == 'S' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'M')
                // Pattern 4: S.M
                {
                    count += 1;
                }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
