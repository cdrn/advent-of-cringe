mod day1;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("day1") => day1::run(),
        // Some("day2") => day2::run(),
        // ... handle other days
        _ => eprintln!("Specify the day to run"),
    }
}
