type Output = usize;

pub enum Dir {
    Left,
    Right,
}

const START_NUM: usize = 50;
const TOTAL_NUMS: usize = 100;

#[allow(clippy::missing_panics_doc)]
pub fn parse(s: &str) -> impl Iterator<Item = (Dir, usize)> {
    s.lines().map(|line| {
        let mut c = line.chars();
        let dir_char = c.next().expect("Input expected to be always correct");
        let dir = match dir_char {
            'R' => Dir::Right,
            'L' => Dir::Left,
            _ => panic!("Impossible Character"),
        };

        (
            dir,
            c.as_str()
                .parse::<usize>()
                .expect("Input expected to be always correct"),
        )
    })
}

#[must_use]
pub fn part1(parsed_input: impl Iterator<Item = (Dir, usize)>) -> Output {
    parsed_input
        .fold((START_NUM as i32, 0), |acc, (dir, steps)| {
            let pos = match dir {
                Dir::Right => (acc.0 + steps as i32) % TOTAL_NUMS as i32,
                Dir::Left => (acc.0 - steps as i32) % TOTAL_NUMS as i32, // doesn't matter if pos
                                                                         // is negative
            };

            (pos, acc.1 + if pos == 0 { 1 } else { 0 })
        })
        .1
}

#[must_use]
pub fn part2(parsed_input: impl Iterator<Item = (Dir, usize)>) -> Output {
    parsed_input
        .fold((START_NUM as i32, 0), |acc, (dir, steps)| match dir {
            Dir::Right => {
                let pos = acc.0 + steps as i32;
                (pos % TOTAL_NUMS as i32, acc.1 + pos as usize / TOTAL_NUMS)
            }

            Dir::Left => {
                let mut count = steps / TOTAL_NUMS;
                let pos = acc.0 - (steps % TOTAL_NUMS) as i32;

                if pos == 0 || (pos < 0 && acc.0 != 0) {
                    count += 1;
                }

                (pos.rem_euclid(TOTAL_NUMS as i32), acc.1 + count)
            }
        })
        .1
}

#[cfg(test)]
mod tests {
    use crate::{Output, parse, part1, part2};
    use common::take_input;

    #[test]
    fn parse_test() {
        let test = take_input("test.txt");
        parse(&test).for_each(|_| {});
    }

    #[test]
    fn part1_test() {
        let test = take_input("test.txt");
        let parse = parse(&test);

        let my_ans = part1(parse);

        let ans = take_input("test_ans.txt")
            .lines()
            .next()
            .expect("Enter the answer for part 1 example problem")
            .parse::<Output>()
            .expect("Number?");

        assert_eq!(my_ans, ans);
    }

    #[test]
    fn part2_test() {
        let test = take_input("test.txt");
        let parse = parse(&test);

        let my_ans = part2(parse);

        let ans = take_input("test_ans.txt")
            .lines()
            .nth(1)
            .expect("Enter the answer for part 2 example problem")
            .parse::<Output>()
            .expect("Number?");

        assert_eq!(my_ans, ans);
    }
}
