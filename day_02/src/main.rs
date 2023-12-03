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

    fn find_all_viable(s: &str, set: &CubeSet) -> Result<Vec<Self>, ParseGameErr> {
        let games = s
            .lines()
            .map(Game::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        // .iter() creates an iterator over references, which cannot be collected into an owned vec
        // Instead we need a consuming iterator that takes ownership of its values, we can do this with .into_iter()
        let games = games
            .into_iter()
            .filter(|g| g.is_viable_with_set(set))
            .collect();

        Ok(games)
    }

    fn sum_ids(games: Vec<Game>) -> u32 {
        games.iter().map(|g| g.id).sum()
    }
}

fn main() -> Result<(), ParseGameErr> {
    // Part 1
    const INPUT: &str = include_str!("./input.txt");

    let games = Game::find_all_viable(
        INPUT,
        &CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        },
    )?;

    let sum = Game::sum_ids(games);

    println!("Part 1: Sum of viable IDs: {sum}");

    Ok(())
}
