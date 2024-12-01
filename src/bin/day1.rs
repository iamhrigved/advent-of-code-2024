fn main() {
    println!("DAY 1!!!");

    let input = include_str!("../../input/day1.txt"); // input file

    let (part1, part2) = answer(input);
    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

fn answer(input: &str) -> (u32, u32) {
    let numbers = input.split_whitespace();
    let mut numbers_left: Vec<u32> = Vec::new();
    let mut numbers_right: Vec<u32> = Vec::new();

    let mut answer: (u32, u32) = (0, 0);

    // convert both sides into two different vectors
    for (i, num_str) in numbers.enumerate() {
        if i % 2 == 0 {
            let num: u32 = num_str.parse().unwrap();
            numbers_left.push(num);
        } else {
            let num: u32 = num_str.parse().unwrap();
            numbers_right.push(num);
        }
    }

    // get answer to the second part
    // (calculated first, because the first part requires the elements of the vectors to be removed)
    for num in &numbers_left {
        answer.1 += *num * find_num(*num, &numbers_right);
    }

    // get answer to the first part
    for i in 0..numbers_left.len() {
        let index_min_left = find_min(&numbers_left);
        let index_min_right = find_min(&numbers_right);

        answer.0 += numbers_left
            .remove(index_min_left)
            .abs_diff(numbers_right.remove(index_min_right));
    }

    answer
}

// returns the index of the smallest number in the list
fn find_min(numbers: &[u32]) -> usize {
    let mut min = numbers[0];
    let mut index_min = 0;
    for (i, num) in numbers.iter().enumerate() {
        if *num < min {
            min = *num;
            index_min = i;
        }
    }
    index_min
}

// returns the number of times an element appears in the list
fn find_num(num: u32, numbers: &[u32]) -> u32 {
    let mut times: u32 = 0;

    for n in numbers {
        if *n == num {
            times += 1;
        }
    }

    times
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(answer(test_input), (11, 31));
    }
}
