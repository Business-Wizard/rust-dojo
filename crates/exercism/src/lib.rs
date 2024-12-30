const DAYS_PER_EARTH_YEAR: f64 = 365.25;
const SECONDS_PER_EARTH_DAY: f64 = 24.0 * 60.0 * 60.0;
const SECONDS_PER_EARTH_YEAR: f64 = DAYS_PER_EARTH_YEAR * SECONDS_PER_EARTH_DAY;

#[derive(Debug)]
pub struct Duration {
    time_in_seconds: u64,
}

impl From<u64> for Duration {
    fn from(time_in_seconds: u64) -> Self {
        Duration { time_in_seconds }
    }
}

pub trait Planet {
    fn years_during(duration: &Duration) -> f64 {
        todo!("convert a duration ({duration:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;

        duration_seconds / (0.2408467 * SECONDS_PER_EARTH_YEAR)
    }
}
impl Planet for Venus {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;

        duration_seconds / (0.61519726 * SECONDS_PER_EARTH_YEAR)
    }
}
impl Planet for Earth {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;

        duration_seconds / SECONDS_PER_EARTH_YEAR
    }
}
impl Planet for Mars {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;

        duration_seconds / (1.8808158 * SECONDS_PER_EARTH_YEAR)
    }
}
impl Planet for Jupiter {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;

        duration_seconds / (11.862615 * SECONDS_PER_EARTH_YEAR)
    }
}
impl Planet for Saturn {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;

        duration_seconds / (29.447498 * SECONDS_PER_EARTH_YEAR)
    }
}
impl Planet for Uranus {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;

        duration_seconds / (84.016846 * SECONDS_PER_EARTH_YEAR)
    }
}
impl Planet for Neptune {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;

        duration_seconds / (164.79132 * SECONDS_PER_EARTH_YEAR)
    }
}
