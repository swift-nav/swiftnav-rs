/// Source of NMEA sentence like GPS, GLONASS or other.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Source {
    /// USA Global Positioning System
    GPS = 0b1,
    /// Russian Federation GLONASS
    GLONASS = 0b10,
    /// European Union Gallileo
    Gallileo = 0b100,
    /// China's Beidou
    Beidou = 0b1000,
    /// Global Navigation Sattelite System. Some combination of other systems. Depends on receiver
    /// model, receiver settings, etc..
    GNSS = 0b10000,
    /// `MediaTek` NMEA packet protocol
    MTK = 0b10_0000,
}
