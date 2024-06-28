use float_eq::assert_float_eq;
use swiftnav::{
    coords::{Coordinate, ECEF},
    reference_frame::{get_transformation, ReferenceFrame},
    time::{GpsTime, UtcTime},
};

fn make_epoch(year: u16) -> GpsTime {
    UtcTime::from_date(year, 1, 1, 0, 0, 0.).to_gps_hardcoded()
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_itrf2014() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2020,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformation =
        get_transformation(ReferenceFrame::ITRF2020, ReferenceFrame::ITRF2014).unwrap();
    let result_coords = transformation.transform(&initial_coords);
    assert_float_eq!(result_coords.position().x(), 4027894.0029, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.6005, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.9063, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0100,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1999,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0302,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ITRF2014);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_itrf2008() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2020,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformation =
        get_transformation(ReferenceFrame::ITRF2020, ReferenceFrame::ITRF2008).unwrap();
    let result_coords = transformation.transform(&initial_coords);
    assert_float_eq!(result_coords.position().x(), 4027894.0032, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.6023, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.9082, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0101,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1999,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0302,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ITRF2008);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf2020() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2020,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformation =
        get_transformation(ReferenceFrame::ITRF2020, ReferenceFrame::ETRF2020).unwrap();
    let result_coords = transformation.transform(&initial_coords);
    assert_float_eq!(result_coords.position().x(), 4027894.1545, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4157, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7999, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0235,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1832,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0200,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF2020);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf2014() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2020,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformation =
        get_transformation(ReferenceFrame::ITRF2020, ReferenceFrame::ETRF2014).unwrap();
    let result_coords = transformation.transform(&initial_coords);
    assert_float_eq!(result_coords.position().x(), 4027894.1548, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4128, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7937, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0238,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1828,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0200,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF2014);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf2014_reverse() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ETRF2014,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformation =
        get_transformation(ReferenceFrame::ETRF2014, ReferenceFrame::ITRF2020).unwrap();
    let result_coords = transformation.transform(&initial_coords);
    assert_float_eq!(result_coords.position().x(), 4027893.8572, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.7872, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919475.0263, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        -0.0038,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.2172,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0400,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ITRF2020);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_adjust_epoch() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2020,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let result_coords = initial_coords.adjust_epoch(&make_epoch(2008));
    assert_float_eq!(result_coords.position().x(), 4027894.0860, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307047.2000, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919475.1500, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.01,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.2,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.03,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2008));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ITRF2020);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_complete_transform() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2020,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformation =
        get_transformation(ReferenceFrame::ITRF2020, ReferenceFrame::ETRF2014).unwrap();

    // Test adjusting the epoch first then transforming
    let result_coords = transformation.transform(&initial_coords.adjust_epoch(&make_epoch(2008)));
    assert_float_eq!(result_coords.position().x(), 4027894.3453, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307046.8755, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.9533, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0238,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1828,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0200,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2008));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF2014);

    // Test transforming first then adjusting the epoch
    let result_coords = transformation
        .transform(&initial_coords)
        .adjust_epoch(&make_epoch(2008));
    assert_float_eq!(result_coords.position().x(), 4027894.3453, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307046.8755, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.9533, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0238,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1828,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0200,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2008));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF2014);
}

/// Truth data obtained from https://geodesy.noaa.gov/TOOLS/Htdp/Htdp.shtml
#[test]
fn htdp_nad83_2011_fixed_date() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2014,
        ECEF::new(-2705105.358, -4262045.769, 3885381.686),
        Some(ECEF::new(-0.02225, 0.02586, 0.01258)),
        make_epoch(2010),
    );

    let transformation =
        get_transformation(ReferenceFrame::ITRF2014, ReferenceFrame::NAD83_2011).unwrap();

    let result_coords = transformation.transform(&initial_coords);
    assert_float_eq!(result_coords.position().x(), -2705104.572, abs <= 0.001);
    assert_float_eq!(result_coords.position().y(), -4262047.032, abs <= 0.001);
    assert_float_eq!(result_coords.position().z(), 3885381.705, abs <= 0.001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        -0.00594,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.02615,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.02217,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2010));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::NAD83_2011);
}

/// Truth data obtained from https://geodesy.noaa.gov/TOOLS/Htdp/Htdp.shtml
#[test]
fn htdp_nad83_2011_adjust_epoch() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2014,
        ECEF::new(-2705105.358, -4262045.769, 3885381.686),
        Some(ECEF::new(-0.02225, 0.02586, 0.01258)),
        make_epoch(2020),
    );

    let transformation =
        get_transformation(ReferenceFrame::ITRF2014, ReferenceFrame::NAD83_2011).unwrap();

    let result_coords = transformation.transform(&initial_coords.adjust_epoch(&make_epoch(2010)));
    assert_float_eq!(result_coords.position().x(), -2705104.349, abs <= 0.001);
    assert_float_eq!(result_coords.position().y(), -4262047.291, abs <= 0.001);
    assert_float_eq!(result_coords.position().z(), 3885381.579, abs <= 0.001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        -0.00594,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.02615,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.02217,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2010));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::NAD83_2011);

    let result_coords = transformation
        .transform(&initial_coords)
        .adjust_epoch(&make_epoch(2010));
    assert_float_eq!(result_coords.position().x(), -2705104.349, abs <= 0.001);
    assert_float_eq!(result_coords.position().y(), -4262047.291, abs <= 0.001);
    assert_float_eq!(result_coords.position().z(), 3885381.579, abs <= 0.001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        -0.00594,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.02615,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.02217,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2010));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::NAD83_2011);
}

/// Truth data obtained from https://webapp.csrs-scrs.nrcan-rncan.gc.ca/geod/tools-outils/trx.php
#[test]
fn trx_nad83_csrs_fixed_date() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2020,
        ECEF::new(1267458.677, -4294620.216, 4526843.210),
        Some(ECEF::new(-0.01578, -0.00380, 0.00466)),
        make_epoch(2010),
    );

    let transformation =
        get_transformation(ReferenceFrame::ITRF2020, ReferenceFrame::NAD83_CSRS).unwrap();

    let result_coords = transformation.transform(&initial_coords);
    assert_float_eq!(result_coords.position().x(), 1267459.462, abs <= 0.001);
    assert_float_eq!(result_coords.position().y(), -4294621.605, abs <= 0.001);
    assert_float_eq!(result_coords.position().z(), 4526843.224, abs <= 0.001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.00261,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        -0.00241,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        -0.00017,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2010));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::NAD83_CSRS);
}

/// Truth data obtained from https://geodesy.noaa.gov/TOOLS/Htdp/Htdp.shtml
#[test]
fn trx_nad83_csrs_adjust_epoch() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2020,
        ECEF::new(1267458.677, -4294620.216, 4526843.210),
        Some(ECEF::new(-0.01578, -0.00380, 0.00466)),
        make_epoch(2020),
    );

    let transformation =
        get_transformation(ReferenceFrame::ITRF2020, ReferenceFrame::NAD83_CSRS).unwrap();

    let result_coords = transformation.transform(&initial_coords.adjust_epoch(&make_epoch(2010)));
    assert_float_eq!(result_coords.position().x(), 1267459.620, abs <= 0.001);
    assert_float_eq!(result_coords.position().y(), -4294621.567, abs <= 0.001);
    assert_float_eq!(result_coords.position().z(), 4526843.177, abs <= 0.001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.00261,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        -0.00241,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        -0.00017,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2010));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::NAD83_CSRS);

    let result_coords = transformation
        .transform(&initial_coords)
        .adjust_epoch(&make_epoch(2010));
    assert_float_eq!(result_coords.position().x(), 1267459.620, abs <= 0.001);
    assert_float_eq!(result_coords.position().y(), -4294621.567, abs <= 0.001);
    assert_float_eq!(result_coords.position().z(), 4526843.177, abs <= 0.001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.00261,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        -0.00241,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        -0.00017,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2010));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::NAD83_CSRS);
}
