use std::env;
use std::io::stdin;

mod aoc;

#[tokio::main]
async fn main()-> Result<(), String> {
    let aoc = aoc::Aoc::new(2023);
    let args: Vec<String> = env::args().collect();
    let arg = &args[1];
    if arg.contains("test") {
        let input = stdin().lines().map(Result::unwrap).collect::<Vec<_>>().join("\n");
        let arg = arg.replace("test", "");
        println!("{}", aoc.solve_with_input(arg.parse().unwrap(), input).await?);
    }
    else {
        println!("{}", aoc.solve(arg.parse().unwrap()).await?);
    }
    Ok(())
}
