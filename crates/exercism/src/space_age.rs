use std::collections::HashMap;

const DAYS_PER_EARTH_YEAR: f64 = 365.25;
const SECONDS_PER_EARTH_DAY: f64 = 24.0 * 60.0 * 60.0;
const SECONDS_PER_EARTH_YEAR: f64 = DAYS_PER_EARTH_YEAR * SECONDS_PER_EARTH_DAY;

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

#[derive(Debug, Eq, PartialEq, Hash)]
enum PlanetEnum {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

fn create_planet_to_earth_orbital_period_ratio_mapping() -> HashMap<PlanetEnum, f64> {
    let mut planet_to_earth_orbital_period_ratio_mapping = HashMap::new();
    planet_to_earth_orbital_period_ratio_mapping.insert(PlanetEnum::Mercury, 0.2408467);
    planet_to_earth_orbital_period_ratio_mapping.insert(PlanetEnum::Venus, 0.61519726);
    planet_to_earth_orbital_period_ratio_mapping.insert(PlanetEnum::Earth, 1.0);
    planet_to_earth_orbital_period_ratio_mapping.insert(PlanetEnum::Mars, 1.8808158);
    planet_to_earth_orbital_period_ratio_mapping.insert(PlanetEnum::Jupiter, 11.862615);
    planet_to_earth_orbital_period_ratio_mapping.insert(PlanetEnum::Saturn, 29.447498);
    planet_to_earth_orbital_period_ratio_mapping.insert(PlanetEnum::Uranus, 84.016846);
    planet_to_earth_orbital_period_ratio_mapping.insert(PlanetEnum::Neptune, 164.79132);
    planet_to_earth_orbital_period_ratio_mapping
}

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

    fn orbital_ratio_to_earth() -> f64 {
        todo!("return the ratio of the orbital period of this planet to that of Earth");
    }
}

impl Planet for Mercury {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;
        let seconds_per_mercury_year = SECONDS_PER_EARTH_YEAR * Mercury::orbital_ratio_to_earth();
        duration_seconds / seconds_per_mercury_year
    }

    fn orbital_ratio_to_earth() -> f64 {
        let planet_to_earth_mapping = create_planet_to_earth_orbital_period_ratio_mapping();
        *planet_to_earth_mapping.get(&PlanetEnum::Mercury).unwrap()
    }
}
impl Planet for Venus {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;
        let seconds_per_venus_year = SECONDS_PER_EARTH_YEAR * Venus::orbital_ratio_to_earth();
        duration_seconds / seconds_per_venus_year
    }

    fn orbital_ratio_to_earth() -> f64 {
        let planet_to_earth_mapping = create_planet_to_earth_orbital_period_ratio_mapping();
        *planet_to_earth_mapping.get(&PlanetEnum::Venus).unwrap()
    }
}
impl Planet for Earth {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;
        let seconds_per_earth_year = SECONDS_PER_EARTH_YEAR * Earth::orbital_ratio_to_earth();
        duration_seconds / seconds_per_earth_year
    }

    fn orbital_ratio_to_earth() -> f64 {
        let planet_to_earth_mapping = create_planet_to_earth_orbital_period_ratio_mapping();
        *planet_to_earth_mapping.get(&PlanetEnum::Earth).unwrap()
    }
}
impl Planet for Mars {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;
        let seconds_per_mars_year = SECONDS_PER_EARTH_YEAR * Mars::orbital_ratio_to_earth();
        duration_seconds / seconds_per_mars_year
    }

    fn orbital_ratio_to_earth() -> f64 {
        let planet_to_earth_mapping = create_planet_to_earth_orbital_period_ratio_mapping();
        *planet_to_earth_mapping.get(&PlanetEnum::Mars).unwrap()
    }
}
impl Planet for Jupiter {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;
        let seconds_per_jupiter_year = SECONDS_PER_EARTH_YEAR * Jupiter::orbital_ratio_to_earth();
        duration_seconds / seconds_per_jupiter_year
    }

    fn orbital_ratio_to_earth() -> f64 {
        let planet_to_earth_mapping = create_planet_to_earth_orbital_period_ratio_mapping();
        *planet_to_earth_mapping.get(&PlanetEnum::Jupiter).unwrap()
    }
}
impl Planet for Saturn {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;
        let seconds_per_saturn_year = SECONDS_PER_EARTH_YEAR * Saturn::orbital_ratio_to_earth();
        duration_seconds / seconds_per_saturn_year
    }

    fn orbital_ratio_to_earth() -> f64 {
        let planet_to_earth_mapping = create_planet_to_earth_orbital_period_ratio_mapping();
        *planet_to_earth_mapping.get(&PlanetEnum::Saturn).unwrap()
    }
}
impl Planet for Uranus {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;
        let seconds_per_uranus_year = SECONDS_PER_EARTH_YEAR * Uranus::orbital_ratio_to_earth();
        duration_seconds / seconds_per_uranus_year
    }

    fn orbital_ratio_to_earth() -> f64 {
        let planet_to_earth_mapping = create_planet_to_earth_orbital_period_ratio_mapping();
        *planet_to_earth_mapping.get(&PlanetEnum::Uranus).unwrap()
    }
}
impl Planet for Neptune {
    fn years_during(duration: &Duration) -> f64 {
        let duration_seconds = duration.time_in_seconds as f64;
        let seconds_per_neptune_year = SECONDS_PER_EARTH_YEAR * Neptune::orbital_ratio_to_earth();
        duration_seconds / seconds_per_neptune_year
    }

    fn orbital_ratio_to_earth() -> f64 {
        let planet_to_earth_mapping = create_planet_to_earth_orbital_period_ratio_mapping();
        *planet_to_earth_mapping.get(&PlanetEnum::Neptune).unwrap()
    }
}
