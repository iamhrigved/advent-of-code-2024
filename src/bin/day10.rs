// I SOLVED PART 2 BEFORE PART 1 XD

fn main() {
    println!("DAY 10!!!");

    let input = include_str!("../../input/day10.txt");
    let (part1, part2) = answer(input);

    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

fn answer(input: &str) -> (u32, u32) {
    let topo_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<_>>>();
    let len = topo_map.len();

    let mut count = (0, 0);

    for i in 0..len {
        for j in 0..len {
            if topo_map[i][j] != '0' {
                continue;
            }
            count.0 += find_path(&topo_map, (i, j), false);
            count.1 += find_path(&topo_map, (i, j), true);
        }
    }

    count
}

fn find_path(map: &[Vec<char>], pos: (usize, usize), part2: bool) -> u32 {
    if map[pos.0][pos.1] != '0' {
        return 0;
    }
    fn search(
        map: &[Vec<char>],
        pos: (i64, i64),
        search_ch: char,
        count: &mut u32,
        visited: &mut Vec<(i64, i64)>,
        part2: bool,
    ) -> bool {
        if !(0..map.len()).contains(&(pos.0 as usize))
            || !(0..map.len()).contains(&(pos.1 as usize))
            || map[pos.0 as usize][pos.1 as usize] != search_ch
        {
            return false;
        }
        if search_ch == '9' {
            if !visited.contains(&pos) || part2 {
                visited.push(pos);
                *count += 1;
            }

            return true;
        };
        let res1 = search(
            map,
            (pos.0 + 1, pos.1),
            (search_ch as u8 + 1) as char,
            count,
            visited,
            part2,
        );
        let res2 = search(
            map,
            (pos.0 - 1, pos.1),
            (search_ch as u8 + 1) as char,
            count,
            visited,
            part2,
        );
        let res3 = search(
            map,
            (pos.0, pos.1 + 1),
            (search_ch as u8 + 1) as char,
            count,
            visited,
            part2,
        );
        let res4 = search(
            map,
            (pos.0, pos.1 - 1),
            (search_ch as u8 + 1) as char,
            count,
            visited,
            part2,
        );
        res1 || res2 || res3 || res4
    }

    let mut count = 0;

    search(
        map,
        (pos.0 as i64, pos.1 as i64),
        map[pos.0][pos.1],
        &mut count,
        &mut vec![],
        part2,
    );

    count
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(answer(input), (36, 81));
    }
}
