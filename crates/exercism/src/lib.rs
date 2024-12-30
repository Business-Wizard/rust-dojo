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

impl Planet for Mercury {}
impl Planet for Venus {}
impl Planet for Earth {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;

        duration_seconds / SECONDS_PER_EARTH_YEAR
    }
}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}
