pub struct MoonAge<T>
    where T: Fn(Option<(f64, f64)>) -> Option<(f64, f64)> {
    calculation: T,
    options: Option<(f64, f64)>,
}

/**
Implementation of closure state. Should avoid expensive operation.
(https://doc.rust-lang.org/book/ch13-01-closures.html)
**/
impl<T> MoonAge<T>
    where T: Fn(Option<(f64, f64)>) -> Option<(f64, f64)> {
    pub fn new(calculation: T) -> MoonAge<T> {
        MoonAge {
            calculation,
            options: None,
        }
    }

    pub fn get_options(
        &mut self,
        moon_width: f64,
        year: i32,
        month: u32,
        day: u32,
    ) -> (f64, f64) {
        match self.options {
            Some((age, x)) => {
                (age, x)
            }
            None => {
                let (age, x) = get_eclipse_options(
                    moon_width,
                    year,
                    month,
                    day,
                );
                self.options = Some((age, x));
                (age, x)
            }
        }
    }
}

fn get_eclipse_options(
    moon_width: f64,
    year: i32,
    month: u32,
    day: u32,
) -> (f64, f64) {
    let moon_age = get_moon_age(day, month, year) as f64;
    let x = if moon_age <= 15.0 {
        360.0 - 360.0 * moon_age / 15.0
    } else {
        360.0 * moon_age / 30.0 - (360.0 - 360.0 * moon_age / 30.0)
    };
    return (moon_age, x);
}

fn get_moon_age(day: u32, month: u32, year: i32) -> i32 {
    let mut age = (get_moon_count(year) * 11 - 14) + (day + month) as i32;
    if age > 30 {
        while age > 30 {
            age -= 30;
        }
    }
    age
}

/**
Returns moon number which used for count moon age. This value placed in range 1-19 and after 19
starts from 1. Each year has own moon number. For example 2001 = 7.
*/
fn get_moon_count(year: i32) -> i32 {
    let mut moon_count: i32 = (year - 1994).abs();
    if moon_count == 0 {
        moon_count = 19;
    } else if moon_count > 19 {
        while moon_count > 19 {
            moon_count -= 19;
        }
    }
    moon_count
}
