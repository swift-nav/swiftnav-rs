#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum Source {
    /// USA Global Positioning System
    #[default]
    GPS,
    /// Russian Federation GLONASS
    GLONASS,
    // Navigation Indian Constellation
    NavIC,
    /// European Union Gallileo
    Gallileo,
    /// China's Beidou
    BDS,
    /// Global Navigation Sattelite System. Some combination of other systems. Depends on receiver
    /// model, receiver settings, etc..
    GNSS,
    /// Quasi-Zenith Satellite System (Japan)
    QZSS,
}

impl Source {
    #[must_use]
    pub fn to_nmea_talker_id(&self) -> &str {
        match self {
            Source::GPS => "GP",
            Source::GLONASS => "GL",
            Source::NavIC => "GI",
            Source::Gallileo => "GA",
            Source::BDS => "GB",
            Source::GNSS => "GN",
            Source::QZSS => "GQ",
        }
    }
}
