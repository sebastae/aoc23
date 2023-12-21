use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    line: usize,
    index: usize,
}

impl Location {
    fn new(line: usize, index: usize) -> Self {
        Location { line, index }
    }
}

#[derive(Debug, PartialEq)]
struct Number {
    number: u32,
    location: Location,
}

impl Number {
    fn new(number: u32, line: usize, index: usize) -> Self {
        Number {
            number,
            location: Location::new(line, index),
        }
    }

    fn get_adjacent_locations(&self) -> Vec<Location> {
        let len = (self.number as f32).log10() as i32;
        let range = (self.location.index as i32 - 1)..=(self.location.index as i32 + len + 1);
        let ln_over: Vec<(i32, i32)> = range
            .clone()
            .map(|i| (self.location.line as i32 - 1, i))
            .collect();

        let ln_same = vec![
            (self.location.line as i32, self.location.index as i32 - 1),
            (
                self.location.line as i32,
                self.location.index as i32 + len + 1,
            ),
        ];

        let ln_next: Vec<(i32, i32)> = range.map(|i| (self.location.line as i32 + 1, i)).collect();

        vec![ln_over, ln_same, ln_next]
            .iter()
            .flatten()
            .filter(|(l, i)| *l >= 0 && *i >= 0)
            .map(|(l, i)| Location::new(*l as usize, *i as usize))
            .collect()
    }
}

type Symbol = String;
type SymbolTable = HashMap<Location, Symbol>;

#[derive(Debug, PartialEq)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: SymbolTable,
}

#[derive(Debug)]
struct ParseSchematicError;

impl FromStr for Schematic {
    type Err = ParseSchematicError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut schematic = Schematic {
            numbers: vec![],
            symbols: HashMap::new(),
        };

        s.lines().enumerate().for_each(|(li, l)| {
            let mut it = l.char_indices();
            let mut num = 0;

            while let Some((i, c)) = it.next() {
                if c.is_ascii_digit() {
                    // While we're reading a number, construct the number
                    num = num * 10 + c.to_digit(10).unwrap();
                } else if num != 0 {
                    // When we're done; push the number to the numbers vec
                    let n_idx = i - ((num as f32).log10() as usize) - 1;
                    schematic.numbers.push(Number::new(num, li, n_idx));
                    num = 0;
                }

                if !c.is_ascii_digit() && c != '.' {
                    schematic
                        .symbols
                        .insert(Location::new(li, i), c.to_string());
                }
            }

            if num != 0 {
                let ni = l.len() - ((num as f32).log10() as usize) - 1;
                schematic.numbers.push(Number::new(num, li, ni));
            }
        });

        Ok(schematic)
    }
}

impl Schematic {
    fn find_part_numbers(self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|n| {
                n.get_adjacent_locations()
                    .iter()
                    .any(|l| self.symbols.contains_key(l))
            })
            .map(|n| n.number)
            .collect()
    }
}

fn part_1(input: &str) -> Result<u32, ParseSchematicError> {
    let schm = Schematic::from_str(input)?;

    Ok(schm.find_part_numbers().iter().sum())
}

fn main() {
    const INPUT: &str = include_str!("./input.txt");
    println!("Part 1: {}", part_1(INPUT).unwrap())
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::*;

    #[test]
    fn it_parses_numbers() {
        const INPUT: &str = "...123..34..5..78";

        let schm = Schematic::from_str(INPUT).unwrap();

        let expect = vec![
            Number::new(123, 0, 3),
            Number::new(34, 0, 8),
            Number::new(5, 0, 12),
            Number::new(78, 0, 15),
        ];

        assert_eq!(schm.numbers, expect);
    }

    #[test]
    fn it_parses_symbols() {
        const INPUT: &str = "...*123..#.4$";

        let schm = Schematic::from_str(INPUT).unwrap();

        let mut expect = HashMap::new();
        expect.insert(Location::new(0, 3), String::from("*"));
        expect.insert(Location::new(0, 9), String::from("#"));
        expect.insert(Location::new(0, 12), String::from("$"));

        assert_eq!(schm.symbols, expect);
    }

    #[test]
    fn it_parses_both() {
        const INPUT: &str = "...*123..#.4$";

        let schm = Schematic::from_str(INPUT).unwrap();

        let expect_num = vec![Number::new(123, 0, 4), Number::new(4, 0, 11)];
        let mut expect_sym = HashMap::new();
        expect_sym.insert(Location::new(0, 3), String::from("*"));
        expect_sym.insert(Location::new(0, 9), String::from("#"));
        expect_sym.insert(Location::new(0, 12), String::from("$"));

        let expect = Schematic {
            numbers: expect_num,
            symbols: expect_sym,
        };

        assert_eq!(schm, expect);
    }

    #[test]
    fn it_solves_part_1() {
        const INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

        assert_eq!(part_1(INPUT).unwrap(), 4361);
    }
}
