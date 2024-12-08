// TODO: MAKE IT BETTER

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("DAY 8!!!");

    let input = include_str!("../../input/day8.txt");
    let (part1, part2) = answer(input);

    println!("Answer to Part 1 it {}", part1);
    println!("Answer to Part 2 it {}", part2);
}

fn answer(input: &str) -> (u32, u32) {
    let mut count: (u32, u32) = (0, 0);

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let len = map.len();

    let mut antena_locs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..len {
        for j in 0..len {
            if map[i][j] == '.' || map[i][j] == '#' {
                continue;
            }
            if antena_locs.contains_key(&map[i][j]) {
                antena_locs
                    .entry(map[i][j])
                    .and_modify(|vec| vec.push((i, j)));
            } else {
                antena_locs.insert(map[i][j], vec![(i, j)]);
            }
        }
    }

    let mut unique_antinodes: HashSet<(usize, usize)> = HashSet::new(); // create a set to store
                                                                        // part1 antinodes
    for locs in antena_locs.values() {
        let antinodes = get_antinodes(locs, map.len(), false);
        for antinode in antinodes {
            unique_antinodes.insert(antinode);
        }
    }
    count.0 = unique_antinodes.len() as u32;

    let mut unique_antinodes: HashSet<(usize, usize)> = HashSet::new(); // create a set to store
                                                                        // part2 antinodes
    for locs in antena_locs.values() {
        let antinodes = get_antinodes(locs, map.len(), true);
        for antinode in antinodes {
            unique_antinodes.insert(antinode);
        }
    }
    count.1 = unique_antinodes.len() as u32;

    count
}

fn get_antinodes(
    locations: &[(usize, usize)],
    map_size: usize,
    part2: bool,
) -> Vec<(usize, usize)> {
    fn calc(
        antena_locs: &[(usize, usize)],
        map_size: usize,
        antinodes: &mut Vec<(usize, usize)>,
        part2: bool,
    ) {
        let first = antena_locs[0];
        let second = antena_locs[1];

        let diff_y: i32 = second.0 as i32 - first.0 as i32;
        let diff_x: i32 = second.1 as i32 - first.1 as i32;

        let mut i = 0;
        if !part2 {
            i = 1
        }
        loop {
            let antinode1 = ((first.0 as i32 - diff_y * i), (first.1 as i32 - diff_x * i));
            if in_bounds(antinode1, map_size) {
                antinodes.push((antinode1.0 as usize, antinode1.1 as usize));
            } else {
                break;
            }

            if !part2 {
                break;
            }
            i += 1;
        }
        i = 0;
        if !part2 {
            i = 1
        }
        loop {
            let antinode2 = (
                (second.0 as i32 + diff_y * i),
                (second.1 as i32 + diff_x * i),
            );
            if in_bounds(antinode2, map_size) {
                antinodes.push((antinode2.0 as usize, antinode2.1 as usize));
            } else {
                break;
            }

            if !part2 {
                break;
            }
            i += 1;
        }

        let mut new_antena_locs = antena_locs.to_owned();
        new_antena_locs.remove(1);
    }
    let mut antinodes: Vec<(usize, usize)> = Vec::new();

    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            calc(
                &[locations[i], locations[j]],
                map_size,
                &mut antinodes,
                part2,
            );
        }
    }

    antinodes
}

fn in_bounds(antinode: (i32, i32), map_size: usize) -> bool {
    (0..map_size).contains(&(antinode.0 as usize)) && (0..map_size).contains(&(antinode.1 as usize))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(answer(input), (14, 34));
    }
}
