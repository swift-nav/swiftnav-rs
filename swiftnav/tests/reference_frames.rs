use float_eq::assert_float_eq;
use swiftnav::{
    coords::{Coordinate, ECEF},
    reference_frame::{ReferenceFrame, TransformationRepository},
    time::{GpsTime, UtcTime},
};

fn make_epoch(year: u16) -> GpsTime {
    UtcTime::from_parts(year, 1, 1, 0, 0, 0.).to_gps_hardcoded()
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
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ITRF2014)
        .unwrap();
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
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ITRF2008)
        .unwrap();
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
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF2020)
        .unwrap();
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
        ReferenceFrame::ITRF2014,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF2014)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027894.1579, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4123, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7973, abs <= 0.0001);
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
        0.0198,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF2014);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf2005() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2005,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords.clone(), &ReferenceFrame::ETRF2005)
        .unwrap();
    assert_float_eq!(
        result_coords.position().x(),
        4027894.2107,
        abs <= 0.0001,
        "Initial: {:?} Result: {:?}",
        initial_coords,
        result_coords
    );
    assert_float_eq!(result_coords.position().y(), 307045.4661, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7626, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0235,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1835,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0200,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF2005);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf2000() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF2000,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF2000)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027894.2015, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4596, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7581, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0229,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1826,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0206,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF2000);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf97() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF97,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF97)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027894.1888, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4489, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7569, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0229,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1825,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0205,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF97);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf96() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF96,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF96)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027894.1888, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4489, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7569, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0229,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1825,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0205,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF96);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf94() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF94,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF94)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027894.1888, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4489, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7569, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0229,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1825,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0205,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF94);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf93() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF93,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF93)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027894.2406, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4251, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7267, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0296,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1793,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0152,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF93);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf92() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF92,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF92)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027894.1916, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4388, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7647, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0234,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1817,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0202,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF92);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf91() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF91,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF91)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027894.1746, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4238, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7647, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0234,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1817,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0202,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF91);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf90() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF90,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF90)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027894.1862, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4466, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7664, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0247,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1835,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0190,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF90);
}

/// Truth data obtained from https://www.epncb.oma.be/_productsservices/coord_trans/
#[test]
fn euref_etrf89() {
    let initial_coords = Coordinate::new(
        ReferenceFrame::ITRF89,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF89)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027894.1672, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.4186, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.7894, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0247,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1835,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0190,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF89);
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
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ITRF2014)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027893.8541, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307045.7877, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919475.0227, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        -0.0038,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.2171,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0402,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2000));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ITRF2014);
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
        ReferenceFrame::ITRF2014,
        ECEF::new(4027894.006, 307045.600, 4919474.910),
        Some(ECEF::new(0.01, 0.2, 0.030)),
        make_epoch(2000),
    );
    let transformations = TransformationRepository::from_builtin();

    // Test adjusting the epoch first then transforming
    let result_coords = transformations
        .transform(
            initial_coords.clone().adjust_epoch(&make_epoch(2008)),
            &ReferenceFrame::ETRF2014,
        )
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 4027894.3484, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307046.8758, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.9554, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0238,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1829,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0198,
        abs <= 0.1
    );
    assert_eq!(result_coords.epoch(), make_epoch(2008));
    assert_eq!(result_coords.reference_frame(), ReferenceFrame::ETRF2014);

    // Test transforming first then adjusting the epoch
    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::ETRF2014)
        .unwrap()
        .adjust_epoch(&make_epoch(2008));
    assert_float_eq!(result_coords.position().x(), 4027894.3484, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 307046.8758, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 4919474.9554, abs <= 0.0001);
    assert!(result_coords.velocity().is_some());
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().x(),
        0.0238,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().y(),
        0.1829,
        abs <= 0.1
    );
    assert_float_eq!(
        result_coords.velocity().as_ref().unwrap().z(),
        0.0198,
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

    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::NAD83_2011)
        .unwrap();
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

    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(
            initial_coords.clone().adjust_epoch(&make_epoch(2010)),
            &ReferenceFrame::NAD83_2011,
        )
        .unwrap();
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

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::NAD83_2011)
        .unwrap()
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

    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::NAD83_CSRS)
        .unwrap();
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

    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(
            initial_coords.clone().adjust_epoch(&make_epoch(2010)),
            &ReferenceFrame::NAD83_CSRS,
        )
        .unwrap();
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

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::NAD83_CSRS)
        .unwrap()
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

/// Truth data obtained from https://www.adv-online.de/AdV-Produkte/Integrierter-geodaetischer-Raumbezug/Transformationsparameter/
#[test]
fn dref91_r2016() {
    let initial_coords: Coordinate = Coordinate::new(
        ReferenceFrame::ITRF2020,
        ECEF::new(3842152.805, 563402.164, 5042888.600),
        None,
        UtcTime::from_parts(2023, 02, 22, 0, 0, 0.).to_gps_hardcoded(),
    );
    let transformations = TransformationRepository::from_builtin();

    let result_coords = transformations
        .transform(initial_coords, &ReferenceFrame::DREF91_R2016)
        .unwrap();
    assert_float_eq!(result_coords.position().x(), 3842153.3718, abs <= 0.0001);
    assert_float_eq!(result_coords.position().y(), 563401.6528, abs <= 0.0001);
    assert_float_eq!(result_coords.position().z(), 5042888.2271, abs <= 0.0001);
    assert!(result_coords.velocity().is_none());
    assert_float_eq!(
        result_coords.epoch().to_fractional_year_hardcoded(),
        2023.15,
        abs <= 0.01
    );
    assert_eq!(
        result_coords.reference_frame(),
        ReferenceFrame::DREF91_R2016
    );
}
