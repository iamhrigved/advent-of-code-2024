// ANSWER WITHOUT GENERATING IMAGES!

fn main() {
    println!("DAY 14!!!");

    let input = include_str!("../../input/day14.txt");
    let (part1, part2) = answer(input);

    println!("Answer to Part 1 is {}", part1);
    println!("Answer to Part 2 is {}", part2);
}

fn answer(input: &str) -> (u32, u32) {
    let width: i32 = 101;
    let height: i32 = 103;
    let mut quad: Vec<u32> = vec![0, 0, 0, 0];
    let mut robots: Vec<((usize, usize), (i32, i32))> = Vec::new(); // stores the initial location
                                                                    // and velocity of the robots
    let mut seconds: u32 = 1;

    // Parse
    input.lines().for_each(|line| {
        let parts = line.split_once(" ").unwrap();

        let tup1_str = parts.0.split("=").nth(1).unwrap().split_once(",").unwrap();
        let tup2_str = parts.1.split("=").nth(1).unwrap().split_once(",").unwrap();

        let tup1: (usize, usize) = (tup1_str.0.parse().unwrap(), tup1_str.1.parse().unwrap());
        let tup2: (i32, i32) = (tup2_str.0.parse().unwrap(), tup2_str.1.parse().unwrap());

        robots.push((tup1, tup2));
    });

    // Part 1
    for robot in &mut robots {
        let mut x = (robot.0 .0 as i32 + (robot.1 .0 * 100)) % width; // directly multiply with 100
        let mut y = (robot.0 .1 as i32 + (robot.1 .1 * 100)) % height;

        // if modulus returns -ve
        if x < 0 {
            x += width;
        }
        if y < 0 {
            y += height;
        }

        // count number of robots in each quadrant
        if x < (width / 2) && y < (height / 2) {
            quad[0] += 1;
        }
        if x > (width / 2) && y < (height / 2) {
            quad[1] += 1;
        }
        if x < (width / 2) && y > (height / 2) {
            quad[2] += 1;
        }
        if x > (width / 2) && y > (height / 2) {
            quad[3] += 1;
        }
    }

    // Part 2
    // When the Christmas tree is formed, the robots are very close to each other, so we will find
    // the second, when the average distance between the robots is the least!
    let mut lowest_avg_second: u32 = 0; // second with the lowest average distance between robots
    let mut lowest_avg = 0.0;
    let mut cur_avg = 0.0;
    while seconds <= 10000 {
        let mut prev_loc = (0, 0); // previous robot's location

        for robot in &robots {
            let mut x = (robot.0 .0 as i32 + (robot.1 .0 * seconds as i32)) % width;
            let mut y = (robot.0 .1 as i32 + (robot.1 .1 * seconds as i32)) % height;

            if x < 0 {
                x += width;
            }
            if y < 0 {
                y += height;
            }

            // calculate distance between previous robot location and the current location and
            // add it's average to the current avg distance
            cur_avg += (((x - prev_loc.0).pow(2) + (y - prev_loc.1).pow(2)) as f32).sqrt()
                / robots.len() as f32;

            prev_loc = (x, y); // reset previous location
        }

        if seconds == 1 || cur_avg < lowest_avg {
            lowest_avg = cur_avg;
            lowest_avg_second = seconds;
        }
        cur_avg = 0.0;

        seconds += 1;
    }

    (quad[0] * quad[1] * quad[2] * quad[3], lowest_avg_second)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        println!("NOTE: To test this program, please change the width to 11 and the height to 7 in the `answer()` function.");
        assert_eq!(answer(input).0, 12);
    }
}
