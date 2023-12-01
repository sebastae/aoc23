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

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn convert_text_numbers(line: &str) -> String {
    DIGITS
        .iter()
        .enumerate()
        .fold(line.to_owned(), |l, (i, &digit)| {
            l.replace(digit, &(i+1).to_string())
        })
}

fn sum_text_lines(s: &str) -> i32 {
    s.lines().map(|l| convert_text_numbers(l)).map(|l| find_embedded_number(&l)).sum()
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1 Sum: {}", sum_lines(input));


}

#[cfg(test)]
mod test {

    use crate::{convert_text_numbers, find_embedded_number, sum_lines, sum_text_lines};
    use test_case::test_case;

    const INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const INPUT2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    fn test_find_nums(line: &str, target: i32) {
        assert_eq!(find_embedded_number(line), target)
    }

    #[test_case(INPUT, 142)]
    fn test_sum_lines(s: &str, target: i32) {
        assert_eq!(sum_lines(s), target)
    }

    #[test_case("two1nine", "219")]
    #[test_case("eighttwothree", "823")]
    #[test_case("abcone2threexyz", "abc123xyz")]
    #[test_case("xtwone3four", "x2ne34")]
    #[test_case("4nineeightseven2", "49872")]
    #[test_case("zoneight234", "z1ight234")]
    #[test_case("7pqrstsixteen", "7pqrst6teen")]
    fn test_convert_string_digits(line: &str, result: &str) {
        assert_eq!(convert_text_numbers(line), result);
    }

    #[test_case(INPUT2, 281)]
    fn test_sum_string_numbers(input: &str, result: i32) {
        assert_eq!(sum_text_lines(input), result)
    }

}
