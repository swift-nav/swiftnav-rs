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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_latitudinal_hemisphere() {
        assert_eq!(format!("{}", LatitudinalHemisphere::North), "N");
        assert_eq!(format!("{}", LatitudinalHemisphere::South), "S");
    }

    #[test]
    fn display_longitudinal_hemisphere() {
        assert_eq!(format!("{}", LongitudinalHemisphere::East), "E");
        assert_eq!(format!("{}", LongitudinalHemisphere::West), "W");
    }

    #[test]
    fn display_hemisphere() {
        assert_eq!(
            format!("{}", Hemisphere::Latitudinal(LatitudinalHemisphere::North)),
            "N"
        );
        assert_eq!(
            format!("{}", Hemisphere::Longitudinal(LongitudinalHemisphere::West)),
            "W"
        );
    }
}
