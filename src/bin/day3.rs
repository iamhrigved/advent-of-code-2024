fn main() {
    println!("DAY 3!!!");

    let input = include_str!("../../input/day3.txt");

    let (part1, part2) = answer(input);

    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

fn answer(input: &str) -> (u32, u32) {
    let input_vec: Vec<char> = input.chars().collect(); // convert &str to Vec<char>
    let mut num1: u32 = 0;
    let mut num2: u32 = 0;
    let mut answer: (u32, u32) = (0, 0);
    let mut enabled = true;

    // loop for part 1
    for mut i in 0..input_vec.len() {
        // check "mul("
        if input_vec[i] == 'm'
            && input_vec[i + 1] == 'u'
            && input_vec[i + 2] == 'l'
            && input_vec[i + 3] == '('
        {
            i += 4;
            // collect digits
            while input_vec[i].is_ascii_digit() {
                num1 = num1 * 10 + input_vec[i].to_digit(10).unwrap();
                i += 1;
            }

            // check ","
            if input_vec[i] == ',' {
                i += 1;
                // collect digits
                while input_vec[i].is_ascii_digit() {
                    num2 = num2 * 10 + input_vec[i].to_digit(10).unwrap();
                    i += 1;
                }
                // check ")"
                if input_vec[i] == ')' {
                    answer.0 += num1 * num2;
                }
            }
            // reset all numbers
            num1 = 0;
            num2 = 0;
        }
    }

    // loop for part 2
    for mut i in 0..input_vec.len() {
        // check "do()"
        if input_vec[i] == 'd'
            && input_vec[i + 1] == 'o'
            && input_vec[i + 2] == '('
            && input_vec[i + 3] == ')'
        {
            enabled = true;
        }

        // check "don't()"
        if input_vec[i] == 'd'
            && input_vec[i + 1] == 'o'
            && input_vec[i + 2] == 'n'
            && input_vec[i + 3] == '\''
            && input_vec[i + 4] == 't'
            && input_vec[i + 5] == '('
            && input_vec[i + 6] == ')'
        {
            enabled = false;
        }

        if input_vec[i] == 'm'
            && input_vec[i + 1] == 'u'
            && input_vec[i + 2] == 'l'
            && input_vec[i + 3] == '('
            && enabled
        {
            i += 4;
            // collect digits
            while input_vec[i].is_ascii_digit() {
                num1 = num1 * 10 + input_vec[i].to_digit(10).unwrap();
                i += 1;
            }

            if input_vec[i] == ',' {
                i += 1;
                // collect digits
                while input_vec[i].is_ascii_digit() {
                    num2 = num2 * 10 + input_vec[i].to_digit(10).unwrap();
                    i += 1;
                }
                if input_vec[i] == ')' {
                    answer.1 += num1 * num2;
                }
            }
            // reset all numbers
            num1 = 0;
            num2 = 0;
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_answer() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(answer(input), (161, 48));
    }
}
