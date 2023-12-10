use reqwest::{Client, Url};
use reqwest::header::{COOKIE, USER_AGENT};

mod y2023;

pub struct Aoc {
    year: i32,
}

impl Aoc {
    pub fn new(year: i32) -> Aoc {
        Aoc{ year }
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

    async fn get_input(&self, date: i32) -> Result<String, String>{
        let url = format!("https://adventofcode.com/{}/day/{}/input", self.year, date).parse::<Url>().unwrap();
        let cookie = format!("session={}", std::env::var("SESS").expect("SESS environment variable"));

        let client = Client::new();
        let result = client.get(url)
            .header(COOKIE, cookie)
            .send().await
            .unwrap();
        let status = result.status();
        let text = result.text().await.unwrap();
        if status.is_client_error() {
            return Err(format!("Client Error: {}", text))
        }
        if status.is_server_error() {
            return Err(format!("Internal Server Error: {}", text))
        }
        Ok(text)
    }
}
