use std::str::FromStr;

#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

struct ParseCubeStructError;

impl FromStr for CubeSet {
    type Err = ParseCubeStructError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = CubeSet::default();

        s.trim().split(',').try_for_each(|c| {
            let (num, color) = c.trim().split_once(' ').ok_or(ParseCubeStructError)?;

            let num = num.parse::<u32>().map_err(|_| ParseCubeStructError)?;

            match color.trim() {
                "red" => {
                    set.red = num;
                    Ok(())
                }
                "green" => {
                    set.green = num;
                    Ok(())
                }
                "blue" => {
                    set.blue = num;
                    Ok(())
                }

                _ => Err(ParseCubeStructError),
            }
        })?;

        Ok(set)
    }
}

impl CubeSet {
    // The power of a set is the factor of its components
    fn get_power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug, Default)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

#[derive(Debug)]
struct ParseGameErr;

impl FromStr for Game {
    type Err = ParseGameErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, game) = s.trim().split_once(':').ok_or(ParseGameErr)?;

        let id = id
            .strip_prefix("Game ")
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or(ParseGameErr)?;

        let sets = game
            .trim()
            .split(';')
            .map(CubeSet::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseGameErr)?;

        Ok(Game { id, sets })
    }
}

impl Game {
    fn is_viable_with_set(&self, set: &CubeSet) -> bool {
        // Check that all sets in the game does not pull more cubes that the provided config
        self.sets
            .iter()
            .all(|s| s.red <= set.red && s.green <= set.green && s.blue <= set.blue)
    }

    fn parse_all(s: &str) -> Result<Vec<Game>, ParseGameErr> {
        s.lines().map(Game::from_str).collect()
    }

    fn find_viable_for_set<'a>(games: &'a Vec<Game>, set: &CubeSet) -> Vec<&'a Game> {
        games
            .into_iter()
            .filter(|g| g.is_viable_with_set(set))
            .collect()
    }

    fn sum_ids(games: &Vec<&Game>) -> u32 {
        games.iter().map(|g| g.id).sum()
    }

    // Find the minimum possible number of cubes for a game
    fn find_min_set(&self) -> CubeSet {
        let mut set = CubeSet::default();

        self.sets.iter().for_each(|s| {
            set.red = set.red.max(s.red);
            set.green = set.green.max(s.green);
            set.blue = set.blue.max(s.blue);
        });

        set
    }
}

fn main() -> Result<(), ParseGameErr> {
    const INPUT: &str = include_str!("./input.txt");
    let games = Game::parse_all(INPUT)?;

    // Part 1
    let viable = Game::find_viable_for_set(
        &games,
        &CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        },
    );

    let sum = Game::sum_ids(&viable);

    println!("Part 1: Sum of viable IDs: {sum}");

    // Part 2
    let power_sum: u32 = games
        .iter()
        .map(|g| g.find_min_set())
        .map(|s| s.get_power())
        .sum();

    println!("Part 2: Sum of min set power: {power_sum}");

    Ok(())
}
