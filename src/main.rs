use artcode::BatDate;
use chrono::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(required = false, help = "Date to convert in YYYYMMDD format")]
    date: Option<String>,
}

fn main() {
    let args = Args::parse();
    let date: BatDate = match args.date {
        None => Local::now().into(),
        Some(value) => NaiveDate::parse_from_str(&value, "%Y%m%d").unwrap().into(),
    };

    println!("{}", date);
}
