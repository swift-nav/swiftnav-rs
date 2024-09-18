use super::{ReferenceFrame, TimeDependentHelmertParams, Transformation};

pub const TRANSFORMATIONS: [Transformation; 31] = [
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF2014,
        params: TimeDependentHelmertParams {
            tx: -1.4,
            tx_dot: 0.0,
            ty: -0.9,
            ty_dot: -0.1,
            tz: 1.4,
            tz_dot: 0.2,
            s: -0.42,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF2008,
        params: TimeDependentHelmertParams {
            tx: 0.2,
            tx_dot: 0.0,
            ty: 1.0,
            ty_dot: -0.1,
            tz: 3.3,
            tz_dot: 0.1,
            s: -0.29,
            s_dot: 0.03,
            rx: 0.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF2005,
        params: TimeDependentHelmertParams {
            tx: 2.7,
            tx_dot: 0.3,
            ty: 0.1,
            ty_dot: -0.1,
            tz: -1.4,
            tz_dot: 0.1,
            s: 0.65,
            s_dot: 0.03,
            rx: 0.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF2000,
        params: TimeDependentHelmertParams {
            tx: -0.2,
            tx_dot: 0.1,
            ty: 0.8,
            ty_dot: 0.0,
            tz: -34.2,
            tz_dot: -1.7,
            s: 2.25,
            s_dot: 0.11,
            rx: 0.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF97,
        params: TimeDependentHelmertParams {
            tx: 6.5,
            tx_dot: 0.1,
            ty: -3.9,
            ty_dot: -0.6,
            tz: -77.9,
            tz_dot: -3.1,
            s: 3.98,
            s_dot: 0.12,
            rx: 0.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.36,
            rz_dot: 0.02,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF96,
        params: TimeDependentHelmertParams {
            tx: 6.5,
            tx_dot: 0.1,
            ty: -3.9,
            ty_dot: -0.6,
            tz: -77.9,
            tz_dot: -3.1,
            s: 3.98,
            s_dot: 0.12,
            rx: 0.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.36,
            rz_dot: 0.02,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF94,
        params: TimeDependentHelmertParams {
            tx: 6.5,
            tx_dot: 0.1,
            ty: -3.9,
            ty_dot: -0.6,
            tz: -77.9,
            tz_dot: -3.1,
            s: 3.98,
            s_dot: 0.12,
            rx: 0.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.36,
            rz_dot: 0.02,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF93,
        params: TimeDependentHelmertParams {
            tx: -65.8,
            tx_dot: -2.8,
            ty: 1.9,
            ty_dot: -0.2,
            tz: -71.3,
            tz_dot: -2.3,
            s: 4.47,
            s_dot: 0.12,
            rx: -3.36,
            rx_dot: -0.11,
            ry: -4.33,
            ry_dot: -0.19,
            rz: 0.75,
            rz_dot: 0.07,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF92,
        params: TimeDependentHelmertParams {
            tx: 14.5,
            tx_dot: 0.1,
            ty: -1.9,
            ty_dot: -0.6,
            tz: -85.9,
            tz_dot: -3.1,
            s: 3.27,
            s_dot: 0.12,
            rx: 0.00,
            rx_dot: 0.00,
            ry: 0.00,
            ry_dot: 0.00,
            rz: 0.36,
            rz_dot: 0.02,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF91,
        params: TimeDependentHelmertParams {
            tx: 26.5,
            tx_dot: 0.1,
            ty: 12.1,
            ty_dot: -0.6,
            tz: -91.9,
            tz_dot: -3.1,
            s: 4.67,
            s_dot: 0.12,
            rx: 0.00,
            rx_dot: 0.00,
            ry: 0.00,
            ry_dot: 0.00,
            rz: 0.36,
            rz_dot: 0.02,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF90,
        params: TimeDependentHelmertParams {
            tx: 24.5,
            tx_dot: 0.1,
            ty: 8.1,
            ty_dot: -0.6,
            tz: -107.9,
            tz_dot: -3.1,
            s: 4.97,
            s_dot: 0.12,
            rx: 0.00,
            rx_dot: 0.00,
            ry: 0.00,
            ry_dot: 0.00,
            rz: 0.36,
            rz_dot: 0.02,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF89,
        params: TimeDependentHelmertParams {
            tx: 29.5,
            tx_dot: 0.1,
            ty: 32.1,
            ty_dot: -0.6,
            tz: -145.9,
            tz_dot: -3.1,
            s: 8.37,
            s_dot: 0.12,
            rx: 0.00,
            rx_dot: 0.00,
            ry: 0.00,
            ry_dot: 0.00,
            rz: 0.36,
            rz_dot: 0.02,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF88,
        params: TimeDependentHelmertParams {
            tx: 24.5,
            tx_dot: 0.1,
            ty: -3.9,
            ty_dot: -0.6,
            tz: -169.9,
            tz_dot: -3.1,
            s: 11.47,
            s_dot: 0.12,
            rx: 0.10,
            rx_dot: 0.00,
            ry: 0.00,
            ry_dot: 0.00,
            rz: 0.36,
            rz_dot: 0.02,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ETRF2020,
        params: TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.086,
            ry: 0.0,
            ry_dot: 0.519,
            rz: 0.0,
            rz_dot: -0.753,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2014,
        to: ReferenceFrame::ETRF2014,
        params: TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.085,
            ry: 0.0,
            ry_dot: 0.531,
            rz: 0.0,
            rz_dot: -0.770,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2005,
        to: ReferenceFrame::ETRF2005,
        params: TimeDependentHelmertParams {
            tx: 56.0,
            tx_dot: 0.0,
            ty: 48.0,
            ty_dot: 0.0,
            tz: -37.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.054,
            ry: 0.0,
            ry_dot: 0.518,
            rz: 0.0,
            rz_dot: -0.781,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2000,
        to: ReferenceFrame::ETRF2000,
        params: TimeDependentHelmertParams {
            tx: 54.0,
            tx_dot: 0.0,
            ty: 51.0,
            ty_dot: 0.0,
            tz: -48.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.081,
            ry: 0.0,
            ry_dot: 0.490,
            rz: 0.0,
            rz_dot: -0.792,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF97,
        to: ReferenceFrame::ETRF97,
        params: TimeDependentHelmertParams {
            tx: 41.0,
            tx_dot: 0.0,
            ty: 41.0,
            ty_dot: 0.0,
            tz: -49.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.200,
            ry: 0.0,
            ry_dot: 0.500,
            rz: 0.0,
            rz_dot: -0.650,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF96,
        to: ReferenceFrame::ETRF96,
        params: TimeDependentHelmertParams {
            tx: 41.0,
            tx_dot: 0.0,
            ty: 41.0,
            ty_dot: 0.0,
            tz: -49.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.200,
            ry: 0.0,
            ry_dot: 0.500,
            rz: 0.0,
            rz_dot: -0.650,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF94,
        to: ReferenceFrame::ETRF94,
        params: TimeDependentHelmertParams {
            tx: 41.0,
            tx_dot: 0.0,
            ty: 41.0,
            ty_dot: 0.0,
            tz: -49.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.200,
            ry: 0.0,
            ry_dot: 0.500,
            rz: 0.0,
            rz_dot: -0.650,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF93,
        to: ReferenceFrame::ETRF93,
        params: TimeDependentHelmertParams {
            tx: 19.0,
            tx_dot: 0.0,
            ty: 53.0,
            ty_dot: 0.0,
            tz: -21.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.320,
            ry: 0.0,
            ry_dot: 0.780,
            rz: 0.0,
            rz_dot: -0.670,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF92,
        to: ReferenceFrame::ETRF92,
        params: TimeDependentHelmertParams {
            tx: 38.0,
            tx_dot: 0.0,
            ty: 40.0,
            ty_dot: 0.0,
            tz: -37.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.210,
            ry: 0.0,
            ry_dot: 0.520,
            rz: 0.0,
            rz_dot: -0.680,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF91,
        to: ReferenceFrame::ETRF91,
        params: TimeDependentHelmertParams {
            tx: 21.0,
            tx_dot: 0.0,
            ty: 25.0,
            ty_dot: 0.0,
            tz: -37.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.210,
            ry: 0.0,
            ry_dot: 0.520,
            rz: 0.0,
            rz_dot: -0.680,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF90,
        to: ReferenceFrame::ETRF90,
        params: TimeDependentHelmertParams {
            tx: 19.0,
            tx_dot: 0.0,
            ty: 28.0,
            ty_dot: 0.0,
            tz: -23.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.110,
            ry: 0.0,
            ry_dot: 0.570,
            rz: 0.0,
            rz_dot: -0.710,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF89,
        to: ReferenceFrame::ETRF89,
        params: TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.110,
            ry: 0.0,
            ry_dot: 0.570,
            rz: 0.0,
            rz_dot: -0.710,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2014,
        to: ReferenceFrame::NAD83_2011,
        params: TimeDependentHelmertParams {
            tx: 1005.30,
            tx_dot: 0.79,
            ty: -1909.21,
            ty_dot: -0.60,
            tz: -541.57,
            tz_dot: -1.44,
            s: 0.36891,
            s_dot: -0.07201,
            rx: -26.78138,
            rx_dot: -0.06667,
            ry: 0.42027,
            ry_dot: 0.75744,
            rz: -10.93206,
            rz_dot: 0.05133,
            epoch: 2010.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2014,
        to: ReferenceFrame::ETRF2014,
        params: TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.085,
            ry: 0.0,
            ry_dot: 0.531,
            rz: 0.0,
            rz_dot: -0.770,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2008,
        to: ReferenceFrame::NAD83_CSRS,
        params: TimeDependentHelmertParams {
            tx: 1003.70,
            tx_dot: 0.79,
            ty: -1911.11,
            ty_dot: -0.60,
            tz: -543.97,
            tz_dot: -1.34,
            s: 0.38891,
            s_dot: -0.10201,
            rx: -26.78138,
            rx_dot: -0.06667,
            ry: 0.42027,
            ry_dot: 0.75744,
            rz: -10.93206,
            rz_dot: 0.05133,
            epoch: 2010.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2014,
        to: ReferenceFrame::NAD83_CSRS,
        params: TimeDependentHelmertParams {
            tx: 1005.30,
            tx_dot: 0.79,
            ty: -1909.21,
            ty_dot: -0.60,
            tz: -541.57,
            tz_dot: -1.44,
            s: 0.36891,
            s_dot: -0.07201,
            rx: -26.78138,
            rx_dot: -0.06667,
            ry: 0.42027,
            ry_dot: 0.75744,
            rz: -10.93206,
            rz_dot: 0.05133,
            epoch: 2010.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::NAD83_CSRS,
        params: TimeDependentHelmertParams {
            tx: 1003.90,
            tx_dot: 0.79,
            ty: -1909.61,
            ty_dot: -0.70,
            tz: -541.17,
            tz_dot: -1.24,
            s: -0.05109,
            s_dot: -0.07201,
            rx: -26.78138,
            rx_dot: -0.06667,
            ry: 0.42027,
            ry_dot: 0.75744,
            rz: -10.93206,
            rz_dot: 0.05133,
            epoch: 2010.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::DREF91_R2016,
        params: TimeDependentHelmertParams {
            tx: -3.0821,
            tx_dot: -20.3181,
            ty: 95.0769,
            ty_dot: -20.3593,
            tz: -73.5435,
            tz_dot: 23.6394,
            s: 7.4874,
            s_dot: -0.3306,
            rx: 2.5445,
            rx_dot: -0.5966,
            ry: 17.6078,
            ry_dot: 1.4967,
            rz: -27.6123,
            rz_dot: -0.5284,
            epoch: 2021.0,
        },
    },
];