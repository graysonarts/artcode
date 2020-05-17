use artcode::BatDate;
use chrono::prelude::*;
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("date")
                .required(false)
                .help("Date to convert in YYYYMMDD format"),
        )
        .get_matches();

    let date: BatDate = match matches.value_of("date") {
        Some(value) => NaiveDate::parse_from_str(value, "%Y%m%d").unwrap().into(),
        None => Local::today().into(),
    };

    println!("{}", date);
}
