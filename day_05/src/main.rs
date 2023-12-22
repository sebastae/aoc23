use std::ops::Range;
use std::str::FromStr;

type AocError = String;
type Number = u64;

#[derive(Debug, PartialEq)]
struct Seeds(Vec<Number>);

impl FromStr for Seeds {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, seeds) = s.split_once(":").ok_or(AocError::from("split seed line"))?;
        let seeds = seeds
            .split_ascii_whitespace()
            .map(|seed| {
                seed.parse::<Number>()
                    .map_err(|e| format!("parse seed value ({seed}): {e}"))
            })
            .collect::<Result<Vec<Number>, AocError>>()?;

        Ok(Seeds(seeds))
    }
}

#[derive(Debug, PartialEq)]
struct Mapping {
    dest: Range<Number>,
    src: Range<Number>,
}

impl FromStr for Mapping {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse::<Number>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<Number>, AocError>>()?;

        if nums.len() != 3 {
            return Err(AocError::from("too few numbers in mapping"));
        }

        Ok(Mapping::new(
            *nums.get(0).unwrap(),
            *nums.get(1).unwrap(),
            *nums.get(2).unwrap(),
        ))
    }
}

impl Mapping {
    fn new(dst: Number, src: Number, len: Number) -> Mapping {
        Mapping {
            dest: dst..(dst + len),
            src: src..(src + len),
        }
    }

    fn map(&self, n: Number) -> Option<Number> {
        if self.src.contains(&n) {
            let offset = n - self.src.start;
            Some(self.dest.start + offset)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct MappingTable {
    from_label: String,
    to_label: String,
    mappings: Vec<Mapping>,
}

impl FromStr for MappingTable {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();

        let header = s
            .lines()
            .take(1)
            .reduce(|_, l| l)
            .ok_or(AocError::from("getting header line"))?;

        let (from, to) = header
            .split_once("-to-")
            .ok_or(AocError::from("split header"))?;

        let mappings = lines
            .skip(1)
            .map(|l| l.parse::<Mapping>())
            .collect::<Result<Vec<Mapping>, AocError>>()?;

        Ok(MappingTable {
            from_label: from.to_owned(),
            to_label: to
                .split_once(" ")
                .ok_or(AocError::from("split header to-part"))?
                .0
                .to_owned(),
            mappings,
        })
    }
}

impl MappingTable {
    fn map(&self, n: Number) -> Number {
        for mapping in &self.mappings {
            if let Some(res) = mapping.map(n) {
                return res;
            }
        }

        return n;
    }
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Seeds,
    mapping_tables: Vec<MappingTable>,
}

impl FromStr for Almanac {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove carriage-returns because windows >:(
        let s = s.replace("\r", "");
        let mut sections = s.split("\n\n");

        let seeds_line = sections.next().ok_or(AocError::from("empty almanac"))?;
        let seeds = seeds_line.parse::<Seeds>()?;

        let mapping_tables = sections
            .map(|sec| sec.parse::<MappingTable>())
            .collect::<Result<Vec<MappingTable>, AocError>>()?;

        Ok(Almanac {
            seeds,
            mapping_tables,
        })
    }
}

impl Almanac {
    fn get_mapped_seeds(&self) -> Vec<Number> {
        self.seeds
            .0
            .iter()
            .map(|seed| {
                self.mapping_tables
                    .iter()
                    .fold(*seed, |s, table| table.map(s))
            })
            .collect::<Vec<Number>>()
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input.txt").expect("read file");
    let almanac = input.parse::<Almanac>().expect("parse almanac");

    let locations = almanac.get_mapped_seeds();

    println!("Part 1: {}", locations.iter().min().unwrap());
}

#[cfg(test)]
mod test {

    use crate::*;
    use test_case::test_case;

    #[test]
    fn it_parses_mapping() {
        assert_eq!(
            Mapping::from_str("2 4 2"),
            Ok(Mapping {
                dest: 2..4,
                src: 4..6
            })
        )
    }

    #[test_case((50, 98, 2), 98, Some(50))]
    #[test_case((50, 98, 2), 99, Some(51))]
    #[test_case((50, 98, 2), 100, None)]
    #[test_case((50, 98, 2), 17, None)]
    #[test_case((50, 98, 0), 98, None)]
    fn it_maps_correctly((dst, src, len): (Number, Number, Number), from: Number, to: Option<Number>) {
        let mapping = Mapping::new(dst, src, len);

        assert_eq!(mapping.map(from), to);
    }

    #[test]
    fn it_constructs_mapping_table() {
        const INPUT: &str = "seed-to-soil map:\n50 98 2\n52 50 48";

        let table = MappingTable {
            from_label: String::from("seed"),
            to_label: String::from("soil"),
            mappings: vec![Mapping::new(50, 98, 2), Mapping::new(52, 50, 48)],
        };

        assert_eq!(MappingTable::from_str(INPUT), Ok(table));
    }

    #[test_case(98, 50)]
    #[test_case(56, 58)]
    #[test_case(17, 17)]
    fn it_maps_with_table(from: Number, to: Number) {
        const INPUT: &str = "seed-to-soil map:\n50 98 2\n52 50 48";

        let table = MappingTable::from_str(INPUT).unwrap();

        assert_eq!(table.map(from), to);
    }

    #[test]
    fn it_solves_part_1_example() {
        const INPUT: &str = include_str!("./example.txt");

        let almanac = INPUT.parse::<Almanac>().unwrap();
        let locations = almanac.get_mapped_seeds();

        assert_eq!(*locations.iter().min().unwrap(), 35Number);
    }
}
