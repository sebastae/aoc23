use regex::Regex;

const DIGITS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

#[derive(PartialEq, PartialOrd, Debug)]
struct Digit {
    index: usize,
    value: i32,
}

impl Digit {
    fn new(index: usize, value: &str) -> Digit {
        Digit {
            index,
            value: match value {
                "1" | "one" => 1,
                "2" | "two" => 2,
                "3" | "three" => 3,
                "4" | "four" => 4,
                "5" | "five" => 5,
                "6" | "six" => 6,
                "7" | "seven" => 7,
                "8" | "eight" => 8,
                "9" | "nine" => 9,
                _ => 0,
            },
        }
    }

    // Returns all digits found in the line in the order they appear
    fn extract_all(line: &str) -> Vec<Digit> {
        let mut digits = Vec::new();

        DIGITS.iter().for_each(|&digit| {
            let re = Regex::new(digit);
            if let Ok(re) = re {
                re.find_iter(line)
                    .for_each(|m| digits.push(Digit::new(m.start(), m.as_str())))
            }
        });

        digits
    }
}

fn combine_outer_digits(digits: &Vec<Digit>) -> i32 {
    let f = digits.iter().min_by_key(|d| d.index);
    let l = digits.iter().max_by_key(|d| d.index);

    f.unwrap_or(&Digit { index: 0, value: 0 }).value * 10
        + l.unwrap_or(&Digit { index: 0, value: 0 }).value
}

fn sum_digit_lines(input: &str) -> i32 {
    input
        .lines()
        .map(|l| Digit::extract_all(l))
        .map(|digits| combine_outer_digits(&digits))
        .sum()
}

// Naive Part 1 solution: 
fn find_embedded_number(line: &str) -> i32 {
    let nums = line.chars().fold(None as Option<(i32, i32)>, |acc, c| {
        if c.is_numeric() {
            let n = c.to_digit(10).unwrap_or_default() as i32;
            if let Some((tenth, _)) = acc {
                Some((tenth, n))
            } else {
                Some((n, n))
            }
        } else {
            acc
        }
    });

    if let Some((tenth, ones)) = nums {
        tenth * 10 + ones
    } else {
        0
    }
}

fn sum_lines(s: &str) -> i32 {
    s.lines().map(|l| find_embedded_number(l)).sum()
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1 Sum: {}", sum_lines(input));

    println!("Part 2 Sum: {}", sum_digit_lines(input));
}

#[cfg(test)]
mod test {

    use crate::*;
    use test_case::test_case;

    const INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const INPUT2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test_case("1", 1)]
    #[test_case("two", 2)]
    #[test_case("jdfkls", 0)]
    fn test_digit_construction(digit: &str, result: i32) {
        assert_eq!(Digit::new(0, digit).value, result)
    }

    #[test_case("1one", vec![(0, 1), (1, 1)]; "digit_and_string")]
    #[test_case("ab1threetwoone5", vec![(2, 1), (14, 5), (11, 1), (8, 2), (3, 3)]; "with_overlap")]
    fn test_digit_extract(line: &str, result: Vec<(usize, i32)>) {
        let r: Vec<Digit> = result
            .iter()
            .map(|(i, v)| Digit {
                index: *i,
                value: *v,
            })
            .collect();
        assert_eq!(Digit::extract_all(line), r)
    }

    #[test_case(vec![(0, 1), (1, 2)], 12 ; "two_digits")]
    #[test_case(vec![(0, 7)], 77 ; "one_digit")]
    #[test_case(vec![(3, 8),(5, 1), (2, 4), (4, 2)], 41; "unsorted")]
    fn test_combine_outer_digits(digits: Vec<(usize, i32)>, sum: i32) {
        assert_eq!(
            combine_outer_digits(
                &digits
                    .iter()
                    .map(|(i, v)| Digit {
                        index: *i,
                        value: *v
                    })
                    .collect()
            ),
            sum
        )
    }

    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    fn test_find_nums(line: &str, target: i32) {
        assert_eq!(combine_outer_digits(&Digit::extract_all(line)), target)
    }

    #[test_case(INPUT, 142)]
    fn test_sum_lines(s: &str, target: i32) {
        assert_eq!(sum_digit_lines(s), target)
    }

    #[test_case(INPUT2, 281)]
    fn test_sum_string_numbers(input: &str, result: i32) {
        assert_eq!(sum_digit_lines(input), result)
    }
}
