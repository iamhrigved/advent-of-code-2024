// I FOUND THE MOST ELEGANT SOLUTION !!!

use std::collections::HashMap;

fn main() {
    println!("DAY 11!!!");

    let input = include_str!("../../input/day11.txt");
    let (part1, part2) = answer(input);

    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

fn answer(input: &str) -> (u64, u64) {
    let nums = input
        .split(" ")
        .map(|num| num.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    (count_rocks(&nums, false), count_rocks(&nums, true))
}

fn count_rocks(nums: &Vec<u64>, part2: bool) -> u64 {
    let mut count = 0;
    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();

    fn solve(num: u64, blinks: u8, cache: &mut HashMap<(u64, u8), u64>, part2: bool) -> u64 {
        if part2 {
            if blinks == 75 {
                return 1;
            }
        } else if blinks == 25 {
            return 1;
        }

        if let Some(ans) = cache.get(&(num, blinks)) {
            return *ans;
        }

        let ret = if num == 0 {
            solve(1, blinks + 1, cache, part2)
        } else if (num.ilog10() + 1) % 2 == 0 {
            let (num1, num2) = split_in_two(num);
            solve(num1, blinks + 1, cache, part2) + solve(num2, blinks + 1, cache, part2)
        } else {
            solve(num * 2024, blinks + 1, cache, part2)
        };

        cache.insert((num, blinks), ret);
        ret
    }

    for num in nums {
        count += solve(*num, 0, &mut cache, part2);
    }

    count
}

fn split_in_two(num: u64) -> (u64, u64) {
    let num_of_digits = num.ilog10() + 1;
    let num2 = num % 10u64.pow(num_of_digits / 2);
    let num1 = (num - num2) / (10u64.pow(num_of_digits / 2));

    (num1, num2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let input = "125 17";

        assert_eq!(answer(input), (55312, 65601038650482));
    }
}
