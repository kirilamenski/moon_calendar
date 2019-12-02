use chrono::{Date, Utc, Datelike, Local, NaiveDate};

pub struct MoonCalendarDate {
    pub current_date: Date<Local>,
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

impl MoonCalendarDate {
    pub fn new() -> MoonCalendarDate {
        let date = chrono::offset::Local::today();
        MoonCalendarDate {
            current_date: date,
            day: date.day(),
            month: date.month(),
            year: date.year(),
        }
    }

    pub fn set_day(&mut self, day: u32) {
        let days = get_days_in_month(self.year, self.month) as u32;
        if day > days {
            self.day = 1;
            self.set_month(self.month + 1);
        } else if day < 1 {
            self.set_month(self.month - 1);
            self.day = get_days_in_month(self.year, self.month) as u32;
        } else {
            self.day = day;
        }
    }

    pub fn set_month(&mut self, month: u32) {
        match month {
            13 => {
                self.month = 1;
                self.year += 1;
            }
            0 => {
                self.month = 12;
                self.year -= 1;
            }
            _ => self.month = month
        }
    }

    pub fn set_year(&mut self, year: i32) {
        self.year = year;
    }
}

fn get_days_in_month(year: i32, month: u32) -> i64 {
    if month == 12 {
        NaiveDate::from_ymd(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(year, month + 1, 1)
    }
        .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
        .num_days()
}