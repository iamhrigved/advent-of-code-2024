fn main() {
    println!("DAY 2!!!");

    let input = include_str!("../../input/day2.txt");

    let (part1, part2) = answer(input);
    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

fn answer(input: &str) -> (u32, u32) {
    let mut count: (u32, u32) = (0, 0);

    for line in input.lines() {
        let mut levels: Vec<i32> = Vec::new();
        line.split(" ")
            .for_each(|lev| levels.push(lev.parse().unwrap()));
        // convert &str to list of numbers

        // if already safe
        if check_safe(&levels) {
            count.0 += 1;
            count.1 += 1;
            continue;
        }

        // if not safe already
        // check by removing elements one by one
        for i in 0..levels.len() {
            // if a safe is found
            if check_safe_remove(&levels, i) {
                count.1 += 1;
                break;
            }
        }
    }

    count
}

// checks if a list of levels is safe (see the problem instructions)
fn check_safe(list: &Vec<i32>) -> bool {
    let mut only_inc = true;
    let mut only_dec = true;
    let mut is_ok = true;

    for i in 0..list.len() - 1 {
        let diff = list[i] - list[i + 1];
        if diff >= 0 {
            only_inc = false
        }
        if diff <= 0 {
            only_dec = false
        }
        if diff.abs() > 3 {
            is_ok = false
        }
    }

    is_ok && (only_inc || only_dec)
}

// removes the element at index and rechecks if safe
fn check_safe_remove(list: &Vec<i32>, index: usize) -> bool {
    let mut list_copy = list.clone();

    list_copy.remove(index);

    check_safe(&list_copy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        assert_eq!(answer(input), (2, 4));
    }
}
