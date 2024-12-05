// THIS PROBLEM WAS REALLY HARD FOR ME!!!
//
// BUT I FINALLY GOT IT!!!

fn main() {
    println!("DAY 5!!!");

    let input = include_str!("../../input/day5.txt");

    let (part1, part2) = answer(input);
    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

fn answer(input: &str) -> (usize, usize) {
    let mut count = (0, 0);

    let mut parts = input.split("\n\n"); // split both parts

    let (rules_raw, updates_raw) = (parts.next().unwrap(), parts.next().unwrap()); // get rules and
                                                                                   // updates separately

    let rules: Vec<(usize, usize)> = rules_raw // convert "X|Y" to vec![(X,Y)]
        .lines()
        .map(|rule| {
            let temp: Vec<usize> = rule
                .split("|")
                .map(|rule_str| rule_str.parse().unwrap())
                .collect();
            (temp[0], temp[1])
        })
        .collect();

    let mut updates = updates_raw // convert "A,B,C\nX,Y,Z" to vec![vec![A, B, C], vec![X, Y, Z]]
        .lines()
        .map(|num_str| {
            num_str
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    updates // for part 1
        .iter()
        .filter(|list| is_ordered(&rules, list)) // get list which are already ordered
        .for_each(|list| count.0 += list[list.len() / 2]); // sum their mid elements

    updates // for part 2
        .iter_mut()
        .filter(|list| !is_ordered(&rules, list)) // get lists which are not ordered
        .for_each(|list| {
            fix_order(&rules, list); // order them
            count.1 += list[list.len() / 2]; // sum their mid elements
        });

    count
}

fn is_ordered(rules: &[(usize, usize)], list: &[usize]) -> bool {
    list.is_sorted_by(|&l, &r| rules.contains(&(l, r)))
}

fn fix_order(rules: &[(usize, usize)], list: &mut [usize]) {
    while !is_ordered(rules, list) {
        // repeat if not ordered after one loop

        for values in rules {
            if !list.contains(&values.0) || !list.contains(&values.1) {
                // if any of the rules'
                // values not in the list
                continue;
            }

            // since we already checked the presence of both values, unwrap() will always work
            let id0 = find(list, values.0).unwrap();
            let id1 = find(list, values.1).unwrap();

            // if in "X|Y", Y occurs before X, then swap them
            if id0 > id1 {
                list.swap(id0, id1);
            }
        }
    }
}

fn find(list: &[usize], item: usize) -> Option<usize> {
    (0..list.len()).find(|&i| list[i] == item)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(answer(input), (143, 123));
    }
}
