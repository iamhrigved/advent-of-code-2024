// PART 2 TAKES A LOT OF TIME (T_T)

use std::collections::{HashMap, HashSet};

fn main() {
    println!("DAY 6!!!");

    let input = include_str!("../../input/day6.txt");
    let (part1, part2) = answer(input);

    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position(usize, usize);

#[derive(Clone, PartialEq)]
struct Direction(i32, i32);

fn answer(input: &str) -> (usize, usize) {
    let mut count = (0, 0);

    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<_>>>();

    let len = map.len(); // given that input is a square

    let mut path: Vec<Position> = Vec::new();
    let mut starting_pos = Position(0, 0);

    for i in 0..len {
        for j in 0..len {
            if map[i][j] == '^' {
                // guard will always start looking up
                starting_pos.0 = j; // store the starting '^' position
                starting_pos.1 = i;

                count.0 += 1; // count the starting position
                path.push(Position(j, i)); // add to path
            }
        }
    }

    let mut cur_pos = starting_pos.clone();
    let mut dir = Direction(0, -1); // going up by default X, Y - 1

    loop {
        let peek = peek_forward(&cur_pos, &dir); // peek the next position

        if is_out_of_map(&peek, len) {
            // if next step is out of map
            break;
        }

        if map[peek.1][peek.0] == '#' {
            // if next step is #
            dir = turn_right(&dir); // just turn right
            continue;
        }

        cur_pos = peek; // actually move forward

        if !path.contains(&cur_pos) {
            // if new position
            count.0 += 1; // count
        }

        path.push(cur_pos.clone()); // remember path
    }

    delete_dupes(&mut path); // because we can end up at the same position in the next loop, which
                             // will result in multiple counting

    for i in 0..path.len() - 1 {
        let next_pos = path[i + 1].clone();

        map[next_pos.1][next_pos.0] = '#'; // add the obstacle in the next position on the path

        if check_loop(&map, &starting_pos) {
            // if loop found
            count.1 += 1;
        }

        map[next_pos.1][next_pos.0] = '.'; // remove the obstacle for the next iteration
    }

    count
}

fn is_out_of_map(pos: &Position, map_size: usize) -> bool {
    !(0..map_size).contains(&pos.0) || !(0..map_size).contains(&pos.1)
}

fn peek_forward(pos: &Position, dir: &Direction) -> Position {
    let mut next_pos = pos.clone();
    next_pos.0 = (pos.0 as i32 + dir.0) as usize;
    next_pos.1 = (pos.1 as i32 + dir.1) as usize;

    next_pos
}

fn turn_right(dir: &Direction) -> Direction {
    let cur_dir = (dir.0, dir.1);

    if cur_dir == (0, -1) {
        return Direction(1, 0);
    }
    if cur_dir == (0, 1) {
        return Direction(-1, 0);
    }
    if cur_dir == (1, 0) {
        return Direction(0, 1);
    }
    if cur_dir == (-1, 0) {
        return Direction(0, -1);
    }
    Direction(0, 0)
}

// deletes duplicates from a vector
fn delete_dupes<T: std::cmp::Eq + std::hash::Hash + Clone>(list: &mut Vec<T>) {
    let mut seen: HashSet<T> = HashSet::new();
    list.retain_mut(|pos| seen.insert(pos.clone())); // HashSet::insert() returns false if the
                                                     // element was already in the set
}

// check if loops
fn check_loop(map: &[Vec<char>], starting_pos: &Position) -> bool {
    let mut cur_dir = Direction(0, -1);
    let mut cur_pos = starting_pos.clone();
    let mut path_history: HashMap<Position, Direction> = HashMap::new();
    // stores the direction associated with a position

    loop {
        let peek = peek_forward(&cur_pos, &cur_dir);

        if is_out_of_map(&peek, map.len()) {
            return false;
        }

        if map[peek.1][peek.0] == '#' {
            // if next step is #
            cur_dir = turn_right(&cur_dir); // just turn right
            continue;
        }

        cur_pos = peek; // actually move forward

        if let Some(stored_dir) = path_history.get(&cur_pos) {
            if *stored_dir == cur_dir {
                // if at the same position, the direction is the same, we have found a loop!
                return true;
            }
        } else {
            path_history.insert(cur_pos.clone(), cur_dir.clone()); // record history if at new position
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_answer() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(answer(input), (41, 6));
    }
}
