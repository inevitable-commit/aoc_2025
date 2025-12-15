use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    range_inclusive::Iter,
    str::ParallelString,
};

type Output = u64;

#[allow(clippy::missing_panics_doc)]
pub fn parse(s: &str) -> impl ParallelIterator<Item = Iter<Output>> {
    s.trim_end().par_split(',').map(|s| {
        let mut it = s.split('-').map(|n| n.parse().expect("Always correct"));
        (it.next().expect("Always correct")..=it.next().expect("Always correct")).into_par_iter()
    })
}

#[must_use]
pub fn part1(parsed_input: impl ParallelIterator<Item = Iter<Output>>) -> Output {
    parsed_input
        .map(|range| {
            range.filter(|&n| {
                let digits = f64::log10(n as f64) as u32 + 1;
                digits.is_multiple_of(2) && {
                    let half = 10u64.pow(digits / 2);
                    n / half == n % half
                }
            })
        })
        .flatten()
        .sum()
}

#[must_use]
pub fn part2(parsed_input: impl ParallelIterator<Item = Iter<Output>>) -> Output {
    parsed_input
        .map(|range| {
            range.filter(|&n| {
                let digits = f64::log10(n as f64) as u32 + 1;

                let mut window = 10u64.pow(digits / 2);

                for size in (1..=(digits / 2)).rev() {
                    let pattern = n % window;

                    if digits.is_multiple_of(size) && n.is_multiple_of(pattern) {
                        let mut num = n / window;

                        let mut correct = true;
                        for _ in 0..(digits / size - 1) {
                            if pattern == num % window {
                                num /= window;
                            } else {
                                correct = false;
                                break;
                            }
                        }

                        if correct {
                            return true;
                        }
                    }
                    window /= 10;
                }
                false
            })
        })
        .flatten()
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{Output, parse, part1, part2};
    use common::take_input;
    use rayon::iter::ParallelIterator;

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
