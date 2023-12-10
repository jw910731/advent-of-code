use std::fs::{File, ReadDir};
use std::io::{ErrorKind, Read, Write};
use std::path::Path;
use std::{fs, io};

use reqwest::header::COOKIE;
use reqwest::{Client, Url};

mod y2023;

pub struct Aoc {
    year: i32,
}

fn create_dir_if_not_exist(path: &Path) -> io::Result<ReadDir> {
    match fs::read_dir(path) {
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                fs::create_dir(path)?;
                fs::read_dir(path)
            } else {
                Err(e)
            }
        }
        Ok(dir) => Ok(dir),
    }
}

impl Aoc {
    pub fn new(year: i32) -> Aoc {
        Aoc { year }
    }

    pub async fn solve_with_input(&self, date: i32, input: String) -> Result<String, String> {
        if date <= 0 || date > 31 {
            panic!("date must be in [1, 31]")
        }
        let solutions = match self.year {
            2023 => y2023::get_solutions(),
            _ => todo!(),
        };
        Ok(solutions[(date - 1) as usize](input))
    }

    pub async fn solve(&self, date: i32) -> Result<String, String> {
        let input = self.get_input(date).await?;
        self.solve_with_input(date, input).await
    }

    async fn get_input(&self, date: i32) -> Result<String, String> {
        create_dir_if_not_exist(Path::new(".cache"))
            .expect("Error when create or access directory");
        let cache_file_path_str = format!(".cache/{}.in", date);
        let cache_file_path = Path::new(&cache_file_path_str);
        // fetch data from internet or not based on cache file existence
        let fetch: bool = fs::metadata(cache_file_path.clone()).is_err();
        let mut file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(&cache_file_path)
            .expect("Cannot open file");
        if fetch {
            let url = format!("https://adventofcode.com/{}/day/{}/input", self.year, date)
                .parse::<Url>()
                .unwrap();
            let cookie = format!(
                "session={}",
                std::env::var("SESS").expect("SESS environment variable")
            );

            let client = Client::new();
            let result = client.get(url).header(COOKIE, cookie).send().await.unwrap();
            let status = result.status();
            let text = result.text().await.unwrap();
            if status.is_client_error() {
                return Err(format!("Client Error: {}", text));
            }
            if status.is_server_error() {
                return Err(format!("Internal Server Error: {}", text));
            }
            if let Err(e) = file.write_all(text.clone().into_bytes().by_ref()) {
                eprintln!("Warning: Failed to write cache: {}", e.to_string());
            }
            Ok(text)
        } else {
            let mut text = String::new();
            file.read_to_string(&mut text)
                .expect("Error when reading file");
            Ok(text)
        }
    }
}
