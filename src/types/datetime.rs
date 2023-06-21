use std::rc::Rc;

use chrono::{
    DateTime,
    Datelike,
    NaiveDate,
    TimeZone,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub(crate) enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::January => write!(f, "January"),
            Self::February => write!(f, "February"),
            Self::March => write!(f, "March"),
            Self::April => write!(f, "April"),
            Self::May => write!(f, "May"),
            Self::June => write!(f, "June"),
            Self::July => write!(f, "July"),
            Self::August => write!(f, "August"),
            Self::September => write!(f, "September"),
            Self::October => write!(f, "October"),
            Self::November => write!(f, "November"),
            Self::December => write!(f, "December"),
        }
    }
}

impl Month {
    pub(crate) fn from_u32(month: u32) -> Self {
        match month % 12 {
            1 => Self::January,
            2 => Self::February,
            3 => Self::March,
            4 => Self::April,
            5 => Self::May,
            6 => Self::June,
            7 => Self::July,
            8 => Self::August,
            9 => Self::September,
            10 => Self::October,
            11 => Self::November,
            _ => Self::December,
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct DateTimeRange<T: TimeZone> {
    pub(crate) start: Rc<DateTime<T>>,
    pub(crate) end: Rc<DateTime<T>>,
}

impl<T: TimeZone> DateTimeRange<T> {
    pub(crate) fn from(start: DateTime<T>, end: DateTime<T>) -> Self {
        Self {
            start: Rc::new(start),
            end: Rc::new(end),
        }
    }

    pub(crate) fn list_years(&self) -> Vec<i32> {
        if self.start > self.end {
            return Vec::new();
        }

        (self.start.year()..=self.end.year()).collect()
    }

    pub(crate) fn list_months_for_year(&self, year: i32) -> Vec<Month> {
        if self.list_years().contains(&year) != true {
            return Vec::new();
        }

        match (year == self.start.year(), year == self.end.year()) {
            (true, true) => (self.start.month()..=self.end.month())
                .map(Month::from_u32)
                .collect(),
            (true, false) => {
                (self.start.month()..=12).map(Month::from_u32).collect()
            },
            (false, true) => {
                (1..=self.end.month()).map(Month::from_u32).collect()
            },
            (false, false) => (1..=12).map(Month::from_u32).collect(),
        }
    }

    pub(crate) fn list_days_for_year_and_month(
        &self,
        year: i32,
        month: Month,
    ) -> Vec<u32> {
        if self.list_months_for_year(year).contains(&month) != true {
            return Vec::new();
        }

        let end_of_month = || -> u32 {
            let next_month = Month::from_u32(month as u32 + 1);
            let first_of_next_month =
                NaiveDate::from_ymd_opt(year, next_month as u32, 1).unwrap();
            let last_of_this_month =
                first_of_next_month - chrono::Duration::days(1);
            last_of_this_month.day()
        };

        let is_start_year = self.start.year() == year;
        let is_end_year = self.end.year() == year;
        let is_start_month = self.start.month() == month as u32;
        let is_end_month = self.end.month() == month as u32;
        match (is_start_year && is_start_month, is_end_year && is_end_month) {
            (true, true) => (self.start.day()..=self.end.day()).collect(),
            (true, false) => (self.start.day()..=end_of_month()).collect(),
            (false, true) => (1..=self.end.day()).collect(),
            (false, false) => (1..=end_of_month()).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{
        DateTime,
        TimeZone,
        Utc,
    };
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::{
        DateTimeRange,
        Month,
    };

    wasm_bindgen_test_configure!(run_in_browser);

    fn make_date(y: i32, m: u32, d: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, 12, 0, 0).unwrap()
    }

    #[wasm_bindgen_test]
    fn month_can_be_contructed_from_u32() {
        let tests = vec![
            (1, Month::January),
            (2, Month::February),
            (3, Month::March),
            (4, Month::April),
            (5, Month::May),
            (6, Month::June),
            (7, Month::July),
            (8, Month::August),
            (9, Month::September),
            (10, Month::October),
            (11, Month::November),
            (12, Month::December),
            (0, Month::December),
            (13, Month::January),
            (14, Month::February),
            (678, Month::June),
        ];

        for (month, expected_month) in tests {
            let month = Month::from_u32(month);

            assert_eq!(month, expected_month);
        }
    }

    #[wasm_bindgen_test]
    fn datetimerange_is_constructed_from_two_datetimes() {
        let date1 = Utc::now();
        let date2 = Utc::now();
        let _ = DateTimeRange::from(date1, date2);
    }

    #[wasm_bindgen_test]
    fn datetimerange_list_years_lists_years_between_its_dates() {
        let tests = vec![
            // incorrect date order
            (make_date(2000, 1, 1), make_date(1999, 1, 1), 1..=0),
            // single year range is correct
            (make_date(1999, 1, 1), make_date(1999, 1, 1), 1999..=1999),
            // multi year range is correct
            (make_date(1999, 1, 1), make_date(2000, 1, 1), 1999..=2000),
            (make_date(1999, 1, 1), make_date(3000, 12, 1), 1999..=3000),
            (make_date(1999, 12, 31), make_date(3003, 8, 5), 1999..=3003),
        ];

        for (date1, date2, expected_years) in tests {
            let range = DateTimeRange::from(date1, date2);
            let years = range.list_years();
            let expected_years: Vec<i32> = expected_years.collect();

            assert_eq!(years, expected_years);
        }
    }

    #[wasm_bindgen_test]
    fn datetimerange_list_months_for_year_lists_available_months_for_given_year(
    ) {
        let tests = vec![
            // incorrect date order
            (make_date(2000, 1, 1), make_date(1999, 1, 1), 2000, 1..=0),
            // single month range is correct
            (make_date(1999, 1, 1), make_date(1999, 1, 1), 1999, 1..=1),
            (make_date(1999, 1, 1), make_date(2000, 1, 1), 2000, 1..=1),
            // year out of range
            (make_date(1999, 1, 1), make_date(2000, 1, 1), 3000, 1..=0),
            // multi month range is correct
            // partial year
            (make_date(1999, 4, 1), make_date(1999, 8, 1), 1999, 4..=8),
            // partial year to end
            (make_date(1999, 4, 1), make_date(2001, 8, 1), 1999, 4..=12),
            // full year
            (make_date(1999, 4, 1), make_date(2001, 8, 1), 2000, 1..=12),
            // partial year from start
            (make_date(1999, 4, 1), make_date(2001, 8, 1), 2001, 1..=8),
        ];

        for (date1, date2, year, expected_months) in tests {
            let range = DateTimeRange::from(date1, date2);
            let months = range.list_months_for_year(year);
            let expected_months: Vec<Month> =
                expected_months.map(Month::from_u32).collect();

            assert_eq!(months, expected_months);
        }
    }

    #[wasm_bindgen_test]
    fn datetimerange_list_days_for_year_and_month_lists_available_days_for_given_combination(
    ) {
        let tests = vec![
            // incorrect date order
            (make_date(2000, 1, 1), make_date(1999, 1, 1), 1999, 1, 1..=0),
            // single day range is correct
            (make_date(1999, 1, 4), make_date(1999, 1, 5), 1999, 1, 4..=5),
            // multi day range is correct
            (make_date(1999, 1, 2), make_date(1999, 1, 20), 1999, 1, 2..=20),
            // year out of range
            (make_date(1999, 1, 1), make_date(1999, 1, 1), 2000, 1, 1..=0),
            // month out of range
            (make_date(1999, 1, 1), make_date(1999, 1, 1), 1999, 2, 1..=0),
            // february with 28 days is correct
            (make_date(1999, 1, 1), make_date(2000, 12, 1), 1999, 2, 1..=28),
            // february with 29 days is correct
            (make_date(1999, 1, 1), make_date(2000, 12, 1), 2000, 2, 1..=29),
            // month with 30 days is correct
            (make_date(1999, 1, 1), make_date(2000, 12, 1), 2000, 4, 1..=30),
            // month with 31 days is correct
            (make_date(1999, 1, 1), make_date(2000, 12, 1), 2000, 5, 1..=31),
        ];

        for (date1, date2, year, month, expected_days) in tests {
            let range = DateTimeRange::from(date1, date2);
            let days = range
                .list_days_for_year_and_month(year, Month::from_u32(month));
            let expected_days: Vec<u32> = expected_days.collect();

            assert_eq!(days, expected_days);
        }
    }
}
