#![allow(clippy::collapsible_if)]

fn main() {
    println!("DAY 4!!!");
    let input = include_str!("../../input/day4.txt");

    let (part1, part2) = answer(input);

    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

fn answer(input: &str) -> (u32, u32) {
    let mut data2d: Vec<Vec<char>> = Vec::new();
    input
        .lines()
        .for_each(|line| data2d.push(line.chars().collect()));

    let mut count = (0, 0);
    let len = data2d[0].len(); // given that the puzzle is going to be a square

    for i in 0..len {
        for j in 0..len {
            /*** PART 1 ***/

            // Horizontal
            if j <= len - 4 {
                // Left to Right
                if data2d[i][j] == 'X'
                    && data2d[i][j + 1] == 'M'
                    && data2d[i][j + 2] == 'A'
                    && data2d[i][j + 3] == 'S'
                {
                    count.0 += 1;
                }
            }
            if j >= 3 {
                // Right to Lift
                if data2d[i][j] == 'X'
                    && data2d[i][j - 1] == 'M'
                    && data2d[i][j - 2] == 'A'
                    && data2d[i][j - 3] == 'S'
                {
                    count.0 += 1;
                }
            }
            // Vertical
            if i <= len - 4 {
                // Top to Bottom
                if data2d[i][j] == 'X'
                    && data2d[i + 1][j] == 'M'
                    && data2d[i + 2][j] == 'A'
                    && data2d[i + 3][j] == 'S'
                {
                    count.0 += 1;
                }
            }
            if i >= 3 {
                // Bottom to Top
                if data2d[i][j] == 'X'
                    && data2d[i - 1][j] == 'M'
                    && data2d[i - 2][j] == 'A'
                    && data2d[i - 3][j] == 'S'
                {
                    count.0 += 1;
                }
            }
            // Diagonal
            if j <= len - 4 && i <= len - 4 {
                // Top-Left to Bottom-Right
                if data2d[i][j] == 'X'
                    && data2d[i + 1][j + 1] == 'M'
                    && data2d[i + 2][j + 2] == 'A'
                    && data2d[i + 3][j + 3] == 'S'
                {
                    count.0 += 1;
                }
            }
            if j >= 3 && i >= 3 {
                // Bottom-Right to Top-Left
                if data2d[i][j] == 'X'
                    && data2d[i - 1][j - 1] == 'M'
                    && data2d[i - 2][j - 2] == 'A'
                    && data2d[i - 3][j - 3] == 'S'
                {
                    count.0 += 1;
                }
            }
            if j <= len - 4 && i >= 3 {
                // Bottom-Left to Top-Right
                if data2d[i][j] == 'X'
                    && data2d[i - 1][j + 1] == 'M'
                    && data2d[i - 2][j + 2] == 'A'
                    && data2d[i - 3][j + 3] == 'S'
                {
                    count.0 += 1;
                }
            }
            if j >= 3 && i <= len - 4 {
                // Top-Right to Bottom-Left
                if data2d[i][j] == 'X'
                    && data2d[i + 1][j - 1] == 'M'
                    && data2d[i + 2][j - 2] == 'A'
                    && data2d[i + 3][j - 3] == 'S'
                {
                    count.0 += 1;
                }
            }

            /*** PART 2 ***/

            // if 'A' is inside an inner square of side len - 2
            if (1..len - 1).contains(&j) && (1..len - 1).contains(&i) {
                if data2d[i][j] == 'A' {
                    /*
                    S   .       M   .
                      A    or,    A
                    .   M        .  S
                    */
                    if (data2d[i - 1][j - 1] == 'M' && data2d[i + 1][j + 1] == 'S')
                        || (data2d[i - 1][j - 1] == 'S' && data2d[i + 1][j + 1] == 'M')
                    {
                        /*
                        .   M        .   S
                          A    or,     A
                        S   .        M   .
                        */
                        if (data2d[i - 1][j + 1] == 'M' && data2d[i + 1][j - 1] == 'S')
                            || (data2d[i - 1][j + 1] == 'S' && data2d[i + 1][j - 1] == 'M')
                        {
                            count.1 += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        assert_eq!(answer(input), (18, 9));
    }
}
