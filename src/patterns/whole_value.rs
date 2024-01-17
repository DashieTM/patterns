#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Date {
    pub year: Year,
    pub month: Month,
    pub day: Day,
}

impl Date {
    pub fn create(year: u32, month: u32, day: u32) -> Option<Self> {
        let explicit_year = Year::get(year);
        let explicit_month = Month::get(month);
        let explicit_day = Day::get(day);
        if explicit_year.is_none() || explicit_month.is_none() || explicit_day.is_none() {
            return None;
        }
        Some(Date {
            year: explicit_year.unwrap(),
            month: explicit_month.unwrap(),
            day: explicit_day.unwrap(),
        })
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Year {
    val: u32,
}

impl Year {
    pub fn get(val: u32) -> Option<Self> {
        if val > 2024 {
            return None;
        }
        Some(Year { val })
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Month {
    val: u32,
}

impl Month {
    pub fn get(val: u32) -> Option<Self> {
        if val > 12 {
            return None;
        }
        Some(Month { val })
    }
}

// for this shitty exercise assume that each month has 31 days
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Day {
    val: u32,
}

impl Day {
    pub fn get(val: u32) -> Option<Self> {
        if val > 31 {
            return None;
        }
        Some(Day { val })
    }
}
