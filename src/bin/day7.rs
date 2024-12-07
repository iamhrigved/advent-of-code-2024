// THIS ALSO TAKES A LOT OF TIME (T_T)

fn main() {
    println!("DAY 7!!!");

    let input = include_str!("../../input/day7.txt");

    let (part1, part2) = answer(input);

    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

fn answer(input: &str) -> (u64, u64) {
    let mut count = (0, 0);
    let (results, numbers): (Vec<u64>, Vec<Vec<u64>>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let res = parts.next().unwrap().parse::<u64>().unwrap();
            let nums = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            (res, nums)
        })
        .collect();

    for i in 0..results.len() {
        if check_res(results[i], &numbers[i], false) {
            // first check for part1
            count.0 += results[i];
        }
        if check_res(results[i], &numbers[i], true) {
            // now check for part2
            count.1 += results[i];
        }
    }
    count
}

fn check_res(target: u64, nums: &[u64], part2: bool) -> bool {
    let len = nums.len();

    if len == 1 && target == nums[0] {
        return true;
    }
    if len == 1 {
        return false;
    }

    let mut add = vec![nums[0] + nums[1]];
    let mut mul = vec![nums[0] * nums[1]];
    add.append(&mut nums[2..].to_owned());
    mul.append(&mut nums[2..].to_owned());

    if check_res(target, &add, part2) {
        // add the first two elements of the vector and pass in the rest
        return true;
    }
    if check_res(target, &mul, part2) {
        // multiply the first two elements of the vector and pass in the rest
        return true;
    }
    if part2 {
        let mut concat = vec![concat_nums(nums[0], nums[1])];
        concat.append(&mut nums[2..].to_owned());

        // concat the first two elements of the vector and pass in the rest
        if check_res(target, &concat, part2) {
            return true;
        }
    }

    false
}

fn concat_nums(num1: u64, num2: u64) -> u64 {
    (num1 as f64 * 10u64.pow(num2.ilog10() + 1) as f64 + num2 as f64) as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(answer(input), (3749, 11387));
        //assert_eq!(de_concat_nums(145, 5), 14);
    }
}
