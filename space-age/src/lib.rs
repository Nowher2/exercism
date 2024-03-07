#[derive(Debug)]
pub struct Duration{
    seconds:u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
      Duration{
          seconds: s,
      }
    }
}
const EARTH_YEAR_SECONDS:u64 = 31557600;
pub trait Planet {
    fn years_during(d: &Duration) -> f64{
        let earth_year = d.seconds as f64 / EARTH_YEAR_SECONDS as f64;
        earth_year/Self::orbital_period()
    }
    fn orbital_period()->f64;
}

macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n; impl Planet for $n { fn orbital_period() -> f64 { $p } }
    }
}
planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

