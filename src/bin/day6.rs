// PART 2 TAKES A LOT OF TIME (T_T)

use std::collections::HashSet;

fn main() {
    println!("DAY 6!!!");

    let input = include_str!("../../input/day6.txt");
    let (part1, part2) = answer(input);

    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

static DIR: &[(i32, i32)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position(usize, usize);

fn answer(input: &str) -> (usize, usize) {
    let mut count = (0, 0);

    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<_>>>();

    let len = map.len(); // given that input is a square

    let mut path: HashSet<Position> = HashSet::new();
    let mut starting_pos = Position(0, 0);

    for i in 0..len {
        for j in 0..len {
            if map[i][j] == '^' {
                // guard will always start looking up
                starting_pos.0 = j; // store the starting '^' position
                starting_pos.1 = i;

                count.0 += 1; // count the starting position
                path.insert(Position(j, i)); // add to path
                break;
            }
        }
    }

    let mut cur_pos = starting_pos.clone();
    let mut dir = 0; // going up by default X, Y - 1

    loop {
        let peek = peek_forward(&cur_pos, dir); // peek the next position

        if is_out_of_map(&peek, len) {
            // if next step is out of map
            break;
        }

        if map[peek.1][peek.0] == '#' {
            // if next step is #
            dir = turn_right(dir); // just turn right
            continue;
        }

        cur_pos = peek; // actually move forward

        if !path.contains(&cur_pos) {
            // if new position
            count.0 += 1; // count
        }

        path.insert(cur_pos.clone()); // remember path
    }

    for cur_pos in path {
        map[cur_pos.1][cur_pos.0] = '#'; // add the obstacle in the next position on the path

        if check_loop(&map, &starting_pos) {
            // if loop found
            count.1 += 1;
        }

        map[cur_pos.1][cur_pos.0] = '.'; // remove the obstacle for the next iteration
    }

    count
}

fn is_out_of_map(pos: &Position, map_size: usize) -> bool {
    !(0..map_size).contains(&pos.0) || !(0..map_size).contains(&pos.1)
}

fn peek_forward(pos: &Position, dir: usize) -> Position {
    let mut next_pos = pos.clone();
    next_pos.0 = (pos.0 as i32 + DIR[dir].0) as usize;
    next_pos.1 = (pos.1 as i32 + DIR[dir].1) as usize;

    next_pos
}

fn turn_right(dir: usize) -> usize {
    (dir + 1) % 4
}

// check if loops
fn check_loop(map: &[Vec<char>], starting_pos: &Position) -> bool {
    let mut cur_dir = 0;
    let mut cur_pos = starting_pos.clone();
    let len = map.len();
    let mut path_history: Vec<bool> = Vec::from([false; 130 * 130 * 4]);
    // 3d boolean array storing the current position (H*W) and the direction (4) of the guard

    loop {
        let peek = peek_forward(&cur_pos, cur_dir);

        if is_out_of_map(&peek, map.len()) {
            return false;
        }

        if map[peek.1][peek.0] == '#' {
            // if next step is #
            cur_dir = turn_right(cur_dir); // just turn right
            continue;
        }

        cur_pos = peek; // actually move forward

        let hash = (cur_pos.0 * len + cur_pos.1) * 4 + cur_dir; // current state of the guard
        if path_history[hash] {
            // if at the same position, the direction is the same, we have found a loop!
            return true;
        } else {
            path_history[hash] = true;
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
