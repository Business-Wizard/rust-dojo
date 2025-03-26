#![allow(dead_code)]
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_raw = hours;
        let minutes_raw = minutes;

        let hours_from_minutes_overrun = minutes_raw / 60;
        let minutes_without_overwrap = minutes_raw % 60;
        let hours_with_overrun = hours_raw + hours_from_minutes_overrun;

        let hours_without_overwrap = hours_with_overrun % 24;

        let hours: i32;
        if hours_without_overwrap < 0 {
            hours = 24 + hours_without_overwrap;
        } else {
            hours = hours_without_overwrap;
        }

        let minutes: i32;
        if minutes_without_overwrap < 0 {
            minutes = 60 + minutes_without_overwrap;
        } else {
            minutes = minutes_without_overwrap;
        }

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours = self.hours + minutes / 60;
        let minutes = self.minutes + minutes % 60;
        Clock::new(hours, minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl From<Clock> for String {
    fn from(clock: Clock) -> Self {
        format!(
            "{hours:02}:{minutes:02}",
            hours = clock.hours,
            minutes = clock.minutes
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Clock {
                hours: 12,
                minutes: 34
            },
            Clock::new(12, 34)
        );
    }

    #[test]
    fn test_new_hours_24() {
        assert_eq!(
            Clock {
                hours: 0,
                minutes: 0
            },
            Clock::new(24, 0)
        );
    }

    #[test]
    fn test_new_negative_hours() {
        let actual = Clock::new(-1, 34);
        let expected = Clock {
            hours: 23,
            minutes: 34,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_new_negative_hours_multiple_days() {
        assert_eq!(
            Clock {
                hours: 22,
                minutes: 34
            },
            Clock::new(-50, 34)
        );
    }

    #[test]
    #[ignore]
    fn test_new_negative_minutes_1() {
        assert_eq!(
            Clock {
                hours: 23,
                minutes: 59
            },
            Clock::new(0, -1)
        );
    }

    #[test]
    #[ignore]
    fn test_negative_hours_and_minutes() {
        assert_eq!(
            Clock {
                hours: 22,
                minutes: 59
            },
            Clock::new(-1, -1)
        );
    }

    #[test]
    #[ignore]
    fn test_negative_hours_and_minutes_wraps() {
        assert_eq!(
            Clock {
                hours: 21,
                minutes: 59
            },
            Clock::new(-25, -61)
        );
    }

    #[test]
    fn test_add_minutes_0() {
        let input = Clock {
            hours: 12,
            minutes: 34,
        };
        let actual = input.add_minutes(0);
        let expected = Clock {
            hours: 12,
            minutes: 34,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_add_minutes_1() {
        assert_eq!(
            Clock {
                hours: 12,
                minutes: 34
            }
            .add_minutes(1),
            Clock {
                hours: 12,
                minutes: 35
            }
        );
    }

    #[test]
    fn test_add_minutes_60() {
        assert_eq!(
            Clock {
                hours: 12,
                minutes: 34
            }
            .add_minutes(60),
            Clock {
                hours: 13,
                minutes: 34
            }
        );
    }

    #[test]
    fn test_add_minutes_61() {
        assert_eq!(
            Clock {
                hours: 12,
                minutes: 34
            }
            .add_minutes(61),
            Clock {
                hours: 13,
                minutes: 35
            }
        );
    }

    #[test]
    fn test_add_minutes_1440() {
        assert_eq!(
            Clock {
                hours: 12,
                minutes: 34
            }
            .add_minutes(1440),
            Clock {
                hours: 12,
                minutes: 34
            }
        );
    }

    #[test]
    fn test_add_minutes_negative_30() {
        assert_eq!(
            Clock {
                hours: 12,
                minutes: 34
            }
            .add_minutes(-30),
            Clock {
                hours: 12,
                minutes: 4
            }
        );
    }

    #[test]
    fn test_add_minutes_negative_wrap_hour() {
        assert_eq!(
            Clock {
                hours: 12,
                minutes: 34
            }
            .add_minutes(-60),
            Clock {
                hours: 11,
                minutes: 34
            }
        );
    }

    #[test]
    fn test_add_minutes_negative_wrap_day() {
        assert_eq!(
            Clock {
                hours: 12,
                minutes: 34
            }
            .add_minutes(-1440),
            Clock {
                hours: 12,
                minutes: 34
            }
        );
    }

    #[test]
    fn test_to_string_0() {
        assert_eq!(Clock::new(0, 0).to_string(), "00:00");
    }

    #[test]
    fn test_to_string_13_37() {
        assert_eq!(Clock::new(13, 37).to_string(), "13:37");
    }

    #[test]
    fn test_string_from_clock_trait() {
        let input = Clock::new(13, 37);
        let actual = String::from(input);
        let expected = "13:37";
        assert_eq!(actual, expected);
    }
}
