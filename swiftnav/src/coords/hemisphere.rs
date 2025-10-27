use core::fmt;

pub enum LatitudinalHemisphere {
    North,
    South,
}

impl fmt::Display for LatitudinalHemisphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LatitudinalHemisphere::North => write!(f, "N"),
            LatitudinalHemisphere::South => write!(f, "S"),
        }
    }
}

pub enum LongitudinalHemisphere {
    East,
    West,
}

impl fmt::Display for LongitudinalHemisphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LongitudinalHemisphere::East => write!(f, "E"),
            LongitudinalHemisphere::West => write!(f, "W"),
        }
    }
}

pub enum Hemisphere {
    Latitudinal(LatitudinalHemisphere),
    Longitudinal(LongitudinalHemisphere),
}

impl fmt::Display for Hemisphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hemisphere::Latitudinal(hemisphere) => write!(f, "{hemisphere}"),
            Hemisphere::Longitudinal(hemisphere) => write!(f, "{hemisphere}"),
        }
    }
}
