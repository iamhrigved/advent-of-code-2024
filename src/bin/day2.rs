fn main() {
    println!("DAY 2!!!");

    let input = include_str!("../../input/day2.txt");

    let (part1, part2) = answer(input);
    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

fn answer(input: &str) -> (u32, u32) {
    let mut count: (u32, u32) = (0, 0);

    'outer: for line in input.lines() {
        let levels: Vec<&str> = line.split(" ").collect();

        if check_safe(&levels) {
            count.0 += 1; // safe counts without removing elements
        } else {
            let mut levels_temp = levels.clone();

            // remove a single element one by one and check if safe or not
            for i in 0..levels.len() {
                levels_temp.remove(i);

                if check_safe(&levels_temp) {
                    count.1 += 1; // safe counts after removing elements
                    continue 'outer;
                }
                levels_temp = levels.clone();
            }
        }
    }

    (count.0, count.0 + count.1)
}

// checks if a list of levels is safe (see the problem instructions)
fn check_safe(list: &Vec<&str>) -> bool {
    if !is_sorted(list) {
        return false;
    }
    for i in 1..list.len() {
        let diff = diff_str(list[i - 1], list[i]);
        if diff > 3 || diff < -3 {
            // absolute difference > 3
            return false;
        }
    }

    true
}

// checks if the list is sorted in either ascending or descending order (repetition not allowed)
fn is_sorted(list: &Vec<&str>) -> bool {
    if list.len() <= 1 {
        return true;
    }

    // declare ascending or descending
    let mut asc = true;
    if diff_str(list[0], list[1]) > 0 {
        asc = false;
    }

    for i in 1..list.len() {
        if asc {
            if diff_str(list[i - 1], list[i]) >= 0 {
                return false;
            }
        } else if diff_str(list[i - 1], list[i]) <= 0 {
            return false;
        }
    }

    true
}

fn diff_str(x_str: &str, y_str: &str) -> i32 {
    let x: i32 = x_str.parse().unwrap();
    let y: i32 = y_str.parse().unwrap();

    x - y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_str() {
        let x_str = "189";
        let y_str = "122";
        assert_eq!(diff_str(x_str, y_str), 67);
    }

    #[test]
    fn test_day2() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        assert_eq!(answer(input), (2, 4));
    }
}
