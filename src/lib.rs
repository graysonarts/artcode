use chrono::prelude::*;
use std::fmt::{self, Display};

const CHARACTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";
const BASE: u32 = CHARACTERS.len() as u32;

pub fn encode(value: u32) -> String {
    let mut retval = Vec::<String>::new();
    let mut rest = value;
    while rest > 0 {
        let digit: usize = (rest % BASE) as usize;
        rest = rest / BASE;
        retval.push(format!("{}", CHARACTERS.get(digit - 1..digit).unwrap()));
    }

    retval.join("")
}

#[derive(Debug)]
pub struct BatDate {
    year: i32,
    cycle_count: u32,
    segment: u32,
    week_day: Weekday,
}

impl Display for BatDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Example: RYBc2.2
        write!(
            f,
            "{}{}{}x{}",
            encode(self.year as u32).to_lowercase(),
            encode(self.cycle_count),
            self.segment + 1,
            self.week_day.num_days_from_monday() + 1
        )
    }
}

impl From<DateTime<Local>> for BatDate {
    fn from(d: DateTime<Local>) -> BatDate {
        BatDate {
            year: d.year(),
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
            week_day: d.weekday(),
            cycle_count: d.iso_week().week() / 2,
            segment: d.iso_week().week() % 2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_naive_date() {
        let date: BatDate = NaiveDate::parse_from_str("20200517", "%Y%m%d")
            .unwrap()
            .into();
        assert_eq!(date.to_string(), "RYBj1x7".to_owned());
    }

    #[test]
    fn test_valid_datetime() {
        let date: BatDate = Local.with_ymd_and_hms(2020, 5, 17, 0, 0, 0).unwrap().into();
        assert_eq!(date.to_string(), "RYBj1x7".to_owned());
    }
}
