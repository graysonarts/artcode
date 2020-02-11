use std::fmt::{self, Display};
use clap::{app_from_crate, crate_name, crate_version, crate_description, crate_authors, Arg};
use chrono::prelude::*;

const CHARACTERS : &'static str = "abcdefghijklmnopqrstuvwxyz";
const BASE : u32 = CHARACTERS.len() as u32;

fn encode(value: u32) -> String {
    let mut retval = Vec::<String>::new();
    let mut rest = value;
    while rest > 0 {
        let digit: usize = (rest % BASE) as usize;
        rest = rest / BASE;
        retval.push(format!("{}", CHARACTERS.get(digit-1..digit).unwrap()));
    }

    retval.join("")
}

#[derive(Debug)]
struct BatDate {
    year: i32,
    day: u32,
    cycle_count: u32,
    segment: u32,
    week_day: Weekday,
}

impl Display for BatDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Example: RYBc2.2
        write!(f, "{}{}{}.{}",
               encode(self.year as u32).to_uppercase(),
               encode(self.cycle_count),
               self.segment + 1,
               self.week_day.num_days_from_monday() + 1)
    }
}

impl From<Date<Local>> for BatDate {
    fn from(d: Date<Local>) -> BatDate {
        BatDate {
            year: d.year(),
            day: d.day0(),
            week_day: d.weekday(),
            cycle_count: d.iso_week().week() / 2,
            segment: d.iso_week().week() % 2,
        }
    }
}

impl From<NaiveDate> for BatDate {
    fn from(d: NaiveDate) -> BatDate {
        BatDate {
            year: d.year(),
            day: d.day0(),
            week_day: d.weekday(),
            cycle_count: d.iso_week().week() / 2,
            segment: d.iso_week().week() % 2,
        }
    }
}

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name("date")
             .required(false)
             .help("Date to convert in YYYYMMDD format")
        )
        .get_matches();

    let date: BatDate = match matches.value_of("date") {
        Some(value) => NaiveDate::parse_from_str(value, "%Y%m%d").unwrap().into(),
        None => Local::today().into()
    };

    println!("{}", date);
}
