use std::{mem::replace, str::Chars};

use rayon::{iter::ParallelIterator, str::ParallelString};

type Output = u64;

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn parse(s: &'_ str) -> impl ParallelIterator<Item = (usize, Chars<'_>)> {
    s.par_lines().map(|l| (l.len(), l.chars()))
}

#[must_use]
pub fn part1<'a>(parsed_input: impl ParallelIterator<Item = (usize, Chars<'a>)>) -> Output {
    parsed_input
        .map(|(_, chars)| {
            let mut chars = chars.rev();
            let mut second_digit = chars.next().unwrap();
            let mut first_digit = chars.next().unwrap();

            for c in chars {
                if c >= first_digit {
                    if first_digit > second_digit {
                        second_digit = first_digit;
                    }
                    first_digit = c;
                }
            }
            first_digit
                .to_digit(10)
                .expect("Only number character expected") as Output
                * 10
                + second_digit
                    .to_digit(10)
                    .expect("Only number character expected") as Output
        })
        .sum()
}

#[must_use]
pub fn part2<'a>(parsed_input: impl ParallelIterator<Item = (usize, Chars<'a>)>) -> Output {
    parsed_input
        .map(|(_, chars)| {
            let mut chars = chars.rev();
            let mut batteries= Vec::with_capacity(12);
            for _ in 0..12 {
                batteries.push(chars.next().unwrap());
            }

            batteries.reverse();

            for c in chars {
                let mut c = c;
                for battery in &mut batteries {
                    if c < *battery {
                        break;
                    }
                    c = replace(battery, c);
                }
            }

            batteries.iter().fold(0, |joltage, digit| joltage * 10 + digit.to_digit(10).unwrap() as Output)
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use crate::{Output, parse, part1, part2};
    use common::take_input;

    #[test]
    fn parse_test() {
        let test = take_input("test.txt");
        parse(&test);
    }

    #[test]
    fn part1_test() {
        let test = take_input("test.txt");
        let parsed = parse(&test);

        let my_ans = part1(parsed);

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
        let parsed = parse(&test);

        let my_ans = part2(parsed);

        let ans = take_input("test_ans.txt")
            .lines()
            .nth(1)
            .expect("Enter the answer for part 2 example problem")
            .parse::<Output>()
            .expect("Number?");

        assert_eq!(my_ans, ans);
    }
}
