use itertools::{Itertools};
use std::cmp::max;

struct Cubes {
    red: i64,
    green: i64,
    blue: i64,
}

impl Cubes {
    fn parse(input: &str) -> Cubes{
        let colors = ["red", "green", "blue"];
        // input example: "3 blue, 4 red"
        let raw_cubes: Vec<_> = input
            .split(",")
            .flat_map(|s| s.split_whitespace())
            .collect();
        let (red, green, blue) = colors
            .iter()
            .map(|color| {
                if let Some((pos, _)) = raw_cubes
                    .iter()
                    .find_position(|x| (***x).eq(*color)) {
                    raw_cubes[pos - 1].parse::<i64>().unwrap()
                } else {
                    0
                }
            })
            .collect_tuple()
            .unwrap();
        Cubes{
            red, green, blue
        }
    }

    fn max(&mut self, cube: &Cubes) {
        self.red  = max(self.red, cube.red);
        self.green  = max(self.green, cube.green);
        self.blue  = max(self.blue, cube.blue);
    }
}

pub fn day2(input: String) -> String{
    input
        .lines()
        .map(|line| {
            let (game_str, content_str) = line.split(":").collect_tuple().unwrap();
            let game_number: i64 = game_str.replace("Game ", "").parse().unwrap();
            let rounds: Vec<_> = content_str
                .split(";")
                .map(Cubes::parse)
                .collect();
            let mut max_cube = Cubes{red: 0, green: 0, blue: 0};
            for cube in rounds {
                max_cube.max(&cube);
            }
            (game_number, max_cube)
        })
        // Part one
        // .filter(|(_, cube)| cube.red <= 12 && cube.green <= 13 && cube.blue <= 14)
        // .map(|(i, _)| i)
        .map(|(_, x)| x.red * x.green * x.blue)
        .sum::<i64>()
        .to_string()
}
